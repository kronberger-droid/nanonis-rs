use crate::error::NanonisError;

// ==================== Oscilloscope Types ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OscilloscopeIndex(pub i32);

impl From<OscilloscopeIndex> for i32 {
    fn from(osci: OscilloscopeIndex) -> Self {
        osci.0
    }
}

impl From<i32> for OscilloscopeIndex {
    fn from(index: i32) -> Self {
        OscilloscopeIndex(index)
    }
}

impl From<usize> for OscilloscopeIndex {
    fn from(index: usize) -> Self {
        OscilloscopeIndex(index as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerMode {
    Immediate = 0,
    Level = 1,
    Digital = 2,
}

impl From<TriggerMode> for u16 {
    fn from(mode: TriggerMode) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for TriggerMode {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TriggerMode::Immediate),
            1 => Ok(TriggerMode::Level),
            2 => Ok(TriggerMode::Digital),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid trigger mode: {}",
                value
            ))),
        }
    }
}

impl TryFrom<i32> for TriggerMode {
    type Error = NanonisError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u16::try_from(value)
            .map_err(|_| NanonisError::Protocol(format!("Invalid trigger mode: {}", value)))?;
        TriggerMode::try_from(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerSlope {
    Falling = 0,
    Rising = 1,
}

impl From<TriggerSlope> for u16 {
    fn from(slope: TriggerSlope) -> Self {
        slope as u16
    }
}

impl TryFrom<u16> for TriggerSlope {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TriggerSlope::Falling),
            1 => Ok(TriggerSlope::Rising),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid trigger slope: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TriggerLevel(pub f64);

impl From<TriggerLevel> for f64 {
    fn from(level: TriggerLevel) -> Self {
        level.0
    }
}

impl From<f64> for TriggerLevel {
    fn from(level: f64) -> Self {
        TriggerLevel(level)
    }
}

impl From<f32> for TriggerLevel {
    fn from(level: f32) -> Self {
        TriggerLevel(level as f64)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampleCount(pub i32);

impl SampleCount {
    pub fn new(count: i32) -> Self {
        Self(count)
    }
}

impl From<SampleCount> for i32 {
    fn from(samples: SampleCount) -> Self {
        samples.0
    }
}

impl From<i32> for SampleCount {
    fn from(count: i32) -> Self {
        SampleCount(count)
    }
}

impl From<u32> for SampleCount {
    fn from(count: u32) -> Self {
        SampleCount(count as i32)
    }
}

impl From<usize> for SampleCount {
    fn from(count: usize) -> Self {
        SampleCount(count as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsciTriggerMode {
    Immediate = 0,
    Level = 1,
    Auto = 2,
}

impl From<OsciTriggerMode> for u16 {
    fn from(mode: OsciTriggerMode) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for OsciTriggerMode {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OsciTriggerMode::Immediate),
            1 => Ok(OsciTriggerMode::Level),
            2 => Ok(OsciTriggerMode::Auto),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid oscilloscope trigger mode: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OversamplingIndex {
    Samples50 = 0,
    Samples20 = 1,
    Samples10 = 2,
    Samples5 = 3,
    Samples2 = 4,
    Samples1 = 5,
}

impl From<OversamplingIndex> for u16 {
    fn from(index: OversamplingIndex) -> Self {
        index as u16
    }
}

impl TryFrom<u16> for OversamplingIndex {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OversamplingIndex::Samples50),
            1 => Ok(OversamplingIndex::Samples20),
            2 => Ok(OversamplingIndex::Samples10),
            3 => Ok(OversamplingIndex::Samples5),
            4 => Ok(OversamplingIndex::Samples2),
            5 => Ok(OversamplingIndex::Samples1),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid oversampling index: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimebaseIndex(pub i32);

impl From<TimebaseIndex> for i32 {
    fn from(index: TimebaseIndex) -> Self {
        index.0
    }
}

impl From<TimebaseIndex> for u16 {
    fn from(index: TimebaseIndex) -> Self {
        index.0 as u16
    }
}

impl From<i32> for TimebaseIndex {
    fn from(value: i32) -> Self {
        TimebaseIndex(value)
    }
}

impl From<u16> for TimebaseIndex {
    fn from(value: u16) -> Self {
        TimebaseIndex(value as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataToGet {
    Current,
    NextTrigger,
    Wait2Triggers,
}

#[derive(Debug, Clone, Copy)]
pub struct TriggerConfig {
    pub mode: OsciTriggerMode,
    pub slope: TriggerSlope,
    pub level: f64,
    pub hysteresis: f64,
}

impl TriggerConfig {
    pub fn new(mode: OsciTriggerMode, slope: TriggerSlope, level: f64, hysteresis: f64) -> Self {
        Self {
            mode,
            slope,
            level,
            hysteresis,
        }
    }

    pub fn immediate() -> Self {
        Self {
            mode: OsciTriggerMode::Immediate,
            slope: TriggerSlope::Rising,
            level: 0.0,
            hysteresis: 0.0,
        }
    }

    pub fn level_trigger(level: f64, slope: TriggerSlope) -> Self {
        Self {
            mode: OsciTriggerMode::Level,
            slope,
            level,
            hysteresis: 0.1,
        }
    }

    pub fn auto_trigger() -> Self {
        Self {
            mode: OsciTriggerMode::Auto,
            slope: TriggerSlope::Rising,
            level: 0.0,
            hysteresis: 0.1,
        }
    }
}

/// Raw oscilloscope acquisition data.
///
/// Contains the time base and sampled data points returned by the
/// Nanonis oscilloscope commands. No application-level analysis
/// fields — stability checking belongs in downstream crates.
#[derive(Debug, Clone)]
pub struct OsciData {
    pub t0: f64,
    pub dt: f64,
    pub size: i32,
    pub data: Vec<f64>,
}

impl OsciData {
    pub fn new(t0: f64, dt: f64, size: i32, data: Vec<f64>) -> Self {
        Self { t0, dt, size, data }
    }

    pub fn values(&self) -> &[f64] {
        &self.data
    }

    pub fn time_series(&self) -> Vec<(f64, f64)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, &value)| (self.t0 + i as f64 * self.dt, value))
            .collect()
    }

    pub fn duration(&self) -> f64 {
        (self.size - 1) as f64 * self.dt
    }

    pub fn sample_rate(&self) -> f64 {
        if self.dt > 0.0 { 1.0 / self.dt } else { 0.0 }
    }

    pub fn time_points(&self) -> Vec<f64> {
        (0..self.size)
            .map(|i| self.t0 + i as f64 * self.dt)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- TriggerMode (fallible TryFrom) ----

    #[test]
    fn trigger_mode_roundtrip() {
        assert_eq!(TriggerMode::try_from(0u16).unwrap(), TriggerMode::Immediate);
        assert_eq!(TriggerMode::try_from(1u16).unwrap(), TriggerMode::Level);
        assert_eq!(TriggerMode::try_from(2u16).unwrap(), TriggerMode::Digital);
        assert!(TriggerMode::try_from(99u16).is_err());
    }

    #[test]
    fn trigger_mode_to_u16() {
        assert_eq!(u16::from(TriggerMode::Immediate), 0);
        assert_eq!(u16::from(TriggerMode::Level), 1);
        assert_eq!(u16::from(TriggerMode::Digital), 2);
    }

    #[test]
    fn trigger_mode_from_i32() {
        assert_eq!(TriggerMode::try_from(0i32).unwrap(), TriggerMode::Immediate);
        assert_eq!(TriggerMode::try_from(1i32).unwrap(), TriggerMode::Level);
        assert!(TriggerMode::try_from(-1i32).is_err());
    }

    // ---- TriggerSlope ----

    #[test]
    fn trigger_slope_roundtrip() {
        assert_eq!(u16::from(TriggerSlope::Falling), 0);
        assert_eq!(u16::from(TriggerSlope::Rising), 1);
        assert_eq!(TriggerSlope::try_from(0u16).unwrap(), TriggerSlope::Falling);
        assert_eq!(TriggerSlope::try_from(1u16).unwrap(), TriggerSlope::Rising);
        assert!(TriggerSlope::try_from(2u16).is_err());
    }

    // ---- OsciTriggerMode ----

    #[test]
    fn osci_trigger_mode_roundtrip() {
        for (mode, val) in [
            (OsciTriggerMode::Immediate, 0),
            (OsciTriggerMode::Level, 1),
            (OsciTriggerMode::Auto, 2),
        ] {
            assert_eq!(u16::from(mode), val);
            assert_eq!(OsciTriggerMode::try_from(val).unwrap(), mode);
        }
        assert!(OsciTriggerMode::try_from(3u16).is_err());
    }

    // ---- OversamplingIndex ----

    #[test]
    fn oversampling_index_roundtrip() {
        for val in 0..6u16 {
            let idx = OversamplingIndex::try_from(val).unwrap();
            assert_eq!(u16::from(idx), val);
        }
        assert!(OversamplingIndex::try_from(6u16).is_err());
    }

    // ---- TriggerConfig constructors ----

    #[test]
    fn trigger_config_immediate() {
        let tc = TriggerConfig::immediate();
        assert_eq!(tc.mode, OsciTriggerMode::Immediate);
        assert_eq!(tc.level, 0.0);
    }

    #[test]
    fn trigger_config_level() {
        let tc = TriggerConfig::level_trigger(1.5, TriggerSlope::Rising);
        assert_eq!(tc.mode, OsciTriggerMode::Level);
        assert_eq!(tc.level, 1.5);
        assert_eq!(tc.slope, TriggerSlope::Rising);
    }

    #[test]
    fn trigger_config_auto() {
        let tc = TriggerConfig::auto_trigger();
        assert_eq!(tc.mode, OsciTriggerMode::Auto);
    }

    // ---- OsciData time methods ----

    #[test]
    fn osci_data_time_series() {
        let data = OsciData::new(0.0, 0.001, 3, vec![10.0, 20.0, 30.0]);
        let ts = data.time_series();
        assert_eq!(ts.len(), 3);
        assert!((ts[0].0 - 0.0).abs() < 1e-15);
        assert!((ts[1].0 - 0.001).abs() < 1e-15);
        assert!((ts[2].0 - 0.002).abs() < 1e-15);
        assert_eq!(ts[0].1, 10.0);
        assert_eq!(ts[2].1, 30.0);
    }

    #[test]
    fn osci_data_time_series_with_offset() {
        let data = OsciData::new(1.0, 0.5, 2, vec![5.0, 6.0]);
        let ts = data.time_series();
        assert!((ts[0].0 - 1.0).abs() < 1e-15);
        assert!((ts[1].0 - 1.5).abs() < 1e-15);
    }

    #[test]
    fn osci_data_duration() {
        let data = OsciData::new(0.0, 0.01, 101, vec![]);
        assert!((data.duration() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn osci_data_sample_rate() {
        let data = OsciData::new(0.0, 0.001, 100, vec![]);
        assert!((data.sample_rate() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn osci_data_sample_rate_zero_dt() {
        let data = OsciData::new(0.0, 0.0, 100, vec![]);
        assert_eq!(data.sample_rate(), 0.0);
    }

    #[test]
    fn osci_data_time_points() {
        let data = OsciData::new(0.5, 0.1, 3, vec![]);
        let tp = data.time_points();
        assert_eq!(tp.len(), 3);
        assert!((tp[0] - 0.5).abs() < 1e-15);
        assert!((tp[1] - 0.6).abs() < 1e-15);
        assert!((tp[2] - 0.7).abs() < 1e-15);
    }

    #[test]
    fn osci_data_empty() {
        let data = OsciData::new(0.0, 0.01, 0, vec![]);
        assert!(data.time_series().is_empty());
        assert!(data.time_points().is_empty());
    }
}
