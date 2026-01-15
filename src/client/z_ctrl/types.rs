// ==================== Z-Controller Types ====================

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
