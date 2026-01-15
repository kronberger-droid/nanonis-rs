use crate::error::NanonisError;
use serde::{Deserialize, Serialize};

// ==================== TCP Logger Types ====================

/// TCP channel index (0-23)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelIndex(pub u8);

impl ChannelIndex {
    pub fn new(index: u8) -> Result<Self, String> {
        if index <= 23 {
            Ok(Self(index))
        } else {
            Err(format!("Channel index {} out of range (0-23)", index))
        }
    }

    pub const fn new_unchecked(index: u8) -> Self {
        Self(index)
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

impl std::fmt::Display for ChannelIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for ChannelIndex {
    fn from(index: u8) -> Self {
        Self::new(index).unwrap_or_else(|_| {
            log::warn!(
                "Creating ChannelIndex from out-of-range value {}, clamping to 23",
                index
            );
            Self(23.min(index))
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TCPLogStatus {
    Disconnected = 0,
    Idle = 1,
    Start = 2,
    Stop = 3,
    Running = 4,
    TCPConnect = 5,
    TCPDisconnect = 6,
    BufferOverflow = 7,
}

impl From<TCPLogStatus> for i32 {
    fn from(status: TCPLogStatus) -> Self {
        status as i32
    }
}

impl TryFrom<i32> for TCPLogStatus {
    type Error = NanonisError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TCPLogStatus::Disconnected),
            1 => Ok(TCPLogStatus::Idle),
            2 => Ok(TCPLogStatus::Start),
            3 => Ok(TCPLogStatus::Stop),
            4 => Ok(TCPLogStatus::Running),
            5 => Ok(TCPLogStatus::TCPConnect),
            6 => Ok(TCPLogStatus::TCPDisconnect),
            7 => Ok(TCPLogStatus::BufferOverflow),
            _ => Err(NanonisError::InvalidCommand(format!(
                "Invalid TCP Logger status: {}",
                value
            ))),
        }
    }
}

impl std::fmt::Display for TCPLogStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            TCPLogStatus::Disconnected => "Disconnected",
            TCPLogStatus::Idle => "Idle",
            TCPLogStatus::Start => "Start",
            TCPLogStatus::Stop => "Stop",
            TCPLogStatus::Running => "Running",
            TCPLogStatus::TCPConnect => "TCP Connect",
            TCPLogStatus::TCPDisconnect => "TCP Disconnect",
            TCPLogStatus::BufferOverflow => "Buffer Overflow",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Debug, Clone)]
pub struct TCPLoggerData {
    pub num_channels: u32,
    pub oversampling: f32,
    pub counter: u64,
    pub state: TCPLogStatus,
    pub data: Vec<f32>,
}
