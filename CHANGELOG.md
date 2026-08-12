# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-08-12

### Added

- `scan_wait_end_of_line` (`Scan.WaitEndOfLine`): blocks until the next
  completed scan line and returns `ScanLineEnd` — line number, movement
  type (`ScanLineMovement`), and MultiPass pass number. The
  synchronization point for per-line processing.
- `Serialize`/`Deserialize` on `ZControllerStatus` and `OsciData`, so
  downstream event logs can carry them without mirror types.
- Panic-path clippy lints (`unwrap_used`, `expect_used`, `panic`, and
  friends) are denied crate-wide via `[lints.clippy]`; tests are exempt
  through `clippy.toml`. A client crate driving hardware must not take
  the caller down. `indexing_slicing`, `arithmetic_side_effects` and
  `as_conversions` are wanted too but need larger changes first; they
  stay commented in `Cargo.toml` with hit counts.

### Changed

- **Breaking:** `TCPLoggerStream::spawn_background_reader` returns
  `Result<BackgroundReader, NanonisError>` instead of panicking when the
  OS refuses to spawn the reader thread (the only panic path in the
  library).

## [0.4.0] - 2026-05-27

### Fixed

- `Scan.PropsGet` is now tolerant of older Nanonis firmware. The reply layout grew
  across firmware versions (the modules-parameters block and Auto-Paste field were
  added later), so the fixed 16-field parser overran the shorter reply and failed
  with `UnexpectedEof`. It now tries the full documented layout and falls back to
  the core fields when the body is too short.
- `TriggerMode` conversion from integers now uses `TryFrom` instead of infallible
  `From`. Unknown values return `NanonisError::Protocol` instead of silently
  defaulting to `TriggerMode::Immediate`. This was the only enum in the crate that
  used an infallible conversion for protocol values.
- `StepCount` conversion from `u32` now uses `TryFrom`. Values exceeding `u16::MAX`
  (65535) return an error instead of silently wrapping to zero.
- `ChannelIndex::new()` now returns `Result<Self, NanonisError>` instead of
  `Result<Self, String>`, consistent with `TryFrom<u8> for ChannelIndex`.

### Added

- `NanonisClient::quick_send_raw()`: sends a command and returns the raw, unparsed
  response body, for commands whose reply layout varies across firmware versions.
- `TCPLoggerStream::read_frame()` now automatically skips the counter-0 metadata
  frame that Nanonis sends when the TCP logger starts. Callers no longer need to
  handle this protocol detail. Use `read_frame_raw()` to access all frames including
  metadata.
- Protocol constants: `MAX_SIGNAL_INDEX` (127), `MAX_TCP_CHANNEL` (23),
  `BASE_ACQUISITION_RATE_HZ` (2000.0).
- `osci1t_trig_set_typed()` and `osci1t_trig_get_typed()`: type-safe trigger
  configuration using `OsciTriggerMode` and `TriggerSlope` enums instead of raw
  `u16`. The original raw-`u16` methods remain for backwards compatibility.
- `signals_vals_get_by_idx(&[SignalIndex])`: type-safe alternative to
  `signals_vals_get(Vec<i32>)` for bulk signal reading.
- Re-exported `pll_freq_swp` module with `PLLFreqSwpParams`, `PLLFreqSwpData`, etc.

### Changed

- **Breaking**: `TriggerMode`: `From<u16>` and `From<i32>` replaced by `TryFrom`.
- **Breaking**: `StepCount`: `From<u32>` replaced by `TryFrom<u32>`.
- **Breaking**: `ChannelIndex::new()` return type changed from `Result<Self, String>`
  to `Result<Self, NanonisError>`.

### Deprecated

- `TCPLoggerData`: use `SignalFrame` from the `signals` module instead. Will be
  removed in a future release.
- `TipShaperProps` type alias and `tip_shaper_props_get()`: use
  `tip_shaper_config_get()` which returns the type-safe `TipShaperConfig` struct.

### Documentation

- Fixed README: corrected version (0.1.0 -> 0.3.0), TCPLogger streaming example
  (wrong method names and return types), error handling example (wrong variant names),
  motor import path.
- Updated CLAUDE.md: fixed stale architecture description that claimed domain types
  live in `types.rs` (they now live in their respective modules).
- Documented the Nanonis boolean inversion convention in `tip_shaper_config_get()`
  where `0 = yes/enabled` and `1 = no/disabled` per the TCP protocol spec.

## [0.3.0] - 2026-03-30

### Changed

- **Breaking**: Reorganized all domain types into dedicated modules (`motor::*`,
  `scan::*`, `oscilloscope::*`, etc.) instead of a flat `types.rs`.

## [0.2.0] - 2026-03-09

### Fixed

- `scan_frame_get()` now correctly extracts f32 values instead of attempting f64
  extraction on f32 wire data, which would cause a runtime Protocol error.
- `scan_frame_data_grab()` now correctly handles the `Array2DF32` value returned
  by the `"2f"` type descriptor, instead of trying to extract a flat `ArrayF32`.
- `tip_rec_data_get()` same 2D array fix as `scan_frame_data_grab()`.
- `tip_shaper_config_get()` now returns `NanonisError::Protocol` for invalid
  `restore_feedback` and `change_bias` values instead of panicking.
- `tcplog_status_get()` replaced stray `println!` debug output with `log::debug!`.

### Changed

- **Breaking**: `parse_response()` now returns `(Vec<NanonisValue>, usize)` instead
  of `Vec<NanonisValue>`. The second element is the cursor position after parsing,
  used internally for error section location. This is a low-level API; most users
  interact through `NanonisClient` methods and are unaffected.
- **Breaking**: `MotorAxis` conversion from integers now uses `TryFrom<u16>` and
  `TryFrom<i32>` instead of `From`. Invalid values return
  `NanonisError::Protocol` instead of silently defaulting to `MotorAxis::All`.
- **Breaking**: `ChannelIndex` conversion from `u8` now uses `TryFrom<u8>` instead
  of `From`. Out-of-range values return `NanonisError::Protocol` instead of
  silently clamping to 23.

### Removed

- Removed unused `ArrayU16` and `ArrayI16` variants from `NanonisValue`. These had
  no serialization or deserialization support and were dead code.
- Removed orphaned `src/mod.rs` that duplicated re-exports already in `lib.rs`.
- Eliminated `calculate_cursor_position()` (~150 lines) which duplicated the
  parsing logic in `parse_response()`. Cursor position is now computed in a single
  pass.

## [0.1.0] - 2025-01-21

### Added

- Initial release with full Nanonis TCP protocol support.
- `NanonisClient` with builder pattern for connection configuration.
- Support for bias, motor, scan, z-controller, oscilloscope, signals, PLL,
  tip recovery, safe tip, spectroscopy, and TCP logger commands.
- `TCPLoggerStream` for continuous data acquisition.
- Type-safe domain types with `From`/`TryFrom` conversions.
