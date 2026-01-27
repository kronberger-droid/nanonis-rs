use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// FFT window type for spectrum analyzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpectrumFFTWindow {
    #[default]
    None = 0,
    Hanning = 1,
    Hamming = 2,
    BlackmanHarris = 3,
    ExactBlackman = 4,
    Blackman = 5,
    FlatTop = 6,
    FourTermBHarris = 7,
    SevenTermBHarris = 8,
    LowSidelobe = 9,
}

impl From<SpectrumFFTWindow> for u16 {
    fn from(window: SpectrumFFTWindow) -> Self {
        window as u16
    }
}

impl TryFrom<u16> for SpectrumFFTWindow {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Hanning),
            2 => Ok(Self::Hamming),
            3 => Ok(Self::BlackmanHarris),
            4 => Ok(Self::ExactBlackman),
            5 => Ok(Self::Blackman),
            6 => Ok(Self::FlatTop),
            7 => Ok(Self::FourTermBHarris),
            8 => Ok(Self::SevenTermBHarris),
            9 => Ok(Self::LowSidelobe),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid FFT window: {}",
                value
            ))),
        }
    }
}

/// Averaging mode for spectrum analyzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpectrumAveragingMode {
    #[default]
    None = 0,
    Vector = 1,
    RMS = 2,
    PeakHold = 3,
}

impl From<SpectrumAveragingMode> for u16 {
    fn from(mode: SpectrumAveragingMode) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for SpectrumAveragingMode {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Vector),
            2 => Ok(Self::RMS),
            3 => Ok(Self::PeakHold),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid averaging mode: {}",
                value
            ))),
        }
    }
}

/// Weighting mode for spectrum analyzer averaging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpectrumWeightingMode {
    #[default]
    Linear = 0,
    Exponential = 1,
}

impl From<SpectrumWeightingMode> for u16 {
    fn from(mode: SpectrumWeightingMode) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for SpectrumWeightingMode {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Linear),
            1 => Ok(Self::Exponential),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid weighting mode: {}",
                value
            ))),
        }
    }
}

/// Cursor type for spectrum analyzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpectrumCursorType {
    #[default]
    XY = 0,
    DxDy = 1,
    X1X2Dx = 2,
    Y1Y2Dy = 3,
    RMSDf = 4,
    NoChange = 5,
}

impl From<SpectrumCursorType> for u16 {
    fn from(cursor: SpectrumCursorType) -> Self {
        cursor as u16
    }
}

impl TryFrom<u16> for SpectrumCursorType {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::XY),
            1 => Ok(Self::DxDy),
            2 => Ok(Self::X1X2Dx),
            3 => Ok(Self::Y1Y2Dy),
            4 => Ok(Self::RMSDf),
            5 => Ok(Self::NoChange),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid cursor type: {}",
                value
            ))),
        }
    }
}

/// Spectrum analyzer instance (1 or 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpectrumAnalyzerInstance {
    #[default]
    Analyzer1 = 1,
    Analyzer2 = 2,
}

impl From<SpectrumAnalyzerInstance> for i32 {
    fn from(instance: SpectrumAnalyzerInstance) -> Self {
        instance as i32
    }
}

/// Frequency range information from spectrum analyzer.
#[derive(Debug, Clone, Default)]
pub struct SpectrumFreqRange {
    /// Currently selected range index
    pub selected_index: i32,
    /// Available frequency ranges in Hz
    pub available_ranges_hz: Vec<f32>,
}

/// Frequency resolution information from spectrum analyzer.
#[derive(Debug, Clone, Default)]
pub struct SpectrumFreqResolution {
    /// Currently selected resolution index
    pub selected_index: u16,
    /// Available frequency resolutions in Hz
    pub available_resolutions_hz: Vec<f32>,
}

/// Averaging configuration for spectrum analyzer.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpectrumAveraging {
    /// Averaging mode
    pub mode: SpectrumAveragingMode,
    /// Weighting mode
    pub weighting: SpectrumWeightingMode,
    /// Number of averages
    pub count: u32,
}

/// Cursor position in spectrum analyzer.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpectrumCursorPos {
    /// X position of cursor 1 in Hz
    pub cursor1_x_hz: f64,
    /// X position of cursor 2 in Hz
    pub cursor2_x_hz: f64,
    /// Y position of cursor 1
    pub cursor1_y: f64,
}

