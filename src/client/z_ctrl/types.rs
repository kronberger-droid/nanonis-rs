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

/// Z-Controller home position mode for `z_ctrl_home_props_set`.
///
/// Controls whether the tip returns to a fixed (absolute) position or a
/// position relative to where it currently is when `z_ctrl_home` is called.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZHomeMode {
    /// Leave the current mode selection unchanged.
    NoChange = 0,
    /// Home position is an absolute Z coordinate.
    Absolute = 1,
    /// Home position is relative to the current tip position.
    Relative = 2,
}

impl From<ZHomeMode> for u16 {
    fn from(mode: ZHomeMode) -> Self {
        mode as u16
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
