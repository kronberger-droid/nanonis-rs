pub mod client;
pub mod error;
pub mod protocol;
pub mod tcplogger_stream;
pub mod types;

// Core types (root level)
pub use error::NanonisError;
pub use types::{NanonisValue, Position};
pub use protocol::Protocol;
pub use tcplogger_stream::TCPLoggerStream;

// Client (root level)
pub use client::{
    NanonisClient, ConnectionConfig, NanonisClientBuilder,
    TipShaperConfig, TipShaperProps, VersionInfo, ZSpectroscopyResult,
};

// Re-export all domain types at root for backward compatibility
pub use client::motor::*;
pub use client::scan::*;
pub use client::z_ctrl::*;
pub use client::oscilloscope::*;
pub use client::signals::*;
pub use client::tcplog::*;
pub use client::bias::*;

// Domain modules - users can also import from here
pub mod motor {
    pub use crate::client::motor::*;
}

pub mod scan {
    pub use crate::client::scan::*;
}

pub mod z_controller {
    pub use crate::client::z_ctrl::*;
}

pub mod oscilloscope {
    pub use crate::client::oscilloscope::*;
}

pub mod signals {
    pub use crate::client::signals::*;
}

pub mod tcplog {
    pub use crate::client::tcplog::*;
}

pub mod bias {
    pub use crate::client::bias::*;
}

// Other client modules remain at root for now (less domain-specific)
pub use client::{
    pll, current, bias_sweep, folme, auto_approach,
    safe_tip, tip_recovery, util, z_spectr,
};
