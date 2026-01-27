// ==================== Piezo Types ====================

/// On/Off toggle with no-change option for piezo settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PiezoToggle {
    /// No change to current state
    #[default]
    NoChange = 0,
    /// Turn on
    On = 1,
    /// Turn off
    Off = 2,
}

impl From<PiezoToggle> for u32 {
    fn from(val: PiezoToggle) -> Self {
        val as u32
    }
}

impl From<PiezoToggle> for u16 {
    fn from(val: PiezoToggle) -> Self {
        val as u16
    }
}

/// XYZ tilt correction angles.
#[derive(Debug, Clone, Copy, Default)]
pub struct TiltCorrection {
    /// Tilt angle in X direction (degrees)
    pub tilt_x_deg: f32,
    /// Tilt angle in Y direction (degrees)
    pub tilt_y_deg: f32,
}

/// XYZ piezo range in meters.
#[derive(Debug, Clone, Copy, Default)]
pub struct PiezoRange {
    /// Range in X direction (meters)
    pub range_x_m: f32,
    /// Range in Y direction (meters)
    pub range_y_m: f32,
    /// Range in Z direction (meters)
    pub range_z_m: f32,
}

/// XYZ piezo sensitivity (m/V).
#[derive(Debug, Clone, Copy, Default)]
pub struct PiezoSensitivity {
    /// Sensitivity in X direction (m/V)
    pub sens_x_m_per_v: f32,
    /// Sensitivity in Y direction (m/V)
    pub sens_y_m_per_v: f32,
    /// Sensitivity in Z direction (m/V)
    pub sens_z_m_per_v: f32,
}

/// Drift compensation configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct DriftCompConfig {
    /// Compensation toggle (no change/on/off)
    pub enabled: PiezoToggle,
    /// Linear speed in X direction (m/s)
    pub vx_m_s: f32,
    /// Linear speed in Y direction (m/s)
    pub vy_m_s: f32,
    /// Linear speed in Z direction (m/s)
    pub vz_m_s: f32,
    /// Saturation limit
    pub saturation_limit: f32,
}

/// Drift compensation status and settings.
#[derive(Debug, Clone, Copy, Default)]
pub struct DriftCompStatus {
    /// Whether compensation is currently enabled
    pub enabled: bool,
    /// Linear speed in X direction (m/s)
    pub vx_m_s: f32,
    /// Linear speed in Y direction (m/s)
    pub vy_m_s: f32,
    /// Linear speed in Z direction (m/s)
    pub vz_m_s: f32,
    /// X axis reached saturation limit
    pub x_saturated: bool,
    /// Y axis reached saturation limit
    pub y_saturated: bool,
    /// Z axis reached saturation limit
    pub z_saturated: bool,
    /// Saturation limit value
    pub saturation_limit: f32,
}

/// HVA (High Voltage Amplifier) gain information.
#[derive(Debug, Clone, Copy, Default)]
pub struct HVAInfo {
    /// AUX gain
    pub gain_aux: f32,
    /// X gain
    pub gain_x: f32,
    /// Y gain
    pub gain_y: f32,
    /// Z gain
    pub gain_z: f32,
    /// XY enabled
    pub xy_enabled: bool,
    /// Z enabled
    pub z_enabled: bool,
    /// AUX enabled
    pub aux_enabled: bool,
}

/// HVA status LED indicators.
#[derive(Debug, Clone, Copy, Default)]
pub struct HVAStatusLED {
    /// Overheated status
    pub overheated: bool,
    /// HV supply status
    pub hv_supply: bool,
    /// High temperature status
    pub high_temperature: bool,
    /// Output connector status
    pub output_connector: bool,
}

/// XYZ voltage limits configuration.
#[derive(Debug, Clone, Copy)]
pub struct XYZLimits {
    /// Limits enabled
    pub enabled: bool,
    /// X low voltage limit (V)
    pub x_low_v: f32,
    /// X high voltage limit (V)
    pub x_high_v: f32,
    /// Y low voltage limit (V)
    pub y_low_v: f32,
    /// Y high voltage limit (V)
    pub y_high_v: f32,
    /// Z low voltage limit (V)
    pub z_low_v: f32,
    /// Z high voltage limit (V)
    pub z_high_v: f32,
}

impl Default for XYZLimits {
    fn default() -> Self {
        Self {
            enabled: false,
            x_low_v: -10.0,
            x_high_v: 10.0,
            y_low_v: -10.0,
            y_high_v: 10.0,
            z_low_v: -10.0,
            z_high_v: 10.0,
        }
    }
}

/// Hysteresis compensation points for one axis.
#[derive(Debug, Clone, Default)]
pub struct HysteresisAxisPoints {
    /// X coordinates of hysteresis points
    pub x_points: Vec<f32>,
    /// Y coordinates of hysteresis points
    pub y_points: Vec<f32>,
}

/// Hysteresis compensation values for fast and slow axes.
#[derive(Debug, Clone, Default)]
pub struct HysteresisValues {
    /// Fast axis hysteresis points
    pub fast_axis: HysteresisAxisPoints,
    /// Slow axis hysteresis points
    pub slow_axis: HysteresisAxisPoints,
}
