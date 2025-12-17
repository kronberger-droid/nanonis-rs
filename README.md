# nanonis-rs

[![Crates.io](https://img.shields.io/crates/v/nanonis-rs.svg)](https://crates.io/crates/nanonis-rs)
[![Documentation](https://docs.rs/nanonis-rs/badge.svg)](https://docs.rs/nanonis-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust client library for communicating with [Nanonis](https://www.specs-group.com/nanonis/) SPM (Scanning Probe Microscopy) systems via the Nanonis TCP protocol. This library provides a type-safe, high-level interface for controlling scanning probe microscopes and reading measurement data.

## Features

- **Type-safe API**: Strongly-typed Rust interface to Nanonis commands
- **Comprehensive coverage**: Support for motors, scanning, spectroscopy, oscilloscopes, and more
- **Error handling**: Robust error types with detailed context
- **TCPLogger streaming**: Continuous data acquisition support
- **Safety features**: Automatic tip withdrawal and motor positioning on client drop
- **Protocol abstraction**: Low-level protocol details handled internally

## Supported Functionality

- **Bias control**: Set and read bias voltage
- **Signal acquisition**: Read signal values and metadata
- **Motor control**: Precise control of XYZ motors and FolMe positioning
- **Z-controller**: Z-feedback loop control and monitoring
- **Scanning**: Scan control and data acquisition
- **Auto-approach**: Automated tip approach functionality
- **Oscilloscopes**: Single-trigger, dual-trigger, and high-resolution modes
- **Spectroscopy**: Z-spectroscopy and bias sweeps
- **Tip management**: Tip shaping and recovery procedures
- **PLL control**: Phase-locked loop operations
- **TCPLogger**: Real-time data streaming

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
nanonis-rs = "0.0.3"
```

### Basic Example

```rust
use nanonis_rs::{NanonisClient, NanonisError};
use std::time::Duration;

fn main() -> Result<(), NanonisError> {
    // Connect to Nanonis system
    let mut client = NanonisClient::builder()
        .address("192.168.1.100:6501")
        .timeout(Duration::from_secs(5))
        .build()?;

    // Get current bias voltage
    let bias = client.bias_get()?;
    println!("Current bias: {} V", bias);

    // Set new bias voltage
    client.bias_set(0.5)?;

    // Read signal values
    let signals = client.signals_vals_get()?;
    println!("Signals: {:?}", signals);

    Ok(())
}
```

### Motor Control Example

```rust
use nanonis_rs::{NanonisClient, MotorDirection, MotorGroup};
use std::time::Duration;

fn main() -> Result<(), NanonisError> {
    let mut client = NanonisClient::connect("192.168.1.100:6501")?;

    // Move motor with specified steps
    client.motor_start_move(
        MotorDirection::ZPlus,
        1000,  // steps
        MotorGroup::Group1,
        false  // non-blocking
    )?;

    // Wait for movement to complete
    std::thread::sleep(Duration::from_millis(500));

    Ok(())
}
```

### TCPLogger Streaming Example

```rust
use nanonis_rs::TCPLoggerStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TCPLoggerStream::connect("192.168.1.100:6502")?;

    // Read streaming data
    loop {
        let data = stream.read_data()?;
        println!("Timestamp: {}, Channels: {}",
                 data.timestamp, data.channels.len());

        // Process data...
    }
}
```

## Architecture

The library is organized into several layers:

- **Protocol layer** (`protocol.rs`): Low-level TCP message encoding/decoding
- **Type system** (`types.rs`): Protocol value types and domain-specific types
- **Client layer** (`client/`): High-level API organized by functionality
- **Error handling** (`error.rs`): Comprehensive error types with context

All communication with Nanonis follows a request-response pattern with strongly-typed inputs and outputs.

## Connection Configuration

Create clients using the builder pattern for advanced configuration:

```rust
use nanonis_rs::NanonisClient;
use std::time::Duration;

let client = NanonisClient::builder()
    .address("192.168.1.100:6501")
    .timeout(Duration::from_secs(10))
    .max_response_size(100_000_000)  // 100 MB
    .build()?;
```

Or use the simpler connect method:

```rust
let client = NanonisClient::connect("192.168.1.100:6501")?;
```

## Safety Features

The client implements automatic safety features:

- **Auto-withdrawal**: Z-controller withdraws tip when client is dropped
- **Motor positioning**: Motors automatically move to safe position on drop
- **Timeout protection**: All operations have configurable timeouts

## Error Handling

The library uses typed errors via `NanonisError`:

```rust
match client.bias_get() {
    Ok(bias) => println!("Bias: {} V", bias),
    Err(NanonisError::Timeout(msg)) => eprintln!("Timeout: {}", msg),
    Err(NanonisError::Server { code, message }) => {
        eprintln!("Server error {}: {}", code, message)
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/nanonis-rs). The documentation includes detailed examples for each command.

## Requirements

- Rust 2021 edition or later
- Nanonis system with TCP interface enabled
- Network access to Nanonis controller

## Development

This project uses Nix flakes for reproducible development environments:

```bash
# Enter development shell
nix develop

# Build the library
cargo build

# Run tests (requires Nanonis connection)
cargo test

# Build documentation
cargo doc --open
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues on [GitHub](https://github.com/kronberger-droid/nanonis-rs).

## Acknowledgments

This library implements the Nanonis TCP protocol as documented by SPECS Zurich GmbH. Nanonis is a registered trademark of SPECS Zurich GmbH.
