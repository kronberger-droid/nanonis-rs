use super::protocol::{Protocol, HEADER_SIZE};
use crate::error::NanonisError;
use crate::types::NanonisValue;
use log::{debug, warn};
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub mod atom_track;
pub mod auto_approach;
pub mod beam_defl;
pub mod bias;
pub mod bias_spectr;
pub mod bias_sweep;
pub mod cpd_comp;
pub mod current;
pub mod data_log;
pub mod dig_lines;
pub mod folme;
pub mod gen_pi_ctrl;
pub mod gen_swp;
pub mod hs_swp;
pub mod interf;
pub mod kelvin_ctrl;
pub mod laser;
pub mod lockin;
pub mod lockin_freq_swp;
pub mod marks;
pub mod motor;
pub mod mpass;
pub mod oc_sync;
pub mod oscilloscope;
pub mod pattern;
pub mod pi_ctrl;
pub mod piezo;
pub mod pll;
pub mod pll_freq_swp;
pub mod pll_signal_anlzr;
pub mod safe_tip;
pub mod scan;
pub mod script;
pub mod signal_chart;
pub mod signals;
pub mod spectrum_anlzr;
pub mod tcplog;
pub mod tip_recovery;
pub mod user_in;
pub mod user_out;
pub mod util;
pub mod z_ctrl;
pub mod z_spectr;


/// Connection configuration for the Nanonis TCP client.
///
/// Contains timeout settings for different phases of the TCP connection lifecycle.
/// All timeouts have sensible defaults but can be customized for specific network conditions.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use nanonis_rs::ConnectionConfig;
///
/// // Use default timeouts
/// let config = ConnectionConfig::default();
///
/// // Customize timeouts for slow network
/// let config = ConnectionConfig {
///     connect_timeout: Duration::from_secs(30),
///     read_timeout: Duration::from_secs(60),
///     write_timeout: Duration::from_secs(10),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Timeout for establishing the initial TCP connection
    pub connect_timeout: Duration,
    /// Timeout for reading data from the Nanonis server
    pub read_timeout: Duration,
    /// Timeout for writing data to the Nanonis server
    pub write_timeout: Duration,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(10),
            write_timeout: Duration::from_secs(5),
        }
    }
}

/// Builder for constructing [`NanonisClient`] instances with flexible configuration.
///
/// The builder pattern allows you to configure various aspects of the client
/// before establishing the connection. This is more ergonomic than having
/// multiple constructor variants.
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// use nanonis_rs::NanonisClient;
///
/// let client = NanonisClient::builder()
///     .address("127.0.0.1")
///     .port(6501)
///     .debug(true)
///     .build()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// With custom timeouts:
/// ```no_run
/// use std::time::Duration;
/// use nanonis_rs::NanonisClient;
///
/// let client = NanonisClient::builder()
///     .address("192.168.1.100")
///     .port(6501)
///     .connect_timeout(Duration::from_secs(30))
///     .read_timeout(Duration::from_secs(60))
///     .debug(false)
///     .build()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Default)]
pub struct NanonisClientBuilder {
    address: Option<String>,
    port: Option<u16>,
    config: ConnectionConfig,
    debug: bool,
    safe_tip_on_drop: bool,
}

impl NanonisClientBuilder {
    pub fn address(mut self, addr: &str) -> Self {
        self.address = Some(addr.to_string());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Enable or disable debug logging
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Set the full connection configuration
    pub fn config(mut self, config: ConnectionConfig) -> Self {
        self.config = config;
        self
    }

    /// Set connect timeout
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.config.connect_timeout = timeout;
        self
    }

