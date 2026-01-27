use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Digital port selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigitalPort {
    /// Port A
    #[default]
    PortA = 0,
    /// Port B
    PortB = 1,
    /// Port C
    PortC = 2,
    /// Port D
    PortD = 3,
}

impl From<DigitalPort> for u32 {
    fn from(port: DigitalPort) -> Self {
        port as u32
    }
}

impl From<DigitalPort> for u16 {
    fn from(port: DigitalPort) -> Self {
        port as u16
    }
}

/// Digital line direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigitalDirection {
    /// Input direction
    #[default]
    Input = 0,
    /// Output direction
    Output = 1,
}

impl From<DigitalDirection> for u32 {
    fn from(dir: DigitalDirection) -> Self {
        dir as u32
    }
}

/// Digital line polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigitalPolarity {
    /// Low active
    #[default]
    LowActive = 0,
    /// High active
    HighActive = 1,
}

impl From<DigitalPolarity> for u32 {
    fn from(pol: DigitalPolarity) -> Self {
        pol as u32
    }
}

/// Digital line configuration.
#[derive(Debug, Clone, Copy)]
pub struct DigitalLineConfig {
    /// Digital line number (1-8)
    pub line: u32,
    /// Port selection
    pub port: DigitalPort,
    /// Line direction (input/output)
    pub direction: DigitalDirection,
    /// Line polarity
    pub polarity: DigitalPolarity,
}

impl Default for DigitalLineConfig {
    fn default() -> Self {
        Self {
            line: 1,
            port: DigitalPort::PortA,
            direction: DigitalDirection::Input,
            polarity: DigitalPolarity::LowActive,
        }
    }
}

/// Pulse generator configuration.
#[derive(Debug, Clone)]
pub struct PulseConfig {
    /// Port selection
    pub port: DigitalPort,
    /// Digital lines to pulse (1-8)
    pub lines: Vec<u8>,
    /// Pulse width in seconds
    pub pulse_width_s: f32,
    /// Pulse pause in seconds
    pub pulse_pause_s: f32,
    /// Number of pulses (1-32767)
    pub num_pulses: i32,
    /// Wait until all pulses are generated before returning
    pub wait_until_finished: bool,
}

impl Default for PulseConfig {
    fn default() -> Self {
        Self {
            port: DigitalPort::PortA,
            lines: vec![1],
            pulse_width_s: 0.001,
            pulse_pause_s: 0.001,
            num_pulses: 1,
            wait_until_finished: true,
        }
    }
}

impl NanonisClient {
    /// Configure the properties of a digital line.
    ///
    /// # Arguments
    /// * `config` - A [`DigitalLineConfig`] struct with line configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::dig_lines::{DigitalLineConfig, DigitalPort, DigitalDirection, DigitalPolarity};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = DigitalLineConfig {
    ///     line: 1,
    ///     port: DigitalPort::PortA,
    ///     direction: DigitalDirection::Output,
    ///     polarity: DigitalPolarity::HighActive,
    /// };
    /// client.dig_lines_props_set(&config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn dig_lines_props_set(&mut self, config: &DigitalLineConfig) -> Result<(), NanonisError> {
        self.quick_send(
            "DigLines.PropsSet",
            vec![
                NanonisValue::U32(config.line),
                NanonisValue::U32(config.port.into()),
                NanonisValue::U32(config.direction.into()),
                NanonisValue::U32(config.polarity.into()),
            ],
            vec!["I", "I", "I", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Set the status of a digital output line.
    ///
    /// # Arguments
    /// * `port` - Port selection
    /// * `line` - Digital line number (1-8)
    /// * `active` - True for active, false for inactive
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn dig_lines_out_status_set(
        &mut self,
        port: DigitalPort,
        line: u32,
        active: bool,
    ) -> Result<(), NanonisError> {
        let status = if active { 1u32 } else { 0u32 };
        self.quick_send(
            "DigLines.OutStatusSet",
            vec![
                NanonisValue::U32(port.into()),
                NanonisValue::U32(line),
                NanonisValue::U32(status),
            ],
            vec!["I", "I", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Read the TTL voltages present at the pins of the selected port.
    ///
    /// # Arguments
    /// * `port` - Port selection
    ///
    /// # Returns
    /// A vector of TTL values for each line (0 = inactive, 1 = active).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn dig_lines_ttl_val_get(&mut self, port: DigitalPort) -> Result<Vec<u32>, NanonisError> {
        let result = self.quick_send(
            "DigLines.TTLValGet",
            vec![NanonisValue::U16(port.into())],
            vec!["H"],
            vec!["i", "*I"],
        )?;

        if result.len() >= 2 {
            Ok(result[1].as_u32_array()?.to_vec())
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Configure and start the pulse generator on the selected digital outputs.
    ///
    /// # Arguments
    /// * `config` - A [`PulseConfig`] struct with pulse configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::dig_lines::{PulseConfig, DigitalPort};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = PulseConfig {
    ///     port: DigitalPort::PortA,
    ///     lines: vec![1, 2],
    ///     pulse_width_s: 0.001,
    ///     pulse_pause_s: 0.001,
    ///     num_pulses: 10,
    ///     wait_until_finished: true,
    /// };
    /// client.dig_lines_pulse(&config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn dig_lines_pulse(&mut self, config: &PulseConfig) -> Result<(), NanonisError> {
        let wait_flag = if config.wait_until_finished {
            1u32
        } else {
            0u32
        };

        self.quick_send(
            "DigLines.Pulse",
            vec![
                NanonisValue::U16(config.port.into()),
                NanonisValue::ArrayU8(config.lines.clone()),
                NanonisValue::F32(config.pulse_width_s),
                NanonisValue::F32(config.pulse_pause_s),
                NanonisValue::I32(config.num_pulses),
                NanonisValue::U32(wait_flag),
            ],
            vec!["H", "+*b", "f", "f", "i", "I"],
            vec![],
        )?;
        Ok(())
    }
}
