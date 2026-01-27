// ==================== Bias Spectroscopy Types ====================

use std::time::Duration;

/// Digital synchronization mode for bias spectroscopy.
///
/// Controls TTL/pulse sequence synchronization with spectroscopy measurement stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigitalSync {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// TTL synchronization disabled
    Off = 1,
    /// TTL synchronization enabled
    TTLSync = 2,
    /// Pulse sequence synchronization enabled
    PulseSequence = 3,
}

impl From<DigitalSync> for u16 {
    fn from(mode: DigitalSync) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for DigitalSync {
    type Error = crate::error::NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DigitalSync::Off),
            1 => Ok(DigitalSync::TTLSync),
            2 => Ok(DigitalSync::PulseSequence),
            _ => Err(crate::error::NanonisError::Type(format!(
                "Invalid DigitalSync value: {}",
                value
            ))),
        }
    }
}

/// TTL line selection for synchronization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TTLLine {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// High-speed digital line 1
    HSLine1 = 1,
    /// High-speed digital line 2
    HSLine2 = 2,
    /// High-speed digital line 3
    HSLine3 = 3,
    /// High-speed digital line 4
    HSLine4 = 4,
}

impl From<TTLLine> for u16 {
    fn from(line: TTLLine) -> Self {
        line as u16
    }
}

impl TryFrom<u16> for TTLLine {
    type Error = crate::error::NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TTLLine::HSLine1),
            1 => Ok(TTLLine::HSLine2),
            2 => Ok(TTLLine::HSLine3),
            3 => Ok(TTLLine::HSLine4),
            _ => Err(crate::error::NanonisError::Type(format!(
                "Invalid TTLLine value: {}",
                value
            ))),
        }
    }
}

/// TTL polarity for switching action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TTLPolarity {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// Active low polarity
    LowActive = 1,
    /// Active high polarity
    HighActive = 2,
}

impl From<TTLPolarity> for u16 {
    fn from(polarity: TTLPolarity) -> Self {
        polarity as u16
    }
}

impl TryFrom<u16> for TTLPolarity {
    type Error = crate::error::NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TTLPolarity::LowActive),
            1 => Ok(TTLPolarity::HighActive),
            _ => Err(crate::error::NanonisError::Type(format!(
                "Invalid TTLPolarity value: {}",
                value
            ))),
        }
    }
}

/// Optional flag for settings that support "no change" option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptionalFlag {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// Enable the option
    On = 1,
    /// Disable the option
    Off = 2,
}

impl From<OptionalFlag> for u16 {
    fn from(flag: OptionalFlag) -> Self {
        flag as u16
    }
}

impl TryFrom<u16> for OptionalFlag {
    type Error = crate::error::NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OptionalFlag::Off),
            1 => Ok(OptionalFlag::On),
            _ => Err(crate::error::NanonisError::Type(format!(
                "Invalid OptionalFlag value: {}",
                value
            ))),
        }
    }
}

/// Bias spectroscopy sweep mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SweepMode {
    /// Linear sweep mode
    #[default]
    Linear,
    /// Multi-Line Segment sweep mode
    MLS,
}

impl TryFrom<&str> for SweepMode {
    type Error = crate::error::NanonisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "linear" => Ok(SweepMode::Linear),
            "mls" => Ok(SweepMode::MLS),
            _ => Err(crate::error::NanonisError::Type(format!(
                "Invalid SweepMode: {}",
                value
            ))),
        }
    }
}

impl From<SweepMode> for &'static str {
    fn from(mode: SweepMode) -> Self {
        match mode {
            SweepMode::Linear => "Linear",
            SweepMode::MLS => "MLS",
        }
    }
}

/// Bias spectroscopy timing configuration.
#[derive(Debug, Clone)]
pub struct BiasSpectrTiming {
    /// Time for Z averaging before sweep
    pub z_averaging_time: Duration,
    /// Z offset applied before spectroscopy (positive = retract)
    pub z_offset_m: f32,
    /// Initial settling time before sweep starts
    pub initial_settling_time: Duration,
    /// Maximum slew rate in V/s
    pub max_slew_rate: f32,
    /// Settling time at each point
    pub settling_time: Duration,
    /// Integration time at each point
    pub integration_time: Duration,
    /// End settling time after sweep
    pub end_settling_time: Duration,
    /// Z control time at end
    pub z_control_time: Duration,
}

impl Default for BiasSpectrTiming {
    fn default() -> Self {
        Self {
            z_averaging_time: Duration::from_millis(100),
            z_offset_m: 0.0,
            initial_settling_time: Duration::from_millis(100),
            max_slew_rate: 1.0,
            settling_time: Duration::from_millis(10),
            integration_time: Duration::from_millis(20),
            end_settling_time: Duration::from_millis(100),
            z_control_time: Duration::from_millis(100),
        }
    }
}

/// Bias spectroscopy properties configuration.
#[derive(Debug, Clone)]
pub struct BiasSpectrProps {
    /// Whether to save individual sweep data with average
    pub save_all: bool,
    /// Number of sweeps to measure and average
    pub num_sweeps: i32,
    /// Whether to acquire backward sweep
    pub backward_sweep: bool,
    /// Number of points in the sweep
    pub num_points: i32,
    /// List of recorded channel names
    pub channels: Vec<String>,
    /// Sweep parameters
    pub parameters: Vec<String>,
    /// Fixed parameters
    pub fixed_parameters: Vec<String>,
}

