# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

nanonis-rs is a Rust client library for communicating with Nanonis SPM (Scanning Probe Microscopy) systems via the Nanonis TCP protocol. It provides a type-safe, high-level interface for controlling scanning probe microscopes and reading measurement data.

## Build Commands

```bash
# Build the library
cargo build

# Build with all features and targets
cargo build --all-targets

# Run tests (includes doctests)
cargo test

# Build documentation
cargo doc --open

# Check code without building
cargo check
```

## Development Environment

This project uses Nix flakes for reproducible development environments:

```bash
# Enter development shell
nix develop

# Or with direnv (if .envrc is present)
direnv allow
```

The Nix shell provides:
- Rust stable toolchain via fenix
- rust-analyzer
- cargo-expand (for macro expansion)
- rusty-man (for documentation)

## Architecture

### Protocol Layer (`src/protocol.rs`)

The low-level protocol implementation handles Nanonis TCP message encoding/decoding:

- **Message format**: 40-byte header + variable-length body
- **Header structure**: 32-byte command name + 4-byte body size + 2-byte response flag + 2-byte padding
- **Type system**: Custom type descriptors (e.g., "f" for f32, "*f" for Vec<f32>, "+*c" for string arrays)
- **Error handling**: Server errors are encoded at the end of response bodies (8-byte error info section)

Key protocol details:
- All multi-byte values use big-endian byte order
- Response bodies contain both data and error information
- Type descriptors can reference previous values (e.g., "*f" uses preceding integer as array length)
- Maximum response size: 100MB

### Type System (`src/types.rs`)

`NanonisValue` is the core enum for protocol value serialization/deserialization:
- Supports primitives: U16, I16, U32, I32, F32, F64, String
- Supports arrays: ArrayU32, ArrayI32, ArrayF32, ArrayF64, ArrayString
- Supports 2D arrays: Array2DF32

Domain-specific types are also defined here:
- Motor control: MotorDirection, MotorGroup, MotorAxis, MovementMode
- Scan control: ScanAction, ScanDirection, ScanFrame
- Oscilloscope: TriggerMode, OsciData, SignalStats
- Position: Position, Position3D, StepCount

### Client Layer (`src/client/`)

`NanonisClient` is the high-level interface, organized by functionality:

- **mod.rs**: Core client structure, connection management, builder pattern
- **bias.rs**: Bias voltage control (Bias.Set, Bias.Get)
- **signals.rs**: Signal reading (Signals.ValsGet, Signals.NamesGet)
- **motor.rs**: Motor control (Motor.*, FolMe.* commands)
- **z_ctrl.rs**: Z-controller (Z.Ctrl.*)
- **scan.rs**: Scan control (Scan.*)
- **auto_approach.rs**: Auto-approach functionality
- **osci_*.rs**: Oscilloscope modules (1-trigger, 2-trigger, high-resolution)
- **z_spectr.rs**: Z-spectroscopy
- **tip_recovery.rs**: Tip shaping/recovery
- **pll.rs**: Phase-locked loop control
- **safe_tip.rs**: Safe tip operations
- **bias_sweep.rs**: Bias sweep functionality
- **tcplog.rs**: TCP logger interface

### Command Pattern

All client methods follow this pattern:

```rust
pub fn command_name(&mut self, arg1: Type1, arg2: Type2) -> Result<ReturnType, NanonisError> {
    let result = self.quick_send(
        "Nanonis.Command",
        vec![arg1.into(), arg2.into()],  // Arguments as NanonisValue
        vec!["f", "i"],                   // Argument type descriptors
        vec!["d", "*f"],                  // Response type descriptors
    )?;

    // Parse result into domain types
    Ok(parsed_value)
}
```

The `quick_send` method handles:
1. Serializing arguments according to type descriptors
2. Creating and sending command header + body
3. Reading and validating response header
4. Reading response body
5. Parsing response values with error checking

### Error Handling (`src/error.rs`)

`NanonisError` uses thiserror for typed errors (6 variants):
- **Io(std::io::Error)**: Network/I/O errors - uses `#[from]` for automatic conversion
- **Timeout(String)**: Connection or operation timeouts with context
- **Protocol(String)**: Binary protocol parsing/validation errors
- **Type(String)**: Internal type conversion failures (NanonisValue mismatches)
- **InvalidInput(String)**: Invalid user input or command parameters
- **Server { code, message }**: Errors returned by the Nanonis server

**Error context helper**: The `io_context()` helper function adds context to I/O errors:

```rust
// Internal use only
NanonisError::io_context(io_error, "writing command header")
```

This creates an I/O error with formatted context while preserving the original error kind. All error handling uses explicit `.map_err()` calls - no extension traits or anyhow-style magic.

### TCPLogger Stream (`src/tcplogger_stream.rs`)

Provides streaming access to Nanonis TCPLogger data, a separate interface from the main command protocol for continuous data acquisition.

## Adding New Commands

When implementing a new Nanonis command:

1. Add the method to the appropriate module in `src/client/`
2. Follow the command pattern shown above
3. Reference Nanonis TCP protocol documentation for type descriptors
4. Add comprehensive doc comments with examples (they become doctests)
5. Handle both success and error cases
6. Extract and convert return values appropriately

Example type descriptor patterns:
- `"f"` - single f32
- `"*f"` - f32 array (length from previous value)
- `"+*f"` - f32 array (length prepended)
- `"*+c"` - string array (count from previous value, each string length prepended)
- `"+*c"` - string array (total size + count prepended, each string length prepended)
- `"2f"` - 2D f32 array (dimensions from two previous values)

## Testing

The project uses Rust's doctest system extensively. All public API examples in doc comments are executable tests. To test a specific module:

```bash
# Test all doctests
cargo test --doc

# Test specific file's doctests
cargo test --doc client::bias
```

No unit tests are currently implemented - the doctests serve as integration tests requiring a real Nanonis connection.

## Resource Cleanup

`NanonisClient` implements Drop to safely withdraw the tip and move motors when the client is destroyed:

```rust
impl Drop for NanonisClient {
    fn drop(&mut self) {
        let _ = self.z_ctrl_withdraw(false, Duration::from_secs(1));
        let _ = self.motor_start_move(MotorDirection::ZPlus, 2u16, MotorGroup::Group1, false);
    }
}
```

This safety feature prevents tip crashes if client code panics or exits unexpectedly.