/// Band RMS measurement result.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpectrumBandRMS {
    /// RMS value in the frequency band
    pub rms: f64,
    /// Minimum frequency of the band in Hz
    pub min_freq_hz: f64,
    /// Maximum frequency of the band in Hz
    pub max_freq_hz: f64,
}

/// Spectrum analyzer data.
#[derive(Debug, Clone, Default)]
pub struct SpectrumData {
    /// X coordinate of the first acquired point (Hz)
    pub f0_hz: f32,
    /// Frequency distance between two acquired points (Hz)
    pub df_hz: f32,
    /// Acquired spectrum data
    pub data: Vec<f32>,
}

impl NanonisClient {
    // ==================== Spectrum Analyzer ====================

    /// Set the channel to display in the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    /// * `channel_index` - Channel index (0-23) corresponding to signal slots
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_ch_set(
        &mut self,
        instance: SpectrumAnalyzerInstance,
        channel_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SpectrumAnlzr.ChSet",
            vec![
                NanonisValue::I32(instance.into()),
                NanonisValue::I32(channel_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the channel displayed in the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// Channel index (0-23) corresponding to signal slots.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_ch_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.ChGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["i"],
        )?;

        result[0].as_i32()
    }

    /// Set the frequency range in the spectrum analyzer.
    ///
    /// Use `spectrum_anlzr_freq_range_get` first to get available ranges.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    /// * `range_index` - Index of the desired frequency range
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_freq_range_set(
        &mut self,
        instance: SpectrumAnalyzerInstance,
        range_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SpectrumAnlzr.FreqRangeSet",
            vec![
                NanonisValue::I32(instance.into()),
                NanonisValue::I32(range_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the frequency range configuration from the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// Frequency range information including selected index and available ranges.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_freq_range_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<SpectrumFreqRange, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.FreqRangeGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["i", "i", "*f"],
        )?;

        Ok(SpectrumFreqRange {
            selected_index: result[0].as_i32()?,
            available_ranges_hz: result[2].as_f32_array()?.to_vec(),
        })
    }

    /// Set the frequency resolution in the spectrum analyzer.
    ///
    /// Use `spectrum_anlzr_freq_res_get` first to get available resolutions.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    /// * `resolution_index` - Index of the desired frequency resolution
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_freq_res_set(
        &mut self,
        instance: SpectrumAnalyzerInstance,
        resolution_index: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SpectrumAnlzr.FreqResSet",
            vec![
                NanonisValue::I32(instance.into()),
                NanonisValue::U16(resolution_index),
            ],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the frequency resolution configuration from the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// Frequency resolution information including selected index and available resolutions.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_freq_res_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<SpectrumFreqResolution, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.FreqResGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["H", "i", "*f"],
        )?;

        Ok(SpectrumFreqResolution {
            selected_index: result[0].as_u16()?,
            available_resolutions_hz: result[2].as_f32_array()?.to_vec(),
        })
    }

    /// Set the FFT window in the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    /// * `window` - FFT window type
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_fft_window_set(
        &mut self,
        instance: SpectrumAnalyzerInstance,
        window: SpectrumFFTWindow,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SpectrumAnlzr.FFTWindowSet",
            vec![
                NanonisValue::I32(instance.into()),
                NanonisValue::U16(window.into()),
            ],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the FFT window from the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// Currently selected FFT window type.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_fft_window_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<SpectrumFFTWindow, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.FFTWindowGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["H"],
        )?;

        SpectrumFFTWindow::try_from(result[0].as_u16()?)
    }

    /// Set the averaging parameters in the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    /// * `averaging` - Averaging configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_averag_set(
        &mut self,
        instance: SpectrumAnalyzerInstance,
        averaging: &SpectrumAveraging,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SpectrumAnlzr.AveragSet",
            vec![
                NanonisValue::I32(instance.into()),
                NanonisValue::U16(averaging.mode.into()),
                NanonisValue::U16(averaging.weighting.into()),
                NanonisValue::U32(averaging.count),
            ],
            vec!["i", "H", "H", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the averaging parameters from the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// Averaging configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_averag_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<SpectrumAveraging, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.AveragGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["H", "H", "I"],
        )?;