    /// Set read timeout
    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.config.read_timeout = timeout;
        self
    }

    /// Set write timeout
    pub fn write_timeout(mut self, timeout: Duration) -> Self {
        self.config.write_timeout = timeout;
        self
    }

    /// Enable automatic tip safety on client drop.
    ///
    /// When enabled, the client will automatically withdraw the Z-controller
    /// and move motors to a safe position when dropped. This is a safety feature
    /// to protect the tip if the program exits unexpectedly.
    ///
    /// **Warning**: This will move hardware on every client drop, including normal
    /// program termination. Only enable if you want this behavior.
    ///
    /// Default: `false` (disabled)
    pub fn safe_tip_on_drop(mut self, enabled: bool) -> Self {
        self.safe_tip_on_drop = enabled;
        self
    }

    /// Build the NanonisClient
    pub fn build(self) -> Result<NanonisClient, NanonisError> {
        let address = self
            .address
            .ok_or_else(|| NanonisError::Protocol("Address must be specified".to_string()))?;

        let port = self
            .port
            .ok_or_else(|| NanonisError::Protocol("Port must be specified".to_string()))?;

        let socket_addr: SocketAddr = format!("{address}:{port}")
            .parse()
            .map_err(|_| NanonisError::Protocol(format!("Invalid address: {address}")))?;

        debug!("Connecting to Nanonis at {address}");

        let stream = TcpStream::connect_timeout(&socket_addr, self.config.connect_timeout)
            .map_err(|e| {
                warn!("Failed to connect to {address}: {e}");
                NanonisError::from_io(e, format!("Failed to connect to {address}"))
            })?;

        // Set socket timeouts
        stream.set_read_timeout(Some(self.config.read_timeout))?;
        stream.set_write_timeout(Some(self.config.write_timeout))?;

        debug!("Successfully connected to Nanonis");

        Ok(NanonisClient {
            stream,
            address,
            port,
            debug: self.debug,
            config: self.config,
            safe_tip_on_drop: self.safe_tip_on_drop,
            poisoned: false,
        })
    }
}

/// High-level client for communicating with Nanonis SPM systems via TCP.
///
/// `NanonisClient` provides a type-safe, Rust-friendly interface to the Nanonis
/// TCP protocol. It handles connection management, protocol serialization/deserialization,
/// and provides convenient methods for common operations like reading signals,
/// controlling bias voltage, and managing the scanning probe.
///
/// # Connection Management
///
/// The client maintains a persistent TCP connection to the Nanonis server.
/// If an I/O error occurs during a command, the client is **poisoned** to
/// prevent desynchronized reads on a corrupted stream. Call
/// [`reconnect()`](Self::reconnect) to re-establish the connection, or check
/// [`is_poisoned()`](Self::is_poisoned) to inspect the state.
///
/// # Protocol Support
///
/// Supports the standard Nanonis TCP command set including:
/// - Signal reading (`Signals.ValsGet`, `Signals.NamesGet`)
/// - Bias control (`Bias.Set`, `Bias.Get`)
/// - Position control (`FolMe.XYPosSet`, `FolMe.XYPosGet`)
/// - Motor control (`Motor.*` commands)
/// - Auto-approach (`AutoApproach.*` commands)
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// use nanonis_rs::NanonisClient;
///
/// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
///
/// // Read signal names
/// let signals = client.signal_names_get()?;
///
/// // Set bias voltage
/// client.bias_set(1.0)?;
///
/// // Read signal values
/// let values = client.signals_vals_get(vec![0, 1, 2], true)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// With builder pattern:
/// ```no_run
/// use std::time::Duration;
/// use nanonis_rs::NanonisClient;
///
/// let mut client = NanonisClient::builder()
///     .address("192.168.1.100")
///     .port(6501)
///     .debug(true)
///     .connect_timeout(Duration::from_secs(30))
///     .build()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct NanonisClient {
    stream: TcpStream,
    address: String,
    port: u16,
    debug: bool,
    config: ConnectionConfig,
    safe_tip_on_drop: bool,
    poisoned: bool,
}

impl NanonisClient {
    /// Create a new client with default configuration.
    ///
    /// This is the most convenient way to create a client for basic usage.
    /// Uses default timeouts and disables debug logging.
    ///
    /// # Arguments
    /// * `addr` - Server address (e.g., "127.0.0.1")
    /// * `port` - Server port (e.g., 6501)
    ///
    /// # Returns
    /// A connected `NanonisClient` ready for use.
    ///
    /// # Errors
    /// Returns `NanonisError` if:
    /// - The address format is invalid
    /// - Connection to the server fails
    /// - Connection times out
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let client = NanonisClient::new("127.0.0.1", 6501)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(addr: &str, port: u16) -> Result<Self, NanonisError> {
        Self::builder().address(addr).port(port).build()
    }

