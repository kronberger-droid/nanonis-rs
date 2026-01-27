//! # nanonis-rs
//!
//! A Rust client library for communicating with Nanonis SPM systems via TCP.
//!
//! ## Quick Start
//!
//! ```no_run
//! use nanonis_rs::{NanonisClient, NanonisError};
//!
//! fn main() -> Result<(), NanonisError> {
//!     let mut client = NanonisClient::new("192.168.1.100", 6501)?;
//!
//!     // Get current bias voltage
//!     let bias = client.bias_get()?;
//!     println!("Current bias: {} V", bias);
//!
//!     // Set new bias voltage
//!     client.bias_set(0.5)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Usage Pattern
//!
//! All instrument control is done through [`NanonisClient`]. Import types
//! from domain modules only when needed:
//!
//! ```no_run
//! use nanonis_rs::{NanonisClient, NanonisError};
//! use nanonis_rs::motor::{MotorDirection, MotorGroup};
//! use nanonis_rs::scan::ScanFrame;
//!
//! fn main() -> Result<(), NanonisError> {
//!     let mut client = NanonisClient::new("192.168.1.100", 6501)?;
//!
//!     // Motor control
//!     client.motor_start_move(MotorDirection::ZPlus, 100u16, MotorGroup::Group1, true)?;
//!
//!     // Scan control
//!     let frame = client.scan_frame_get()?;
//!     println!("Scan center: ({}, {})", frame.center.x, frame.center.y);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Streaming Data
//!
//! For continuous data acquisition, use [`TCPLoggerStream`]:
//!
//! ```no_run
//! use nanonis_rs::TCPLoggerStream;
//!
//! let mut stream = TCPLoggerStream::new("192.168.1.100", 6502)?;
//! let frame = stream.read_frame()?;
//! println!("Got {} channels", frame.data.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// Internal modules
mod client;
mod error;
mod protocol;
mod tcplogger_stream;
mod types;

// ==================== Public API ====================

pub use error::NanonisError;
pub use client::{NanonisClient, NanonisClientBuilder, ConnectionConfig};
pub use tcplogger_stream::TCPLoggerStream;

// Re-export commonly used types from the internal types module
pub use types::{NanonisValue, Position};

// ==================== Domain Type Modules ====================
//
// Import types from these modules as needed.

/// Motor control types.
///
/// ```
/// use nanonis_rs::motor::{MotorDirection, MotorGroup, MotorAxis};
/// ```
pub mod motor {
    pub use crate::client::motor::*;
}

/// Scan control types.
///
/// ```
/// use nanonis_rs::scan::{ScanFrame, ScanAction, ScanDirection};
/// ```
pub mod scan {
    pub use crate::client::scan::*;
}

/// Bias control types.
///
/// ```
/// use nanonis_rs::bias::PulseMode;
/// ```
pub mod bias {
    pub use crate::client::bias::*;
}

/// Bias spectroscopy types.
///
/// ```
/// use nanonis_rs::bias_spectr::{BiasSpectrPropsBuilder, OptionalFlag};
/// ```
pub mod bias_spectr {
    pub use crate::client::bias_spectr::*;
}

/// Z-controller types.
///
/// ```
/// use nanonis_rs::z_ctrl::{ZControllerHold, ZControllerStatus};
/// ```
pub mod z_ctrl {
    pub use crate::client::z_ctrl::*;
}

/// Oscilloscope types.
///
/// ```
/// use nanonis_rs::oscilloscope::{TriggerMode, TriggerSlope, OsciData};
/// ```
pub mod oscilloscope {
    pub use crate::client::oscilloscope::*;
}

/// Signal management types.
///
/// ```
/// use nanonis_rs::signals::SignalIndex;
/// ```
pub mod signals {
    pub use crate::client::signals::*;
}

/// Lock-in amplifier types.
pub mod lockin {
    pub use crate::client::lockin::*;
}

/// Piezo control types.
pub mod piezo {
    pub use crate::client::piezo::*;
}

/// User output types.
pub mod user_out {
    pub use crate::client::user_out::*;
}

/// TCP logger types.
pub mod tcplog {
    pub use crate::client::tcplog::*;
}

/// PLL (Phase-Locked Loop) types.
pub mod pll {
    pub use crate::client::pll::*;
}

/// Generic sweep types.
pub mod gen_swp {
    pub use crate::client::gen_swp::*;
}

/// High-speed sweep types.
pub mod hs_swp {
    pub use crate::client::hs_swp::*;
}

/// Pattern types.
pub mod pattern {
    pub use crate::client::pattern::*;
}

/// Follow-me types.
pub mod folme {
    pub use crate::client::folme::*;
}

/// Atom tracking types.
pub mod atom_track {
    pub use crate::client::atom_track::*;
}

/// Kelvin controller types.
pub mod kelvin_ctrl {
    pub use crate::client::kelvin_ctrl::*;
}

/// Spectrum analyzer types.
pub mod spectrum_anlzr {
    pub use crate::client::spectrum_anlzr::*;
}

/// PLL signal analyzer types.
pub mod pll_signal_anlzr {
    pub use crate::client::pll_signal_anlzr::*;
}

/// Generic PI controller types.
pub mod gen_pi_ctrl {
    pub use crate::client::gen_pi_ctrl::*;
}

/// Marks types.
pub mod marks {
    pub use crate::client::marks::*;
}

/// Data logging types.
pub mod data_log {
    pub use crate::client::data_log::*;
}

/// Script types.
pub mod script {
    pub use crate::client::script::*;
}

/// Interferometer types.
pub mod interf {
    pub use crate::client::interf::*;
}

/// Beam deflection types.
pub mod beam_defl {
    pub use crate::client::beam_defl::*;
}

/// OC sync types.
pub mod oc_sync {
    pub use crate::client::oc_sync::*;
}

/// Digital lines types.
pub mod dig_lines {
    pub use crate::client::dig_lines::*;
}

/// PI controller types.
pub mod pi_ctrl {
    pub use crate::client::pi_ctrl::*;
}

/// Lock-in frequency sweep types.
pub mod lockin_freq_swp {
    pub use crate::client::lockin_freq_swp::*;
}

/// Signal chart types.
pub mod signal_chart {
    pub use crate::client::signal_chart::*;
}

/// Tip recovery types.
pub mod tip_recovery {
    pub use crate::client::tip_recovery::*;
}

/// CPD compensation types.
pub mod cpd_comp {
    pub use crate::client::cpd_comp::*;
}

/// Z spectroscopy types.
pub mod z_spectr {
    pub use crate::client::z_spectr::*;
}

/// Utility types.
pub mod util {
    pub use crate::client::util::*;
}
