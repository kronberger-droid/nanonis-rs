// ==================== Z-Controller Types ====================

use crate::error::NanonisError;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z_controller_hold_encoding() {
        assert_eq!(u16::from(ZControllerHold::NoChange), 0);
        assert_eq!(u16::from(ZControllerHold::Hold), 1);
        assert_eq!(u16::from(ZControllerHold::Release), 2);
    }

    #[test]
    fn z_home_mode_encoding() {
        assert_eq!(u16::from(ZHomeMode::NoChange), 0);
        assert_eq!(u16::from(ZHomeMode::Absolute), 1);
        assert_eq!(u16::from(ZHomeMode::Relative), 2);
    }

    #[test]
    fn z_controller_status_valid() {
        assert_eq!(
            ZControllerStatus::try_from(1u16).unwrap(),
            ZControllerStatus::Off
        );
        assert_eq!(
            ZControllerStatus::try_from(2u16).unwrap(),
            ZControllerStatus::On
        );
        assert_eq!(
            ZControllerStatus::try_from(3u16).unwrap(),
            ZControllerStatus::Hold
        );
        assert_eq!(
            ZControllerStatus::try_from(4u16).unwrap(),
            ZControllerStatus::SwitchingOff
        );
        assert_eq!(
            ZControllerStatus::try_from(5u16).unwrap(),
            ZControllerStatus::SafeTip
        );
        assert_eq!(
            ZControllerStatus::try_from(6u16).unwrap(),
            ZControllerStatus::Withdrawing
        );
    }

    #[test]
    fn z_controller_status_invalid() {
        assert!(ZControllerStatus::try_from(0u16).is_err()); // starts at 1
        assert!(ZControllerStatus::try_from(7u16).is_err());
    }
}
