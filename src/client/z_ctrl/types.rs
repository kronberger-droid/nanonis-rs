// ==================== Z-Controller Types ====================

use crate::error::NanonisError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZControllerHold {
    NoChange = 0,
    Hold = 1,
    Release = 2,
}

impl From<ZControllerHold> for u16 {
    fn from(hold: ZControllerHold) -> Self {
        hold as u16
    }
}

/// Z-Controller status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ZControllerStatus {
    #[default]
    Off = 1,
    On = 2,
    Hold = 3,
    SwitchingOff = 4,
    SafeTip = 5,
    Withdrawing = 6,
}

impl TryFrom<u16> for ZControllerStatus {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Off),
            2 => Ok(Self::On),
            3 => Ok(Self::Hold),
            4 => Ok(Self::SwitchingOff),
            5 => Ok(Self::SafeTip),
            6 => Ok(Self::Withdrawing),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid Z-controller status: {}",
                value
            ))),
        }
    }
}