/// Bias spectroscopy advanced properties.
#[derive(Debug, Clone, Copy)]
pub struct BiasSpectrAdvProps {
    /// Reset bias to initial value after sweep
    pub reset_bias: bool,
    /// Hold Z-controller during sweep
    pub z_controller_hold: bool,
    /// Record final Z position
    pub record_final_z: bool,
    /// Run lock-in during measurement
    pub lockin_run: bool,
}

impl Default for BiasSpectrAdvProps {
    fn default() -> Self {
        Self {
            reset_bias: true,
            z_controller_hold: true,
            record_final_z: false,
            lockin_run: false,
        }
    }
}

/// Builder for configuring bias spectroscopy properties.
///
/// All fields default to `NoChange`, meaning only the fields you explicitly
/// set will be modified on the instrument.
///
/// # Examples
/// ```no_run
/// use nanonis_rs::bias_spectr::{BiasSpectrPropsBuilder, OptionalFlag};
///
/// // Only change num_points and autosave
/// let config = BiasSpectrPropsBuilder::new()
///     .num_points(500)
///     .autosave(OptionalFlag::On)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct BiasSpectrPropsBuilder {
    /// Whether to save individual sweep data with average
    pub save_all: OptionalFlag,
    /// Number of sweeps (0 = no change)
    pub num_sweeps: i32,
    /// Whether to acquire backward sweep
    pub backward_sweep: OptionalFlag,
    /// Number of points (0 = no change)
    pub num_points: i32,
    /// Z offset in meters (0.0 = no change)
    pub z_offset_m: f32,
    /// Enable autosave
    pub autosave: OptionalFlag,
    /// Show save dialog
    pub show_save_dialog: OptionalFlag,
}

impl BiasSpectrPropsBuilder {
    /// Create a new builder with all defaults (no change).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to save all individual sweeps.
    pub fn save_all(mut self, flag: OptionalFlag) -> Self {
        self.save_all = flag;
        self
    }

    /// Set the number of sweeps to average.
    pub fn num_sweeps(mut self, count: i32) -> Self {
        self.num_sweeps = count;
        self
    }

    /// Set whether to acquire backward sweep.
    pub fn backward_sweep(mut self, flag: OptionalFlag) -> Self {
        self.backward_sweep = flag;
        self
    }

    /// Set the number of points in the sweep.
    pub fn num_points(mut self, count: i32) -> Self {
        self.num_points = count;
        self
    }

    /// Set the Z offset in meters.
    pub fn z_offset_m(mut self, offset: f32) -> Self {
        self.z_offset_m = offset;
        self
    }

    /// Set the autosave option.
    pub fn autosave(mut self, flag: OptionalFlag) -> Self {
        self.autosave = flag;
        self
    }

    /// Set the show save dialog option.
    pub fn show_save_dialog(mut self, flag: OptionalFlag) -> Self {
        self.show_save_dialog = flag;
        self
    }

    /// Build the configuration (returns self for use with props_set).
    pub fn build(self) -> Self {
        self
    }
}

/// TTL synchronization configuration.
#[derive(Debug, Clone)]
pub struct TTLSyncConfig {
    /// TTL line to use
    pub line: TTLLine,
    /// Polarity of switching
    pub polarity: TTLPolarity,
    /// Time to wait before activating
    pub time_to_on: Duration,
    /// Duration to keep activated
    pub on_duration: Duration,
}

impl Default for TTLSyncConfig {
    fn default() -> Self {
        Self {
            line: TTLLine::HSLine1,
            polarity: TTLPolarity::HighActive,
            time_to_on: Duration::ZERO,
            on_duration: Duration::from_millis(100),
        }
    }
}

/// Pulse sequence synchronization configuration.
#[derive(Debug, Clone, Copy)]
pub struct PulseSeqSyncConfig {
    /// Pulse sequence number
    pub sequence_nr: u16,
    /// Number of periods to execute
    pub num_periods: u32,
}

impl Default for PulseSeqSyncConfig {
    fn default() -> Self {
        Self {
            sequence_nr: 1,
            num_periods: 1,
        }
    }
}

/// Alternate Z-controller setpoint configuration.
#[derive(Debug, Clone)]
pub struct AltZCtrlConfig {
    /// Whether alternate setpoint is enabled
    pub enabled: bool,
    /// Alternate setpoint value
    pub setpoint: f32,
    /// Settling time after changing setpoint
    pub settling_time: Duration,
}

impl Default for AltZCtrlConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            setpoint: 0.0,
            settling_time: Duration::from_millis(100),
        }
    }
}

/// Result data from a bias spectroscopy measurement.
#[derive(Debug, Clone)]
pub struct BiasSpectrResult {
    /// Names of recorded channels
    pub channel_names: Vec<String>,
    /// 2D data array `[rows][columns]`
    pub data: Vec<Vec<f32>>,
    /// Measurement parameters
    pub parameters: Vec<f32>,
}

/// MLS (Multi-Line Segment) segment configuration.
#[derive(Debug, Clone)]
pub struct MLSSegment {
    /// Bias start value in volts
    pub bias_start: f32,
    /// Bias end value in volts
    pub bias_end: f32,
    /// Initial settling time
    pub initial_settling_time: Duration,
    /// Settling time per point
    pub settling_time: Duration,
    /// Integration time per point
    pub integration_time: Duration,
    /// Maximum slew rate in V/s
    pub max_slew_rate: f32,
    /// Number of steps in segment
    pub steps: i32,
}

impl Default for MLSSegment {
    fn default() -> Self {
        Self {
            bias_start: -1.0,
            bias_end: 1.0,
            initial_settling_time: Duration::from_millis(100),
            settling_time: Duration::from_millis(10),
            integration_time: Duration::from_millis(20),
            max_slew_rate: 1.0,
            steps: 100,
        }
    }
}
