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

impl From<u16> for TriggerMode {
    fn from(value: u16) -> Self {
        match value {
            0 => TriggerMode::Immediate,
            1 => TriggerMode::Level,
            2 => TriggerMode::Digital,
            _ => TriggerMode::Immediate,
        }
    }
}

impl From<i32> for TriggerMode {
    fn from(value: i32) -> Self {
        TriggerMode::from(value as u16)
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
            _ => Err(NanonisError::InvalidCommand(format!(
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
            _ => Err(NanonisError::InvalidCommand(format!(
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
            _ => Err(NanonisError::InvalidCommand(format!(
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

#[derive(Debug, Clone)]
pub struct SignalStats {
    pub mean: f64,
    pub std_dev: f64,
    pub relative_std: f64,
    pub window_size: usize,
    pub stability_method: String,
}

#[derive(Debug, Clone)]
pub struct OsciData {
    pub t0: f64,
    pub dt: f64,
    pub size: i32,
    pub data: Vec<f64>,
    pub signal_stats: Option<SignalStats>,
    pub is_stable: bool,
    pub fallback_value: Option<f64>,
}

impl OsciData {
    pub fn new(t0: f64, dt: f64, size: i32, data: Vec<f64>) -> Self {
        Self {
            t0,
            dt,
            size,
            data,
            signal_stats: None,
            is_stable: true,
            fallback_value: None,
        }
    }

    pub fn new_with_stats(t0: f64, dt: f64, size: i32, data: Vec<f64>, stats: SignalStats) -> Self {
        Self {
            t0,
            dt,
            size,
            data,
            signal_stats: Some(stats),
            is_stable: true,
            fallback_value: None,
        }
    }

    pub fn new_stable(t0: f64, dt: f64, size: i32, data: Vec<f64>) -> Self {
        Self {
            t0,
            dt,
            size,
            data,
            signal_stats: None,
            is_stable: true,
            fallback_value: None,
        }
    }

    pub fn new_unstable_with_fallback(
        t0: f64,
        dt: f64,
        size: i32,
        data: Vec<f64>,
        fallback: f64,
    ) -> Self {
        Self {
            t0,
            dt,
            size,
            data,
            signal_stats: None,
            is_stable: false,
            fallback_value: Some(fallback),
        }
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

    pub fn stats(&self) -> Option<&SignalStats> {
        self.signal_stats.as_ref()
    }

    pub fn is_stable(&self) -> bool {
        self.signal_stats.is_some()
    }

    pub fn duration(&self) -> f64 {
        (self.size - 1) as f64 * self.dt
    }

    pub fn sample_rate(&self) -> f64 {
        if self.dt > 0.0 {
            1.0 / self.dt
        } else {
            0.0
        }
    }

    pub fn time_points(&self) -> Vec<f64> {
        (0..self.size)
            .map(|i| self.t0 + i as f64 * self.dt)
            .collect()
    }
}
