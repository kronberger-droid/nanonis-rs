use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Lock-In frequency sweep properties configuration.
#[derive(Debug, Clone)]
pub struct LockInFreqSwpProps {
    /// Number of frequency steps (logarithmic distribution)
    pub num_steps: u16,
    /// Number of lock-in periods to average per measurement
    pub integration_periods: u16,
    /// Minimum integration time in seconds
    pub min_integration_time_s: f32,
    /// Number of lock-in periods to wait before acquiring
    pub settling_periods: u16,
    /// Minimum settling time in seconds
    pub min_settling_time_s: f32,
    /// Automatically save data at end of sweep
    pub autosave: bool,
    /// Show save dialog when saving
    pub save_dialog: bool,
    /// Base filename for saved files
    pub basename: String,
}

impl Default for LockInFreqSwpProps {
    fn default() -> Self {
        Self {
            num_steps: 100,
            integration_periods: 10,
            min_integration_time_s: 0.1,
            settling_periods: 5,
            min_settling_time_s: 0.05,
            autosave: true,
            save_dialog: false,
            basename: String::new(),
        }
    }
}

/// Result data from a lock-in frequency sweep measurement.
#[derive(Debug, Clone)]
pub struct LockInFreqSwpResult {
    /// Names of recorded channels
    pub channel_names: Vec<String>,
    /// 2D data array `[rows][columns]`
    /// First row is swept frequency, additional rows are channel data
    pub data: Vec<Vec<f32>>,
}

/// Sweep direction for lock-in frequency sweep.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FreqSwpDirection {
    /// Sweep down (from upper limit to lower limit)
    Down = 0,
    /// Sweep up (from lower limit to upper limit)
    #[default]
    Up = 1,
}

impl From<FreqSwpDirection> for u32 {
    fn from(dir: FreqSwpDirection) -> Self {
        dir as u32
    }
}

impl NanonisClient {
    /// Open the Lock-In Frequency Sweep (Transfer Function) module.
    ///
    /// The transfer function does not run when its front panel is closed.
    /// To automate measurements it may be required to open the module first.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.lockin_freq_swp_open()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lockin_freq_swp_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("LockInFreqSwp.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Start a Lock-In frequency sweep.
    ///
    /// # Arguments
    /// * `get_data` - If true, returns measurement data
    /// * `direction` - Sweep direction (up or down)
    ///
    /// # Returns
    /// A [`LockInFreqSwpResult`] with channel names and 2D data.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::lockin_freq_swp::FreqSwpDirection;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let result = client.lockin_freq_swp_start(true, FreqSwpDirection::Up)?;
    /// println!("Channels: {:?}", result.channel_names);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lockin_freq_swp_start(
        &mut self,
        get_data: bool,
        direction: FreqSwpDirection,
    ) -> Result<LockInFreqSwpResult, NanonisError> {
        let get_data_flag = if get_data { 1u32 } else { 0u32 };

        let result = self.quick_send(
            "LockInFreqSwp.Start",
            vec![
                NanonisValue::U32(get_data_flag),
                NanonisValue::U32(direction.into()),
            ],
            vec!["I", "I"],
            vec!["i", "i", "*+c", "i", "i", "2f"],
        )?;

        if result.len() >= 6 {
            let channel_names = result[2].as_string_array()?.to_vec();
            let rows = result[3].as_i32()? as usize;
            let cols = result[4].as_i32()? as usize;

            let flat_data = result[5].as_f32_array()?;
            let mut data_2d = Vec::with_capacity(rows);
            for row in 0..rows {
                let start_idx = row * cols;
                let end_idx = start_idx + cols;
                if end_idx <= flat_data.len() {
                    data_2d.push(flat_data[start_idx..end_idx].to_vec());
                }
            }

            Ok(LockInFreqSwpResult {
                channel_names,
                data: data_2d,
            })
        } else {
            Ok(LockInFreqSwpResult {
                channel_names: vec![],
                data: vec![],
            })
        }
    }

    /// Set the sweep signal for the Lock-In frequency sweep module.
    ///
    /// # Arguments
    /// * `signal_index` - Sweep signal index, or -1 for no signal
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_freq_swp_signal_set(&mut self, signal_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "LockInFreqSwp.SignalSet",
            vec![NanonisValue::I32(signal_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the sweep signal for the Lock-In frequency sweep module.
    ///
    /// # Returns
    /// The sweep signal index, or -1 if no signal is selected.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_freq_swp_signal_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("LockInFreqSwp.SignalGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the frequency limits for the Lock-In frequency sweep.
    ///
    /// # Arguments
    /// * `lower_limit_hz` - Lower frequency limit in Hz
    /// * `upper_limit_hz` - Upper frequency limit in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_freq_swp_limits_set(
        &mut self,
        lower_limit_hz: f32,
        upper_limit_hz: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockInFreqSwp.LimitsSet",
            vec![
                NanonisValue::F32(lower_limit_hz),
                NanonisValue::F32(upper_limit_hz),
            ],
            vec!["f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the frequency limits for the Lock-In frequency sweep.
    ///
    /// # Returns
    /// A tuple of (lower_limit_hz, upper_limit_hz).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_freq_swp_limits_get(&mut self) -> Result<(f32, f32), NanonisError> {
        let result = self.quick_send("LockInFreqSwp.LimitsGet", vec![], vec![], vec!["f", "f"])?;

        if result.len() >= 2 {
            Ok((result[0].as_f32()?, result[1].as_f32()?))
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Lock-In frequency sweep properties.
    ///
    /// # Arguments
    /// * `props` - A [`LockInFreqSwpProps`] struct with configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::lockin_freq_swp::LockInFreqSwpProps;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let props = LockInFreqSwpProps {
    ///     num_steps: 200,
    ///     integration_periods: 20,
    ///     ..Default::default()
    /// };
    /// client.lockin_freq_swp_props_set(&props)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lockin_freq_swp_props_set(
        &mut self,
        props: &LockInFreqSwpProps,
    ) -> Result<(), NanonisError> {
        let autosave_flag = if props.autosave { 1u32 } else { 0u32 };
        let dialog_flag = if props.save_dialog { 1u32 } else { 0u32 };

        self.quick_send(
            "LockInFreqSwp.PropsSet",
            vec![
                NanonisValue::U16(props.num_steps),
                NanonisValue::U16(props.integration_periods),
                NanonisValue::F32(props.min_integration_time_s),
                NanonisValue::U16(props.settling_periods),
                NanonisValue::F32(props.min_settling_time_s),
                NanonisValue::U32(autosave_flag),
                NanonisValue::U32(dialog_flag),
                NanonisValue::String(props.basename.clone()),
            ],
            vec!["H", "H", "f", "H", "f", "I", "I", "+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Lock-In frequency sweep properties.
    ///
    /// # Returns
    /// A [`LockInFreqSwpProps`] struct with current configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_freq_swp_props_get(&mut self) -> Result<LockInFreqSwpProps, NanonisError> {
        let result = self.quick_send(
            "LockInFreqSwp.PropsGet",
            vec![],
            vec![],
            vec!["H", "H", "f", "H", "f", "I", "I", "i", "*-c"],
        )?;

        if result.len() >= 9 {
            Ok(LockInFreqSwpProps {
                num_steps: result[0].as_u16()?,
                integration_periods: result[1].as_u16()?,
                min_integration_time_s: result[2].as_f32()?,
                settling_periods: result[3].as_u16()?,
                min_settling_time_s: result[4].as_f32()?,
                autosave: result[5].as_u32()? != 0,
                save_dialog: result[6].as_u32()? != 0,
                basename: result[8].as_string()?.to_string(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
