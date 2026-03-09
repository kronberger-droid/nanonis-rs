# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