        Ok(SpectrumAveraging {
            mode: SpectrumAveragingMode::try_from(result[0].as_u16()?)?,
            weighting: SpectrumWeightingMode::try_from(result[1].as_u16()?)?,
            count: result[2].as_u32()?,
        })
    }

    /// Set the AC coupling mode in the spectrum analyzer.
    ///
    /// Use `spectrum_anlzr_dc_get` to get the DC component when AC coupling is enabled.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    /// * `enabled` - True to enable AC coupling, false to disable
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_ac_coupling_set(
        &mut self,
        instance: SpectrumAnalyzerInstance,
        enabled: bool,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SpectrumAnlzr.ACCouplingSet",
            vec![
                NanonisValue::I32(instance.into()),
                NanonisValue::U32(if enabled { 1 } else { 0 }),
            ],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the AC coupling mode from the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// True if AC coupling is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_ac_coupling_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<bool, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.ACCouplingGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["I"],
        )?;

        Ok(result[0].as_u32()? != 0)
    }

    /// Set the cursor positions in the spectrum analyzer.
    ///
    /// Cursors 1 and 2 are used to define the frequency band.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    /// * `cursor_type` - Type of cursor to display
    /// * `cursor1_x_hz` - X position of cursor 1 in Hz
    /// * `cursor2_x_hz` - X position of cursor 2 in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_cursor_pos_set(
        &mut self,
        instance: SpectrumAnalyzerInstance,
        cursor_type: SpectrumCursorType,
        cursor1_x_hz: f64,
        cursor2_x_hz: f64,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SpectrumAnlzr.CursorPosSet",
            vec![
                NanonisValue::I32(instance.into()),
                NanonisValue::U16(cursor_type.into()),
                NanonisValue::F64(cursor1_x_hz),
                NanonisValue::F64(cursor2_x_hz),
            ],
            vec!["i", "H", "d", "d"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the cursor positions from the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    /// * `cursor_type` - Type of cursor to display (also sets the cursor type)
    ///
    /// # Returns
    /// Cursor position information.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_cursor_pos_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
        cursor_type: SpectrumCursorType,
    ) -> Result<SpectrumCursorPos, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.CursorPosGet",
            vec![
                NanonisValue::I32(instance.into()),
                NanonisValue::U16(cursor_type.into()),
            ],
            vec!["i", "H"],
            vec!["d", "d", "d"],
        )?;

        Ok(SpectrumCursorPos {
            cursor1_x_hz: result[0].as_f64()?,
            cursor2_x_hz: result[1].as_f64()?,
            cursor1_y: result[2].as_f64()?,
        })
    }

    /// Get the RMS value in the frequency band from the spectrum analyzer.
    ///
    /// This function sets the cursor type to Band RMS if previously set to another type.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// Band RMS measurement including RMS value and frequency limits.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_band_rms_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<SpectrumBandRMS, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.BandRMSGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["d", "d", "d"],
        )?;

        Ok(SpectrumBandRMS {
            rms: result[0].as_f64()?,
            min_freq_hz: result[1].as_f64()?,
            max_freq_hz: result[2].as_f64()?,
        })
    }

    /// Get the DC value from the spectrum analyzer.
    ///
    /// Only returns meaningful values when AC coupling mode is enabled.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// DC value.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_dc_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<f64, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.DCGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["d"],
        )?;

        result[0].as_f64()
    }

    /// Start the spectrum analyzer.
    ///
    /// The spectrum analyzer does not run when its front panel is closed.
    /// Use this function to start the module for automated measurements.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_run(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SpectrumAnlzr.Run",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the data from the spectrum analyzer.
    ///
    /// # Arguments
    /// * `instance` - Spectrum analyzer instance (1 or 2)
    ///
    /// # Returns
    /// Spectrum data including frequency axis and amplitude values.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn spectrum_anlzr_data_get(
        &mut self,
        instance: SpectrumAnalyzerInstance,
    ) -> Result<SpectrumData, NanonisError> {
        let result = self.quick_send(
            "SpectrumAnlzr.DataGet",
            vec![NanonisValue::I32(instance.into())],
            vec!["i"],
            vec!["f", "f", "i", "*f"],
        )?;

        Ok(SpectrumData {
            f0_hz: result[0].as_f32()?,
            df_hz: result[1].as_f32()?,
            data: result[3].as_f32_array()?.to_vec(),
        })
    }
}
