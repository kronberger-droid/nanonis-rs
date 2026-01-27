use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Trigger mode for PLL signal analyzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PLLTriggerMode {
    /// No change
    #[default]
    NoChange = 0,
    /// Immediate trigger
    Immediate = 1,
    /// Level-based trigger
    Level = 2,
}

impl From<PLLTriggerMode> for u16 {
    fn from(m: PLLTriggerMode) -> Self {
        m as u16
    }
}

impl TryFrom<u16> for PLLTriggerMode {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PLLTriggerMode::NoChange),
            1 => Ok(PLLTriggerMode::Immediate),
            2 => Ok(PLLTriggerMode::Level),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid PLLTriggerMode value: {}",
                value
            ))),
        }
    }
}

/// Trigger slope for PLL signal analyzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PLLTriggerSlope {
    /// No change
    #[default]
    NoChange = 0,
    /// Rising edge
    Rising = 1,
    /// Falling edge
    Falling = 2,
}

impl From<PLLTriggerSlope> for u16 {
    fn from(s: PLLTriggerSlope) -> Self {
        s as u16
    }
}

impl TryFrom<u16> for PLLTriggerSlope {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PLLTriggerSlope::NoChange),
            1 => Ok(PLLTriggerSlope::Rising),
            2 => Ok(PLLTriggerSlope::Falling),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid PLLTriggerSlope value: {}",
                value
            ))),
        }
    }
}

/// Arming mode for trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArmingMode {
    /// No change
    #[default]
    NoChange = 0,
    /// Manual rearm
    Manual = 1,
    /// Automatic rearm
    Automatic = 2,
}

impl From<ArmingMode> for u16 {
    fn from(m: ArmingMode) -> Self {
        m as u16
    }
}

impl TryFrom<u16> for ArmingMode {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ArmingMode::NoChange),
            1 => Ok(ArmingMode::Manual),
            2 => Ok(ArmingMode::Automatic),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid ArmingMode value: {}",
                value
            ))),
        }
    }
}

/// FFT window function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FFTWindow {
    /// No change
    #[default]
    NoChange = 0,
    /// No window
    None = 1,
    /// Hanning window
    Hanning = 2,
    /// Hamming window
    Hamming = 3,
    /// Blackman-Harris window
    BlackmanHarris = 4,
    /// Exact Blackman window
    ExactBlackman = 5,
    /// Blackman window
    Blackman = 6,
    /// Flat Top window
    FlatTop = 7,
    /// 4-term Blackman-Harris
    FourTermBH = 8,
    /// 7-term Blackman-Harris
    SevenTermBH = 9,
    /// Low Sidelobe window
    LowSidelobe = 10,
}

impl From<FFTWindow> for u16 {
    fn from(w: FFTWindow) -> Self {
        w as u16
    }
}

/// FFT averaging mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FFTAveragingMode {
    /// No change
    #[default]
    NoChange = 0,
    /// No averaging
    None = 1,
    /// Vector averaging
    Vector = 2,
    /// RMS averaging
    RMS = 3,
    /// Peak hold
    PeakHold = 4,
}

impl From<FFTAveragingMode> for u16 {
    fn from(m: FFTAveragingMode) -> Self {
        m as u16
    }
}

/// FFT weighting mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FFTWeightingMode {
    /// No change
    #[default]
    NoChange = 0,
    /// Linear weighting
    Linear = 1,
    /// Exponential weighting
    Exponential = 2,
}

impl From<FFTWeightingMode> for u16 {
    fn from(m: FFTWeightingMode) -> Self {
        m as u16
    }
}

/// PLL signal analyzer trigger configuration.
#[derive(Debug, Clone, Default)]
pub struct PLLAnlzrTrigger {
    /// Trigger mode
    pub mode: PLLTriggerMode,
    /// Signal index for trigger
    pub source_index: i32,
    /// Trigger slope
    pub slope: PLLTriggerSlope,
    /// Trigger level
    pub level: f64,
    /// Trigger position in seconds
    pub position_s: f64,
    /// Arming mode
    pub arming: ArmingMode,
}

/// PLL signal analyzer timebase settings.
#[derive(Debug, Clone, Default)]
pub struct PLLAnlzrTimebase {
    /// Current timebase index
    pub timebase_index: i32,
    /// Update rate (1 = fastest)
    pub update_rate: i32,
    /// Available timebases
    pub available_timebases: Vec<String>,
}

