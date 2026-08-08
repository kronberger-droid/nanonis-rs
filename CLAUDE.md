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

### Module Organization (v0.3.0+, updated v0.4.0)

Domain-specific types are now organized into modules for better maintainability:

**Domain Modules:**
- `motor::*` - Motor control types (MotorDirection, MotorGroup, MotorAxis, StepCount, Frequency, Amplitude, Position3D, MovementMode, MotorMovement, MotorDisplacement)
- `scan::*` - Scan types (ScanFrame, ScanAction, ScanDirection, ScanConfig)
- `z_controller::*` - Z-controller types (ZControllerHold)
- `bias::*` - Bias types (PulseMode)
- `oscilloscope::*` - Oscilloscope types (TriggerMode, TriggerSlope, TriggerLevel, SampleCount, OsciTriggerMode, OversamplingIndex, TimebaseIndex, DataToGet, TriggerConfig, OsciData, OscilloscopeIndex)
- `signals::*` - Signal types (SignalIndex, SignalFrame)
- `tcplog::*` - TCP logger types (ChannelIndex, TCPLogStatus, TCPLoggerData)

**Import Examples:**
```rust
use nanonis_rs::NanonisClient;
use nanonis_rs::motor::{MotorDirection, MotorGroup};
use nanonis_rs::scan::{ScanFrame, ScanAction};
use nanonis_rs::oscilloscope::TriggerConfig;
```

**Core types at root:** `NanonisClient`, `NanonisError`, `NanonisValue`, `Position`

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

Domain types live in their respective modules (not in types.rs):
- `motor::*` — MotorDirection, MotorGroup, StepCount, etc.
- `scan::*` — ScanAction, ScanDirection, ScanFrame, etc.
- `oscilloscope::*` — TriggerMode, OsciData, TriggerConfig, etc.
- `signals::*` — SignalIndex, SignalFrame
- `z_ctrl::*` — ZControllerHold, ZControllerStatus

`types.rs` only contains `NanonisValue` and `Position`.

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

`NanonisError` uses thiserror for typed errors (4 variants):
- **Io { source, context }**: Network/I/O errors with context string
- **Timeout(String)**: Connection or operation timeouts with context
- **Protocol(String)**: Binary protocol parsing/validation errors and type mismatches
- **Server { code, message }**: Errors returned by the Nanonis server

All error handling uses explicit `.map_err()` calls - no extension traits or anyhow-style magic.

### TCPLogger Stream (`src/tcplogger_stream.rs`)

Provides streaming access to Nanonis TCPLogger data, a separate interface from the main command protocol for continuous data acquisition.

## Adding New Commands

When implementing a new Nanonis command:

1. Add the method to the appropriate module in `src/client/`
2. Follow the command pattern shown above
3. Look the command up in `docs/tcp-protocol.md` to get its argument and return types
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

### Protocol Reference (`docs/tcp-protocol.md`)

The full SPECS "Nanonis TCP Protocol" spec (R14718, April 2025) converted to markdown so it can be searched with `rg`. It is the authoritative source for every command's signature.

Each of the 661 commands is a `####` heading, its module a `###` heading:

```bash
# The full entry for one command
rg -A 20 '^#### Motor\.StartMove$' docs/tcp-protocol.md

# Every command in a module
rg '^#### Scan\.' docs/tcp-protocol.md
```

An entry lists a description, `Arguments:`, and `Return arguments (...)`, each argument written as `Name (type)` followed by prose. Map those types onto the descriptors above:

| Spec type | Descriptor |
|---|---|
| `int` / `unsigned int32` | `"i"` / `"I"` |
| `unsigned int16` | `"H"` |
| `float32` / `float64` | `"f"` / `"d"` |
| `string` | `"+*c"` |
| `1D array float32` | `"*f"`, or `"+*f"` when the length is prepended |
| `1D array string` | `"+*c"`, or `"*+c"` when the count is a preceding argument |
| `2D array float32` | `"2f"` |

Note that `"+*c"` covers both a single string and a string array; the encoder dispatches on the `NanonisValue` variant, not the descriptor (`src/protocol.rs:198`).

Watch for size arguments: a spec entry listing `Channels names size (int)`, `Number of channels (int)`, then `Channels names (1D array string)` describes a *single* `"+*c"` value, not three arguments. Those length prefixes are protocol framing, so they should not become Rust parameters.

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

`NanonisClient` optionally implements Drop to safely withdraw the tip and move motors away from the surface when the client is destroyed. This must be explicitly enabled via the builder:

```rust
let client = NanonisClient::builder()
    .address("127.0.0.1")
    .port(6501)
    .safe_tip_on_drop(true)  // opt-in
    .build()?;
```

When enabled, on drop it withdraws the Z-controller and moves motors in the ZMinus direction (away from surface). Default: disabled.
