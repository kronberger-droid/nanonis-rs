use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// PLL frequency sweep parameters.
#[derive(Debug, Clone, Copy, Default)]
pub struct PLLFreqSwpParams {
    /// Number of points in sweep
    pub num_points: i32,
    /// Measurement/wait period in seconds
    pub period_s: f32,
    /// Initial settling time in seconds
    pub settling_time_s: f32,
}

/// PLL frequency sweep characteristic values.
#[derive(Debug, Clone, Copy, Default)]
pub struct PLLFreqSwpCharacteristics {
    /// Resonance frequency in Hz
    pub resonance_freq_hz: f64,
    /// Quality factor
    pub q_factor: f64,
    /// Phase at resonance in degrees
    pub phase_deg: f32,
    /// Amplitude to excitation ratio in nm/mV
    pub amp_exc_ratio_nm_per_mv: f32,
    /// Fit length (samples)
    pub fit_length: i32,
    /// Number of points
    pub num_points: i32,
}

/// PLL frequency sweep result data.
#[derive(Debug, Clone, Default)]
pub struct PLLFreqSwpData {
    /// Channel names
    pub channel_names: Vec<String>,
    /// Data rows (one per point)
    pub data: Vec<Vec<f32>>,
    /// Sweep characteristics
    pub characteristics: PLLFreqSwpCharacteristics,
}

/// PLL phase sweep result data.
#[derive(Debug, Clone, Default)]
pub struct PLLPhasSwpData {
    /// Channel names
    pub channel_names: Vec<String>,
    /// Data rows (one per point)
    pub data: Vec<Vec<f32>>,
}

impl NanonisClient {
    // ==================== PLL Frequency Sweep ====================

    /// Open the PLL frequency sweep module.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_swp_open(&mut self, modulator_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLFreqSwp.Open",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Set the PLL frequency sweep parameters.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `params` - Sweep parameters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_swp_params_set(
        &mut self,
        modulator_index: i32,
        params: &PLLFreqSwpParams,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLFreqSwp.ParamsSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::I32(params.num_points),
                NanonisValue::F32(params.period_s),
                NanonisValue::F32(params.settling_time_s),
            ],
            vec!["i", "i", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the PLL frequency sweep parameters.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Sweep parameters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_swp_params_get(
        &mut self,
        modulator_index: i32,
    ) -> Result<PLLFreqSwpParams, NanonisError> {
        let result = self.quick_send(
            "PLLFreqSwp.ParamsGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["i", "f", "f"],
        )?;

        if result.len() >= 3 {
            Ok(PLLFreqSwpParams {
                num_points: result[0].as_i32()?,
                period_s: result[1].as_f32()?,
                settling_time_s: result[2].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Start a PLL frequency sweep.
    ///
    /// Set center frequency and frequency range in Oscillation Control module first.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `get_data` - If true, return recorded channels and data
    /// * `sweep_up` - If true, sweep from lower to upper limit
    ///
    /// # Returns
    /// Sweep data and characteristics if `get_data` is true.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_swp_start(
        &mut self,
        modulator_index: i32,
        get_data: bool,
        sweep_up: bool,
    ) -> Result<Option<PLLFreqSwpData>, NanonisError> {
        let get_flag = if get_data { 1u32 } else { 0u32 };
        let dir_flag = if sweep_up { 1u32 } else { 0u32 };

        let result = self.quick_send(
            "PLLFreqSwp.Start",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::U32(get_flag),
                NanonisValue::U32(dir_flag),
            ],
            vec!["i", "I", "I"],
            vec!["i", "i", "*+c", "i", "i", "2f", "d", "d", "f", "f", "i", "i"],
        )?;

        if get_data && result.len() >= 12 {
            let channel_names = result[2].as_string_array()?.to_vec();
            let data = result[5].as_f32_2d_array()?.to_vec();

            Ok(Some(PLLFreqSwpData {
                channel_names,
                data,
                characteristics: PLLFreqSwpCharacteristics {
                    resonance_freq_hz: result[6].as_f64()?,
                    q_factor: result[7].as_f64()?,
                    phase_deg: result[8].as_f32()?,
                    amp_exc_ratio_nm_per_mv: result[9].as_f32()?,
                    fit_length: result[10].as_i32()?,
                    num_points: result[11].as_i32()?,
                },
            }))
        } else {
            Ok(None)
        }
    }

    /// Stop the PLL frequency sweep.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_swp_stop(&mut self, modulator_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLFreqSwp.Stop",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    // ==================== PLL Phase Sweep ====================

    /// Start a PLL phase sweep.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `get_data` - If true, return recorded channels and data
    ///
    /// # Returns
    /// Sweep data if `get_data` is true.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_phas_swp_start(
        &mut self,
        modulator_index: i32,
        get_data: bool,
    ) -> Result<Option<PLLPhasSwpData>, NanonisError> {
        let get_flag = if get_data { 1u32 } else { 0u32 };

        let result = self.quick_send(
            "PLLPhasSwp.Start",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::U32(get_flag),
            ],
            vec!["i", "I"],
            vec!["i", "i", "*+c", "i", "i", "2f"],
        )?;

        if get_data && result.len() >= 6 {
            let channel_names = result[2].as_string_array()?.to_vec();
            let data = result[5].as_f32_2d_array()?.to_vec();

            Ok(Some(PLLPhasSwpData {
                channel_names,
                data,
            }))
        } else {
            Ok(None)
        }
    }

    /// Stop the PLL phase sweep.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_phas_swp_stop(&mut self, modulator_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "PLLPhasSwp.Stop",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }
}