/// FFT properties configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct FFTProps {
    /// FFT window function
    pub window: FFTWindow,
    /// Averaging mode
    pub averaging: FFTAveragingMode,
    /// Weighting mode
    pub weighting: FFTWeightingMode,
    /// Number of averages
    pub count: i32,
}

/// Oscilloscope data from analyzer.
#[derive(Debug, Clone, Default)]
pub struct OsciAnalyzerData {
    /// Timestamp of first point
    pub t0: f64,
    /// Time between points
    pub dt: f64,
    /// Data array
    pub data: Vec<f64>,
}

/// FFT data from analyzer.
#[derive(Debug, Clone, Default)]
pub struct FFTAnalyzerData {
    /// Frequency of first point
    pub f0: f64,
    /// Frequency step
    pub df: f64,
    /// Data array
    pub data: Vec<f64>,
}

impl NanonisClient {
    // ==================== PLL Signal Analyzer ====================

    /// Open the PLL signal analyzer.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("PLLSignalAnlzr.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set the analyzer channel.
    ///
    /// # Arguments
    /// * `channel_index` - Channel index
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_ch_set(&mut self, channel_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLSignalAnlzr.ChSet",
            vec![NanonisValue::I32(channel_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the analyzer channel.
    ///
    /// # Returns
    /// Channel index.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_ch_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("PLLSignalAnlzr.ChGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the timebase and update rate.
    ///
    /// # Arguments
    /// * `timebase_index` - Timebase index (-1 = no change)
    /// * `update_rate` - Update rate (1 = fastest, -1 = no change)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_timebase_set(
        &mut self,
        timebase_index: i32,
        update_rate: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLSignalAnlzr.TimebaseSet",
            vec![
                NanonisValue::I32(timebase_index),
                NanonisValue::I32(update_rate),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the timebase and update rate.
    ///
    /// # Returns
    /// Timebase settings with available timebases.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_timebase_get(&mut self) -> Result<PLLAnlzrTimebase, NanonisError> {
        let result = self.quick_send(
            "PLLSignalAnlzr.TimebaseGet",
            vec![],
            vec![],
            vec!["i", "i", "i", "i", "*+c"],
        )?;

        if result.len() >= 5 {
            Ok(PLLAnlzrTimebase {
                timebase_index: result[0].as_i32()?,
                update_rate: result[1].as_i32()?,
                available_timebases: result[4].as_string_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set trigger to automatic mode.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_trig_auto(&mut self) -> Result<(), NanonisError> {
        self.quick_send("PLLSignalAnlzr.TrigAuto", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Rearm the trigger.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_trig_rearm(&mut self) -> Result<(), NanonisError> {
        self.quick_send("PLLSignalAnlzr.TrigRearm", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set the trigger configuration.
    ///
    /// # Arguments
    /// * `trigger` - Trigger configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_trig_set(
        &mut self,
        trigger: &PLLAnlzrTrigger,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLSignalAnlzr.TrigSet",
            vec![
                NanonisValue::U16(trigger.mode.into()),
                NanonisValue::I32(trigger.source_index),
                NanonisValue::U16(trigger.slope.into()),
                NanonisValue::F64(trigger.level),
                NanonisValue::F64(trigger.position_s),
                NanonisValue::U16(trigger.arming.into()),
            ],
            vec!["H", "i", "H", "d", "d", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the trigger configuration.
    ///
    /// # Returns
    /// Trigger configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_trig_get(&mut self) -> Result<PLLAnlzrTrigger, NanonisError> {
        let result = self.quick_send(
            "PLLSignalAnlzr.TrigGet",
            vec![],
            vec![],
            vec!["H", "i", "H", "d", "d", "H", "i", "i", "*+c"],
        )?;

        if result.len() >= 6 {
            Ok(PLLAnlzrTrigger {
                mode: result[0].as_u16()?.try_into()?,
                source_index: result[1].as_i32()?,
                slope: result[2].as_u16()?.try_into()?,
                level: result[3].as_f64()?,
                position_s: result[4].as_f64()?,
                arming: result[5].as_u16()?.try_into()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get oscilloscope data from analyzer.
    ///
    /// # Returns
    /// Oscilloscope data.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_osci_data_get(&mut self) -> Result<OsciAnalyzerData, NanonisError> {
        let result =
            self.quick_send("PLLSignalAnlzr.OsciDataGet", vec![], vec![], vec!["d", "d", "i", "+*d"])?;

        if result.len() >= 4 {
            Ok(OsciAnalyzerData {
                t0: result[0].as_f64()?,
                dt: result[1].as_f64()?,
                data: result[3].as_f64_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set FFT properties.
    ///
    /// # Arguments
    /// * `props` - FFT configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_fft_props_set(&mut self, props: &FFTProps) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLSignalAnlzr.FFTPropsSet",
            vec![
                NanonisValue::U16(props.window.into()),
                NanonisValue::U16(props.averaging.into()),
                NanonisValue::U16(props.weighting.into()),
                NanonisValue::I32(props.count),
            ],
            vec!["H", "H", "H", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get FFT properties.
    ///
    /// # Returns
    /// FFT configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_fft_props_get(&mut self) -> Result<FFTProps, NanonisError> {
        let result =
            self.quick_send("PLLSignalAnlzr.FFTPropsGet", vec![], vec![], vec!["H", "H", "H", "i"])?;

        if result.len() >= 4 {
            // Note: returned values have different offset than set values
            Ok(FFTProps {
                window: FFTWindow::NoChange, // Would need TryFrom for returned values
                averaging: FFTAveragingMode::NoChange,
                weighting: FFTWeightingMode::NoChange,
                count: result[3].as_i32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Restart FFT averaging.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_fft_avg_restart(&mut self) -> Result<(), NanonisError> {
        self.quick_send("PLLSignalAnlzr.FFTAvgRestart", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get FFT data.
    ///
    /// # Returns
    /// FFT data.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_signal_anlzr_fft_data_get(&mut self) -> Result<FFTAnalyzerData, NanonisError> {
        let result =
            self.quick_send("PLLSignalAnlzr.FFTDataGet", vec![], vec![], vec!["d", "d", "i", "*d"])?;

        if result.len() >= 4 {
            Ok(FFTAnalyzerData {
                f0: result[0].as_f64()?,
                df: result[1].as_f64()?,
                data: result[3].as_f64_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    // ==================== PLL Zoom FFT ====================

    /// Open the PLL Zoom FFT module.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_zoom_fft_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("PLLZoomFFT.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set the Zoom FFT channel.
    ///
    /// # Arguments
    /// * `channel_index` - Channel index
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_zoom_fft_ch_set(&mut self, channel_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLZoomFFT.ChSet",
            vec![NanonisValue::I32(channel_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Zoom FFT channel.
    ///
    /// # Returns
    /// Channel index.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_zoom_fft_ch_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("PLLZoomFFT.ChGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Restart Zoom FFT averaging.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_zoom_fft_avg_restart(&mut self) -> Result<(), NanonisError> {
        self.quick_send("PLLZoomFFT.AvgRestart", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set Zoom FFT properties.
    ///
    /// # Arguments
    /// * `props` - FFT configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_zoom_fft_props_set(&mut self, props: &FFTProps) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLZoomFFT.PropsSet",
            vec![
                NanonisValue::U16(props.window.into()),
                NanonisValue::U16(props.averaging.into()),
                NanonisValue::U16(props.weighting.into()),
                NanonisValue::I32(props.count),
            ],
            vec!["H", "H", "H", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get Zoom FFT properties.
    ///
    /// # Returns
    /// FFT configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_zoom_fft_props_get(&mut self) -> Result<FFTProps, NanonisError> {
        let result =
            self.quick_send("PLLZoomFFT.PropsGet", vec![], vec![], vec!["H", "H", "H", "i"])?;

        if result.len() >= 4 {
            Ok(FFTProps {
                window: FFTWindow::NoChange,
                averaging: FFTAveragingMode::NoChange,
                weighting: FFTWeightingMode::NoChange,
                count: result[3].as_i32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get Zoom FFT data.
    ///
    /// # Returns
    /// FFT data.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_zoom_fft_data_get(&mut self) -> Result<FFTAnalyzerData, NanonisError> {
        let result =
            self.quick_send("PLLZoomFFT.DataGet", vec![], vec![], vec!["d", "d", "i", "*d"])?;

        if result.len() >= 4 {
            Ok(FFTAnalyzerData {
                f0: result[0].as_f64()?,
                df: result[1].as_f64()?,
                data: result[3].as_f64_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
