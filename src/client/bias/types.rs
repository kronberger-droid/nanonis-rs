// ==================== Bias Control Types ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseMode {
    Keep = 0,
    Relative = 1,
    Absolute = 2,
}

impl From<PulseMode> for u16 {
    fn from(mode: PulseMode) -> Self {
        mode as u16
    }
}