    /// Create a builder for flexible configuration.
    ///
    /// Use this when you need to customize timeouts, enable debug logging,
    /// or other advanced configuration options.
    ///
    /// # Returns
    /// A `NanonisClientBuilder` with default settings that can be customized.
    ///
    /// # Examples
    /// ```no_run
    /// use std::time::Duration;
    /// use nanonis_rs::NanonisClient;
    ///
    /// let client = NanonisClient::builder()
    ///     .address("192.168.1.100")
    ///     .port(6501)
    ///     .debug(true)
    ///     .connect_timeout(Duration::from_secs(30))
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn builder() -> NanonisClientBuilder {
        NanonisClientBuilder::default()
    }

    /// Create a new client with custom configuration (legacy method).
    ///
    /// **Deprecated**: Use [`NanonisClient::builder()`] instead for more flexibility.
    ///
    /// # Arguments
    /// * `addr` - Server address in format "host:port"
    /// * `config` - Connection configuration with custom timeouts
    #[deprecated(since = "0.2.0", note = "Use NanonisClient::builder() instead")]
    pub fn with_config(addr: &str, config: ConnectionConfig) -> Result<Self, NanonisError> {
        let (host, port_str) = addr.rsplit_once(':').ok_or_else(|| {
            NanonisError::Protocol(format!(
                "Invalid address format '{addr}': expected 'host:port'"
            ))
        })?;
        let port: u16 = port_str.parse().map_err(|_| {
            NanonisError::Protocol(format!("Invalid port in address '{addr}'"))
        })?;
        Self::builder().address(host).port(port).config(config).build()
    }

    /// Enable or disable debug output
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Get the current connection configuration
    pub fn config(&self) -> &ConnectionConfig {
        &self.config
    }

    /// Returns `true` if the connection has been poisoned by an I/O error.
    ///
    /// After a failed read or write, the TCP stream may be in a desynchronized
    /// state where subsequent commands would parse garbage data. The client
    /// refuses further commands until [`reconnect()`](Self::reconnect) is called.
    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    /// Re-establish the TCP connection using the original address and configuration.
    ///
    /// This drops the old stream, connects a new one, and clears the poisoned flag.
    /// Use this after an I/O error has poisoned the client, or when the Nanonis
    /// application has been restarted.
    pub fn reconnect(&mut self) -> Result<(), NanonisError> {
        let socket_addr: SocketAddr = format!("{}:{}", self.address, self.port)
            .parse()
            .map_err(|_| {
                NanonisError::Protocol(format!("Invalid address: {}", self.address))
            })?;

        debug!("Reconnecting to Nanonis at {}:{}", self.address, self.port);

        let stream =
            TcpStream::connect_timeout(&socket_addr, self.config.connect_timeout).map_err(
                |e| {
                    warn!("Failed to reconnect to {}:{}: {e}", self.address, self.port);
                    NanonisError::from_io(
                        e,
                        format!("Failed to reconnect to {}:{}", self.address, self.port),
                    )
                },
            )?;

        stream.set_read_timeout(Some(self.config.read_timeout))?;
        stream.set_write_timeout(Some(self.config.write_timeout))?;

        self.stream = stream;
        self.poisoned = false;

        debug!("Successfully reconnected to Nanonis");
        Ok(())
    }

