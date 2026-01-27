use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Comparison condition for auto-reverse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReverseCondition {
    /// Signal greater than threshold
    #[default]
    GreaterThan = 0,
    /// Signal less than threshold
    LessThan = 1,
}

impl From<ReverseCondition> for i32 {
    fn from(c: ReverseCondition) -> Self {
        c as i32
    }
}

impl TryFrom<i32> for ReverseCondition {
    type Error = NanonisError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReverseCondition::GreaterThan),
            1 => Ok(ReverseCondition::LessThan),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid ReverseCondition value: {}",
                value
            ))),
        }
    }
}

/// Linkage mode for second auto-reverse condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConditionLinkage {
    /// No second condition
    #[default]
    Off = 0,
    /// Either condition 1 or 2 must be met
    Or = 1,
    /// Both conditions must be met simultaneously
    And = 2,
    /// Condition 1 must be met first, then condition 2
    Then = 3,
}

impl From<ConditionLinkage> for i32 {
    fn from(l: ConditionLinkage) -> Self {
        l as i32
    }
}

impl TryFrom<i32> for ConditionLinkage {
    type Error = NanonisError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ConditionLinkage::Off),
            1 => Ok(ConditionLinkage::Or),
            2 => Ok(ConditionLinkage::And),
            3 => Ok(ConditionLinkage::Then),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid ConditionLinkage value: {}",
                value
            ))),
        }
    }
}

/// Auto-reverse configuration for high-speed sweeper.
#[derive(Debug, Clone, Copy, Default)]
pub struct HSSwpAutoReverse {
    /// Enable auto-reverse
    pub enabled: bool,
    /// First condition comparison type
    pub condition: ReverseCondition,
    /// First condition signal index
    pub signal_index: i32,
    /// First condition threshold
    pub threshold: f32,
    /// Linkage to second condition
    pub linkage: ConditionLinkage,
    /// Second condition comparison type
    pub condition2: ReverseCondition,
    /// Second condition signal index
    pub signal2_index: i32,
    /// Second condition threshold
    pub threshold2: f32,
}

/// Sweep timing parameters.
#[derive(Debug, Clone, Copy, Default)]
pub struct HSSwpTiming {
    /// Initial settling time in seconds
    pub initial_settling_s: f32,
    /// Settling time in seconds
    pub settling_s: f32,
    /// Integration time in seconds
    pub integration_s: f32,
    /// Maximum slew rate in units/s
    pub max_slew_rate: f32,
}

/// Sweep channel limits configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct HSSwpLimits {
    /// Use relative limits (vs absolute)
    pub relative: bool,
    /// Start value
    pub start: f32,
    /// Stop value
    pub stop: f32,
}

/// Z-controller behavior during sweep.
#[derive(Debug, Clone, Copy, Default)]
pub struct HSSwpZCtrl {
    /// Whether to switch off Z-controller during sweep
    pub switch_off: bool,
    /// Z-controller index (1 = tip 1, 2-4 for multiprobe)
    pub controller_index: i32,
    /// Time to average Z position before switch-off
    pub averaging_time_s: f32,
    /// Z offset for tip retraction
    pub z_offset_m: f32,
    /// Time to wait after switching back on
    pub control_time_s: f32,
}

/// Available channels information.
#[derive(Debug, Clone, Default)]
pub struct HSSwpAvailableChannels {
    /// Currently selected channel indices
    pub selected_indices: Vec<u32>,
    /// Names of available channels
    pub available_names: Vec<String>,
    /// Indices of available channels
    pub available_indices: Vec<i32>,
}

/// Sweep signal information.
#[derive(Debug, Clone, Default)]
pub struct HSSwpSignalList {
    /// Signal names
    pub names: Vec<String>,
    /// Signal indices
    pub indices: Vec<i32>,
}

/// Save options configuration.
#[derive(Debug, Clone, Default)]
pub struct HSSwpSaveOptions {
    /// Comment for file header
    pub comment: String,
    /// Module names for parameters to save
    pub modules: Vec<String>,
}

