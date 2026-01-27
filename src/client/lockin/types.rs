// ==================== Lock-In Amplifier Types ====================

/// Demodulator RT signal output mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RTSignalMode {
    /// X/Y (Cartesian) output
    #[default]
    XY = 0,
    /// R/phi (Polar) output
    RPhi = 1,
}

impl From<RTSignalMode> for u32 {
    fn from(mode: RTSignalMode) -> Self {
        mode as u32
    }
}

impl TryFrom<u32> for RTSignalMode {
    type Error = crate::error::NanonisError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RTSignalMode::XY),
            1 => Ok(RTSignalMode::RPhi),
            _ => Err(crate::error::NanonisError::Protocol(format!(
                "Invalid RTSignalMode value: {}",
                value
            ))),
        }
    }
}

/// Lock-in modulator configuration.
#[derive(Debug, Clone)]
pub struct ModulatorConfig {
    /// Modulator number (1-8)
    pub number: i32,
    /// Whether modulator is on
    pub enabled: bool,
    /// Signal index being modulated
    pub signal_index: i32,
    /// Phase register index (1-8)
    pub phase_register: i32,
    /// Harmonic number (1 = base frequency)
    pub harmonic: i32,
    /// Phase offset in degrees
    pub phase_deg: f32,
    /// Modulation amplitude
    pub amplitude: f32,
    /// Frequency in Hz
    pub frequency_hz: f64,
}

impl Default for ModulatorConfig {
    fn default() -> Self {
        Self {
            number: 1,
            enabled: false,
            signal_index: 0,
            phase_register: 1,
            harmonic: 1,
            phase_deg: 0.0,
            amplitude: 0.0,
            frequency_hz: 1000.0,
        }
    }
}

/// Lock-in demodulator configuration.
#[derive(Debug, Clone)]
pub struct DemodulatorConfig {
    /// Demodulator number (1-8)
    pub number: i32,
    /// Signal index being demodulated
    pub signal_index: i32,
    /// Harmonic number (1 = base frequency)
    pub harmonic: i32,
    /// High-pass filter order (0 = off, 1-8)
    pub hp_filter_order: i32,
    /// High-pass filter cutoff frequency in Hz
    pub hp_filter_cutoff_hz: f32,
    /// Low-pass filter order (0 = off, 1-8)
    pub lp_filter_order: i32,
    /// Low-pass filter cutoff frequency in Hz
    pub lp_filter_cutoff_hz: f32,
    /// Phase register index (1-8)
    pub phase_register: i32,
    /// Reference phase in degrees
    pub phase_deg: f32,
    /// Sync filter enabled
    pub sync_filter: bool,
    /// RT signal mode (X/Y or R/phi)
    pub rt_signal_mode: RTSignalMode,
}

impl Default for DemodulatorConfig {
    fn default() -> Self {
        Self {
            number: 1,
            signal_index: 0,
            harmonic: 1,
            hp_filter_order: 0,
            hp_filter_cutoff_hz: 10.0,
            lp_filter_order: 4,
            lp_filter_cutoff_hz: 100.0,
            phase_register: 1,
            phase_deg: 0.0,
            sync_filter: false,
            rt_signal_mode: RTSignalMode::XY,
        }
    }
}

/// High-pass or low-pass filter configuration.
#[derive(Debug, Clone, Copy)]
pub struct FilterConfig {
    /// Filter order (0 = off, 1-8 active)
    pub order: i32,
    /// Cutoff frequency in Hz
    pub cutoff_hz: f32,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            order: 0,
            cutoff_hz: 100.0,
        }
    }
}