    /// Send a quick command with minimal response handling.
    ///
    /// This is a low-level method for sending custom commands that don't fit
    /// the standard method patterns. Most users should use the specific
    /// command methods instead.
    pub fn quick_send(
        &mut self,
        command: &str,
        args: Vec<NanonisValue>,
        argument_types: Vec<&str>,
        return_types: Vec<&str>,
    ) -> Result<Vec<NanonisValue>, NanonisError> {
        // Refuse commands on a poisoned connection to prevent desynchronized reads
        if self.poisoned {
            return Err(NanonisError::Io {
                source: std::io::Error::new(
                    std::io::ErrorKind::NotConnected,
                    "connection poisoned after previous I/O error",
                ),
                context: format!(
                    "Cannot send '{command}': call reconnect() to re-establish the connection"
                ),
            });
        }

        debug!("=== COMMAND START: {} ===", command);
        debug!("Arguments: {:?}", args);
        debug!("Argument types: {:?}", argument_types);
        debug!("Return types: {:?}", return_types);

        // Serialize arguments
        let mut body = Vec::new();
        for (arg, arg_type) in args.iter().zip(argument_types.iter()) {
            debug!("Serializing {:?} as {}", arg, arg_type);
            Protocol::serialize_value(arg, arg_type, &mut body)?;
        }

        // Create command header
        let header = Protocol::create_command_header(command, body.len() as u32);

        debug!("Header size: {}, Body size: {}", header.len(), body.len());
        debug!("Full header bytes: {:02x?}", header);
        debug!(
            "Command in header: {:?}",
            String::from_utf8_lossy(&header[0..32]).trim_end_matches('\0')
        );
        debug!(
            "Body size in header: {}",
            u32::from_be_bytes([header[32], header[33], header[34], header[35]])
        );

        if !body.is_empty() {
            debug!("Body bytes: {:02x?}", body);
        }

        // Send command
        // Any I/O failure from here on poisons the connection, because the
        // stream may be left in a partially-written or partially-read state.
        debug!("Sending header ({} bytes)...", header.len());
        self.stream.write_all(&header).map_err(|e| {
            debug!("Failed to write header: {}", e);
            self.poisoned = true;
            NanonisError::from_io(e, "Writing command header")
        })?;

        if !body.is_empty() {
            debug!("Sending body ({} bytes)...", body.len());
            self.stream.write_all(&body).map_err(|e| {
                debug!("Failed to write body: {}", e);
                self.poisoned = true;
                NanonisError::from_io(e, "Writing command body")
            })?;
        }

        debug!("Command data sent successfully");

        // Read response header with improved error handling
        debug!("Reading response header ({} bytes)...", HEADER_SIZE);
        let response_header =
            Protocol::read_exact_bytes::<HEADER_SIZE>(&mut self.stream).map_err(|e| {
                debug!("Failed to read response header: {}", e);
                self.poisoned = true;
                e
            })?;

        debug!("Response header received: {:02x?}", response_header);
        debug!(
            "Response command: {:?}",
            String::from_utf8_lossy(&response_header[0..32]).trim_end_matches('\0')
        );

        // Validate and get body size
        let body_size = Protocol::validate_response_header(&response_header, command)?;
        debug!("Expected response body size: {}", body_size);

        // Read response body with size validation
        let response_body = if body_size > 0 {
            debug!("Reading response body ({} bytes)...", body_size);
            let body = Protocol::read_variable_bytes(&mut self.stream, body_size as usize)
                .map_err(|e| {
                    debug!("Failed to read response body: {}", e);
                    self.poisoned = true;
                    e
                })?;
            debug!(
                "Response body received ({} bytes): {:02x?}",
                body.len(),
                if body.len() <= 100 {
                    &body[..]
                } else {
                    &body[..100]
                }
            );
            body
        } else {
            debug!("No response body expected");
            Vec::new()
        };

        // Parse response with error checking
        debug!("Parsing response with types: {:?}", return_types);
        let result = Protocol::parse_response_with_error_check(&response_body, &return_types)
            .map_err(|e| {
                debug!("Failed to parse response: {}", e);
                e
            })?;

        // Validate that the parsed result has the expected number of values.
        // parse_response should always produce exactly one value per type descriptor,
        // but this guard prevents panics in callers if there's ever a mismatch.
        if result.len() < return_types.len() {
            return Err(NanonisError::Protocol(format!(
                "{command}: expected {} return values, got {}",
                return_types.len(),
                result.len()
            )));
        }

        debug!("=== COMMAND SUCCESS: {} ===", command);
        debug!("Parsed result: {:?}", result);

        Ok(result)
    }
}

impl Drop for NanonisClient {
    fn drop(&mut self) {
        if self.safe_tip_on_drop {
            // Temporarily clear poisoned flag so safety commands can attempt to run.
            // These are best-effort anyway (errors are ignored via `let _ =`).
            self.poisoned = false;
            use motor::{MotorDirection, MotorGroup};
            let _ = self.z_ctrl_withdraw(false, Duration::from_secs(2));
            let _ = self.motor_start_move(MotorDirection::ZMinus, 15u16, MotorGroup::Group1, false);
        }
    }
}