impl NanonisClient {
    /// Set the acquisition channels for the high-speed sweeper.
    ///
    /// # Arguments
    /// * `channel_indices` - Indices of channels to record
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_acq_chs_set(&mut self, channel_indices: &[i32]) -> Result<(), NanonisError> {
        self.quick_send(
            "HSSwp.AcqChsSet",
            vec![NanonisValue::ArrayI32(channel_indices.to_vec())],
            vec!["+*i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the acquisition channels for the high-speed sweeper.
    ///
    /// # Returns
    /// An [`HSSwpAvailableChannels`] struct with channel information.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_acq_chs_get(&mut self) -> Result<HSSwpAvailableChannels, NanonisError> {
        let result = self.quick_send(
            "HSSwp.AcqChsGet",
            vec![],
            vec![],
            vec!["i", "*I", "i", "i", "*+c", "i", "*i"],
        )?;

        if result.len() >= 7 {
            Ok(HSSwpAvailableChannels {
                selected_indices: result[1].as_u32_array()?.to_vec(),
                available_names: result[4].as_string_array()?.to_vec(),
                available_indices: result[6].as_i32_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the auto-reverse configuration.
    ///
    /// # Arguments
    /// * `config` - Auto-reverse configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_auto_reverse_set(
        &mut self,
        config: &HSSwpAutoReverse,
    ) -> Result<(), NanonisError> {
        let on_off = if config.enabled { 1i32 } else { 0i32 };
        self.quick_send(
            "HSSwp.AutoReverseSet",
            vec![
                NanonisValue::I32(on_off),
                NanonisValue::I32(config.condition.into()),
                NanonisValue::I32(config.signal_index),
                NanonisValue::F32(config.threshold),
                NanonisValue::I32(config.linkage.into()),
                NanonisValue::I32(config.condition2.into()),
                NanonisValue::I32(config.signal2_index),
                NanonisValue::F32(config.threshold2),
            ],
            vec!["i", "i", "i", "f", "i", "i", "i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the auto-reverse configuration.
    ///
    /// # Returns
    /// An [`HSSwpAutoReverse`] struct with current configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_auto_reverse_get(&mut self) -> Result<HSSwpAutoReverse, NanonisError> {
        let result = self.quick_send(
            "HSSwp.AutoReverseGet",
            vec![],
            vec![],
            vec!["i", "i", "i", "f", "i", "i", "i", "f"],
        )?;

        if result.len() >= 8 {
            Ok(HSSwpAutoReverse {
                enabled: result[0].as_i32()? != 0,
                condition: result[1].as_i32()?.try_into()?,
                signal_index: result[2].as_i32()?,
                threshold: result[3].as_f32()?,
                linkage: result[4].as_i32()?.try_into()?,
                condition2: result[5].as_i32()?.try_into()?,
                signal2_index: result[6].as_i32()?,
                threshold2: result[7].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the end settling time.
    ///
    /// # Arguments
    /// * `time_s` - End settling time in seconds
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_end_settl_set(&mut self, time_s: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "HSSwp.EndSettlSet",
            vec![NanonisValue::F32(time_s)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the end settling time.
    ///
    /// # Returns
    /// End settling time in seconds.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_end_settl_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("HSSwp.EndSettlGet", vec![], vec![], vec!["f"])?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the number of sweeps.
    ///
    /// # Arguments
    /// * `num_sweeps` - Number of sweeps (ignored if continuous)
    /// * `continuous` - Enable continuous sweep mode
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_num_sweeps_set(
        &mut self,
        num_sweeps: u32,
        continuous: bool,
    ) -> Result<(), NanonisError> {
        let cont_flag = if continuous { 1i32 } else { 0i32 };
        self.quick_send(
            "HSSwp.NumSweepsSet",
            vec![NanonisValue::U32(num_sweeps), NanonisValue::I32(cont_flag)],
            vec!["I", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the number of sweeps.
    ///
    /// # Returns
    /// Tuple of (number of sweeps, continuous mode enabled).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_num_sweeps_get(&mut self) -> Result<(u32, bool), NanonisError> {
        let result = self.quick_send("HSSwp.NumSweepsGet", vec![], vec![], vec!["I", "i"])?;

        if result.len() >= 2 {
            Ok((result[0].as_u32()?, result[1].as_i32()? != 0))
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set whether signals are reset at sweep end.
    ///
    /// # Arguments
    /// * `reset` - True to reset signals at sweep end
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_reset_signals_set(&mut self, reset: bool) -> Result<(), NanonisError> {
        let flag = if reset { 1i32 } else { 0i32 };
        self.quick_send(
            "HSSwp.ResetSignalsSet",
            vec![NanonisValue::I32(flag)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get whether signals are reset at sweep end.
    ///
    /// # Returns
    /// True if signals are reset at sweep end.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_reset_signals_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("HSSwp.ResetSignalsGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the save basename and path.
    ///
    /// # Arguments
    /// * `basename` - Base name for saved files
    /// * `path` - Directory path for saved files
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_save_basename_set(
        &mut self,
        basename: &str,
        path: &str,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "HSSwp.SaveBasenameSet",
            vec![
                NanonisValue::String(basename.to_string()),
                NanonisValue::String(path.to_string()),
            ],
            vec!["+*c", "+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the save basename and path.
    ///
    /// # Returns
    /// Tuple of (basename, path).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_save_basename_get(&mut self) -> Result<(String, String), NanonisError> {
        let result =
            self.quick_send("HSSwp.SaveBasenameGet", vec![], vec![], vec!["i", "*-c", "*-c"])?;

        if result.len() >= 3 {
            Ok((
                result[1].as_string()?.to_string(),
                result[2].as_string()?.to_string(),
            ))
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set whether data is saved.
    ///
    /// # Arguments
    /// * `save` - True to save data
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_save_data_set(&mut self, save: bool) -> Result<(), NanonisError> {
        let flag = if save { 1i32 } else { 0i32 };
        self.quick_send(
            "HSSwp.SaveDataSet",
            vec![NanonisValue::I32(flag)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get whether data is saved.
    ///
    /// # Returns
    /// True if data is being saved.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_save_data_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("HSSwp.SaveDataGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set save options.
    ///
    /// # Arguments
    /// * `options` - Save options configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_save_options_set(&mut self, options: &HSSwpSaveOptions) -> Result<(), NanonisError> {
        self.quick_send(
            "HSSwp.SaveOptionsSet",
            vec![
                NanonisValue::String(options.comment.clone()),
                NanonisValue::ArrayString(options.modules.clone()),
            ],
            vec!["+*c", "+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Get save options.
    ///
    /// # Returns
    /// An [`HSSwpSaveOptions`] struct with current options.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_save_options_get(&mut self) -> Result<HSSwpSaveOptions, NanonisError> {
        let result = self.quick_send(
            "HSSwp.SaveOptionsGet",
            vec![],
            vec![],
            vec!["i", "*-c", "i", "i", "*+c"],
        )?;

        if result.len() >= 5 {
            Ok(HSSwpSaveOptions {
                comment: result[1].as_string()?.to_string(),
                modules: result[4].as_string_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Start a high-speed sweep.
    ///
    /// # Arguments
    /// * `wait_until_done` - Wait for sweep to complete before returning
    /// * `timeout_ms` - Timeout in milliseconds (-1 for indefinite)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_start(&mut self, wait_until_done: bool, timeout_ms: i32) -> Result<(), NanonisError> {
        let wait_flag = if wait_until_done { 1i32 } else { 0i32 };
        self.quick_send(
            "HSSwp.Start",
            vec![NanonisValue::I32(wait_flag), NanonisValue::I32(timeout_ms)],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Stop the high-speed sweep.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_stop(&mut self) -> Result<(), NanonisError> {
        self.quick_send("HSSwp.Stop", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get the sweep status.
    ///
    /// # Returns
    /// True if sweep is running.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_status_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("HSSwp.StatusGet", vec![], vec![], vec!["I"])?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get the list of available sweep signals.
    ///
    /// # Returns
    /// An [`HSSwpSignalList`] struct with signal information.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_sig_list_get(&mut self) -> Result<HSSwpSignalList, NanonisError> {
        let result =
            self.quick_send("HSSwp.SwpChSigListGet", vec![], vec![], vec!["+*c", "+*i"])?;

        if result.len() >= 2 {
            Ok(HSSwpSignalList {
                names: result[0].as_string_array()?.to_vec(),
                indices: result[1].as_i32_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the sweep channel signal.
    ///
    /// # Arguments
    /// * `signal_index` - Index of sweep signal
    /// * `timed_sweep` - Use timed sweep mode (ignores signal)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_signal_set(
        &mut self,
        signal_index: i32,
        timed_sweep: bool,
    ) -> Result<(), NanonisError> {
        let timed_flag = if timed_sweep { 1i32 } else { 0i32 };
        self.quick_send(
            "HSSwp.SwpChSignalSet",
            vec![NanonisValue::I32(signal_index), NanonisValue::I32(timed_flag)],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the sweep channel signal.
    ///
    /// # Returns
    /// Tuple of (signal index, timed sweep enabled).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_signal_get(&mut self) -> Result<(i32, bool), NanonisError> {
        let result = self.quick_send("HSSwp.SwpChSignalGet", vec![], vec![], vec!["i", "i"])?;

        if result.len() >= 2 {
            Ok((result[0].as_i32()?, result[1].as_i32()? != 0))
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the sweep channel limits.
    ///
    /// # Arguments
    /// * `limits` - Limits configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_limits_set(&mut self, limits: &HSSwpLimits) -> Result<(), NanonisError> {
        let rel_flag = if limits.relative { 1i32 } else { 0i32 };
        self.quick_send(
            "HSSwp.SwpChLimitsSet",
            vec![
                NanonisValue::I32(rel_flag),
                NanonisValue::F32(limits.start),
                NanonisValue::F32(limits.stop),
            ],
            vec!["i", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the sweep channel limits.
    ///
    /// # Returns
    /// An [`HSSwpLimits`] struct with current limits.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_limits_get(&mut self) -> Result<HSSwpLimits, NanonisError> {
        let result = self.quick_send("HSSwp.SwpChLimitsGet", vec![], vec![], vec!["i", "f", "f"])?;

        if result.len() >= 3 {
            Ok(HSSwpLimits {
                relative: result[0].as_i32()? != 0,
                start: result[1].as_f32()?,
                stop: result[2].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the number of sweep points.
    ///
    /// # Arguments
    /// * `num_points` - Number of points in sweep
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_num_pts_set(&mut self, num_points: u32) -> Result<(), NanonisError> {
        self.quick_send(
            "HSSwp.SwpChNumPtsSet",
            vec![NanonisValue::U32(num_points)],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the number of sweep points.
    ///
    /// # Returns
    /// Number of points in sweep.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_num_pts_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("HSSwp.SwpChNumPtsGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the sweep timing parameters.
    ///
    /// # Arguments
    /// * `timing` - Timing configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_timing_set(&mut self, timing: &HSSwpTiming) -> Result<(), NanonisError> {
        self.quick_send(
            "HSSwp.SwpChTimingSet",
            vec![
                NanonisValue::F32(timing.initial_settling_s),
                NanonisValue::F32(timing.settling_s),
                NanonisValue::F32(timing.integration_s),
                NanonisValue::F32(timing.max_slew_rate),
            ],
            vec!["f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the sweep timing parameters.
    ///
    /// # Returns
    /// An [`HSSwpTiming`] struct with current timing.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_timing_get(&mut self) -> Result<HSSwpTiming, NanonisError> {
        let result =
            self.quick_send("HSSwp.SwpChTimingGet", vec![], vec![], vec!["f", "f", "f", "f"])?;

        if result.len() >= 4 {
            Ok(HSSwpTiming {
                initial_settling_s: result[0].as_f32()?,
                settling_s: result[1].as_f32()?,
                integration_s: result[2].as_f32()?,
                max_slew_rate: result[3].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set whether backward sweep is enabled.
    ///
    /// # Arguments
    /// * `enabled` - Enable backward sweep
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_bwd_sw_set(&mut self, enabled: bool) -> Result<(), NanonisError> {
        let flag = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "HSSwp.SwpChBwdSwSet",
            vec![NanonisValue::U32(flag)],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get whether backward sweep is enabled.
    ///
    /// # Returns
    /// True if backward sweep is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_bwd_sw_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("HSSwp.SwpChBwdSwGet", vec![], vec![], vec!["I"])?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the backward sweep delay.
    ///
    /// # Arguments
    /// * `delay_s` - Delay between forward and backward sweep in seconds
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_bwd_delay_set(&mut self, delay_s: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "HSSwp.SwpChBwdDelaySet",
            vec![NanonisValue::F32(delay_s)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the backward sweep delay.
    ///
    /// # Returns
    /// Delay in seconds.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_swp_ch_bwd_delay_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("HSSwp.SwpChBwdDelayGet", vec![], vec![], vec!["f"])?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Z-controller behavior during sweep.
    ///
    /// # Arguments
    /// * `config` - Z-controller configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_z_ctrl_off_set(&mut self, config: &HSSwpZCtrl) -> Result<(), NanonisError> {
        let switch_off = if config.switch_off { 0i32 } else { 1i32 }; // 0=switch off, 1=don't switch
        self.quick_send(
            "HSSwp.ZCtrlOffSet",
            vec![
                NanonisValue::I32(switch_off),
                NanonisValue::I32(config.controller_index),
                NanonisValue::F32(config.averaging_time_s),
                NanonisValue::F32(config.z_offset_m),
                NanonisValue::F32(config.control_time_s),
            ],
            vec!["i", "i", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Z-controller behavior during sweep.
    ///
    /// # Returns
    /// An [`HSSwpZCtrl`] struct with current configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn hs_swp_z_ctrl_off_get(&mut self) -> Result<HSSwpZCtrl, NanonisError> {
        let result =
            self.quick_send("HSSwp.ZCtrlOffGet", vec![], vec![], vec!["i", "i", "f", "f", "f"])?;

        if result.len() >= 5 {
            Ok(HSSwpZCtrl {
                switch_off: result[0].as_i32()? == 0, // 0=switch off, 1=don't switch
                controller_index: result[1].as_i32()?,
                averaging_time_s: result[2].as_f32()?,
                z_offset_m: result[3].as_f32()?,
                control_time_s: result[4].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
