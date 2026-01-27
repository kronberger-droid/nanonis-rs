use super::super::NanonisClient;
use super::*;
use crate::error::NanonisError;
use crate::types::NanonisValue;
use crate::client::signals::SignalIndex;

impl NanonisClient {
    /// Set the measured signal index of the selected channel from the Oscilloscope High Resolution
    pub fn osci_hr_ch_set(
        &mut self,
        osci_index: impl Into<OscilloscopeIndex>,
        signal: impl Into<SignalIndex>,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.ChSet",
            vec![
                NanonisValue::I32(osci_index.into().into()),
                NanonisValue::I32(signal.into().into()),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the measured signal index of the selected channel from the Oscilloscope High Resolution
    pub fn osci_hr_ch_get(
        &mut self,
        osci_index: impl Into<OscilloscopeIndex>,
    ) -> Result<SignalIndex, NanonisError> {
        let result = self.quick_send(
            "OsciHR.ChGet",
            vec![NanonisValue::I32(osci_index.into().into())],
            vec!["i"],
            vec!["i"],
        )?;
        match result.first() {
            Some(value) => Ok(SignalIndex::new(value.as_i32()? as u8)),
            None => Err(NanonisError::Protocol(
                "No signal index returned".to_string(),
            )),
        }
    }

    /// Set the oversampling index of the Oscilloscope High Resolution
    pub fn osci_hr_oversampl_set(&mut self, oversampling_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.OversamplSet",
            vec![NanonisValue::I32(oversampling_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the oversampling index of the Oscilloscope High Resolution
    pub fn osci_hr_oversampl_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("OsciHR.OversamplGet", vec![], vec![], vec!["i"])?;
        match result.first() {
            Some(value) => Ok(value.as_i32()?),
            None => Err(NanonisError::Protocol(
                "No oversampling index returned".to_string(),
            )),
        }
    }

    /// Set the calibration mode of the selected channel from the Oscilloscope High Resolution
    /// calibration_mode: 0 = Raw values, 1 = Calibrated values
    pub fn osci_hr_calibr_mode_set(
        &mut self,
        osci_index: i32,
        calibration_mode: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.CalibrModeSet",
            vec![
                NanonisValue::I32(osci_index),
                NanonisValue::U16(calibration_mode),
            ],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the calibration mode of the selected channel from the Oscilloscope High Resolution
    /// Returns: 0 = Raw values, 1 = Calibrated values
    pub fn osci_hr_calibr_mode_get(&mut self, osci_index: i32) -> Result<u16, NanonisError> {
        let result = self.quick_send(
            "OsciHR.CalibrModeGet",
            vec![NanonisValue::I32(osci_index)],
            vec!["i"],
            vec!["H"],
        )?;
        match result.first() {
            Some(value) => Ok(value.as_u16()?),
            None => Err(NanonisError::Protocol(
                "No calibration mode returned".to_string(),
            )),
        }
    }

    /// Set the number of samples to acquire in the Oscilloscope High Resolution
    pub fn osci_hr_samples_set(
        &mut self,
        number_of_samples: impl Into<SampleCount>,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.SamplesSet",
            vec![NanonisValue::I32(number_of_samples.into().into())],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the number of samples to acquire in the Oscilloscope High Resolution
    pub fn osci_hr_samples_get(&mut self) -> Result<SampleCount, NanonisError> {
        let result = self.quick_send("OsciHR.SamplesGet", vec![], vec![], vec!["i"])?;
        match result.first() {
            Some(value) => Ok(SampleCount::new(value.as_i32()?)),
            None => Err(NanonisError::Protocol(
                "No sample count returned".to_string(),
            )),
        }
    }

    /// Set the Pre-Trigger Samples or Seconds in the Oscilloscope High Resolution
    pub fn osci_hr_pre_trig_set(
        &mut self,
        pre_trigger_samples: u32,
        pre_trigger_s: f64,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.PreTrigSet",
            vec![
                NanonisValue::U32(pre_trigger_samples),
                NanonisValue::F64(pre_trigger_s),
            ],
            vec!["I", "d"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Pre-Trigger Samples in the Oscilloscope High Resolution
    pub fn osci_hr_pre_trig_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("OsciHR.PreTrigGet", vec![], vec![], vec!["i"])?;
        match result.first() {
            Some(value) => Ok(value.as_i32()?),
            None => Err(NanonisError::Protocol(
                "No pre-trigger samples returned".to_string(),
            )),
        }
    }

    /// Start the Oscilloscope High Resolution module
    pub fn osci_hr_run(&mut self) -> Result<(), NanonisError> {
        self.quick_send("OsciHR.Run", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get the graph data of the selected channel from the Oscilloscope High Resolution
    /// data_to_get: 0 = Current returns the currently displayed data, 1 = Next trigger waits for the next trigger
    /// Returns: (timestamp, time_delta, data_values, timeout_occurred)
    pub fn osci_hr_osci_data_get(
        &mut self,
        osci_index: i32,
        data_to_get: u16,
        timeout_s: f64,
    ) -> Result<(String, f64, Vec<f32>, bool), NanonisError> {
        let result = self.quick_send(
            "OsciHR.OsciDataGet",
            vec![
                NanonisValue::I32(osci_index),
                NanonisValue::U16(data_to_get),
                NanonisValue::F64(timeout_s),
            ],
            vec!["i", "H", "d"],
            vec!["i", "*-c", "d", "i", "*f", "I"],
        )?;

        if result.len() >= 6 {
            let timestamp = result[1].as_string()?.to_string();
            let time_delta = result[2].as_f64()?;
            let data_values = result[4].as_f32_array()?.to_vec();
            let timeout_occurred = result[5].as_u32()? == 1;
            Ok((timestamp, time_delta, data_values, timeout_occurred))
        } else {
            Err(NanonisError::Protocol(
                "Invalid oscilloscope data response".to_string(),
            ))
        }
    }

    /// Set the trigger mode in the Oscilloscope High Resolution
    pub fn osci_hr_trig_mode_set(
        &mut self,
        trigger_mode: impl Into<TriggerMode>,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.TrigModeSet",
            vec![NanonisValue::U16(trigger_mode.into().into())],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the trigger mode in the Oscilloscope High Resolution
    pub fn osci_hr_trig_mode_get(&mut self) -> Result<TriggerMode, NanonisError> {
        let result = self.quick_send("OsciHR.TrigModeGet", vec![], vec![], vec!["H"])?;
        match result.first() {
            Some(value) => {
                let mode_val = value.as_u16()?;
                match mode_val {
                    0 => Ok(TriggerMode::Immediate),
                    1 => Ok(TriggerMode::Level),
                    2 => Ok(TriggerMode::Digital),
                    _ => Err(NanonisError::Protocol(format!(
                        "Unknown trigger mode: {}",
                        mode_val
                    ))),
                }
            }
            None => Err(NanonisError::Protocol(
                "No trigger mode returned".to_string(),
            )),
        }
    }

    /// Set the Level Trigger Channel index in the Oscilloscope High Resolution
    pub fn osci_hr_trig_lev_ch_set(
        &mut self,
        level_trigger_channel_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.TrigLevChSet",
            vec![NanonisValue::I32(level_trigger_channel_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Level Trigger Channel index in the Oscilloscope High Resolution
    pub fn osci_hr_trig_lev_ch_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("OsciHR.TrigLevChGet", vec![], vec![], vec!["i"])?;
        match result.first() {
            Some(value) => Ok(value.as_i32()?),
            None => Err(NanonisError::Protocol(
                "No level trigger channel returned".to_string(),
            )),
        }
    }

    /// Set the Level Trigger value in the Oscilloscope High Resolution
    pub fn osci_hr_trig_lev_val_set(
        &mut self,
        level_trigger_value: impl Into<TriggerLevel>,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.TrigLevValSet",
            vec![NanonisValue::F64(level_trigger_value.into().into())],
            vec!["d"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Level Trigger value in the Oscilloscope High Resolution
    pub fn osci_hr_trig_lev_val_get(&mut self) -> Result<TriggerLevel, NanonisError> {
        let result = self.quick_send("OsciHR.TrigLevValGet", vec![], vec![], vec!["d"])?;
        match result.first() {
            Some(value) => Ok(TriggerLevel(value.as_f64()?)),
            None => Err(NanonisError::Protocol(
                "No level trigger value returned".to_string(),
            )),
        }
    }

    /// Set the Trigger Arming Mode in the Oscilloscope High Resolution
    pub fn osci_hr_trig_arm_mode_set(
        &mut self,
        trigger_arming_mode: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.TrigArmModeSet",
            vec![NanonisValue::U16(trigger_arming_mode)],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Trigger Arming Mode in the Oscilloscope High Resolution
    pub fn osci_hr_trig_arm_mode_get(&mut self) -> Result<u16, NanonisError> {
        let result = self.quick_send("OsciHR.TrigArmModeGet", vec![], vec![], vec!["H"])?;
        match result.first() {
            Some(value) => Ok(value.as_u16()?),
            None => Err(NanonisError::Protocol(
                "No trigger arming mode returned".to_string(),
            )),
        }
    }

    /// Set the Level Trigger Hysteresis in the Oscilloscope High Resolution
    pub fn osci_hr_trig_lev_hyst_set(&mut self, hysteresis: f64) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.TrigLevHystSet",
            vec![NanonisValue::F64(hysteresis)],
            vec!["d"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Level Trigger Hysteresis in the Oscilloscope High Resolution
    pub fn osci_hr_trig_lev_hyst_get(&mut self) -> Result<f64, NanonisError> {
        let result = self.quick_send("OsciHR.TrigLevHystGet", vec![], vec![], vec!["d"])?;
        match result.first() {
            Some(value) => Ok(value.as_f64()?),
            None => Err(NanonisError::Protocol(
                "No trigger hysteresis returned".to_string(),
            )),
        }
    }

    /// Set the Level Trigger Slope in the Oscilloscope High Resolution
    pub fn osci_hr_trig_lev_slope_set(&mut self, slope: u16) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.TrigLevSlopeSet",
            vec![NanonisValue::U16(slope)],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Level Trigger Slope in the Oscilloscope High Resolution
    pub fn osci_hr_trig_lev_slope_get(&mut self) -> Result<u16, NanonisError> {
        let result = self.quick_send("OsciHR.TrigLevSlopeGet", vec![], vec![], vec!["H"])?;
        match result.first() {
            Some(value) => Ok(value.as_u16()?),
            None => Err(NanonisError::Protocol(
                "No trigger slope returned".to_string(),
            )),
        }
    }

    /// Set the Digital Trigger Channel index in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `digital_trigger_channel` - Digital trigger channel index
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_trig_dig_ch_set(
        &mut self,
        digital_trigger_channel: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.TrigDigChSet",
            vec![NanonisValue::I32(digital_trigger_channel)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Digital Trigger Channel index in the Oscilloscope High Resolution.
    ///
    /// # Returns
    /// Digital trigger channel index.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_trig_dig_ch_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("OsciHR.TrigDigChGet", vec![], vec![], vec!["i"])?;
        match result.first() {
            Some(value) => Ok(value.as_i32()?),
            None => Err(NanonisError::Protocol(
                "No digital trigger channel returned".to_string(),
            )),
        }
    }

    /// Set the Digital Trigger Slope in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `slope` - Digital trigger slope (0 = Rising, 1 = Falling, 2 = Both)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_trig_dig_slope_set(&mut self, slope: u16) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.TrigDigSlopeSet",
            vec![NanonisValue::U16(slope)],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Digital Trigger Slope in the Oscilloscope High Resolution.
    ///
    /// # Returns
    /// Digital trigger slope (0 = Rising, 1 = Falling, 2 = Both).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_trig_dig_slope_get(&mut self) -> Result<u16, NanonisError> {
        let result = self.quick_send("OsciHR.TrigDigSlopeGet", vec![], vec![], vec!["H"])?;
        match result.first() {
            Some(value) => Ok(value.as_u16()?),
            None => Err(NanonisError::Protocol(
                "No digital trigger slope returned".to_string(),
            )),
        }
    }

    /// Rearm the trigger in the Oscilloscope High Resolution.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_trig_rearm(&mut self) -> Result<(), NanonisError> {
        self.quick_send("OsciHR.TrigRearm", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Show the PSD (Power Spectral Density) view in the Oscilloscope High Resolution.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_show(&mut self) -> Result<(), NanonisError> {
        self.quick_send("OsciHR.PSDShow", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set the PSD weighting mode in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    /// * `weighting` - Weighting mode (0 = None, 1 = A-weighting, 2 = C-weighting)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_weight_set(
        &mut self,
        osci_index: i32,
        weighting: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.PSDWeightSet",
            vec![
                NanonisValue::I32(osci_index),
                NanonisValue::U16(weighting),
            ],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the PSD weighting mode in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    ///
    /// # Returns
    /// Weighting mode (0 = None, 1 = A-weighting, 2 = C-weighting).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_weight_get(&mut self, osci_index: i32) -> Result<u16, NanonisError> {
        let result = self.quick_send(
            "OsciHR.PSDWeightGet",
            vec![NanonisValue::I32(osci_index)],
            vec!["i"],
            vec!["H"],
        )?;
        match result.first() {
            Some(value) => Ok(value.as_u16()?),
            None => Err(NanonisError::Protocol(
                "No PSD weighting returned".to_string(),
            )),
        }
    }

    /// Set the PSD window function in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    /// * `window` - Window function (0 = None, 1 = Hann, 2 = Hamming, 3 = Blackman-Harris, 4 = Flat Top)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_window_set(
        &mut self,
        osci_index: i32,
        window: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.PSDWindowSet",
            vec![
                NanonisValue::I32(osci_index),
                NanonisValue::U16(window),
            ],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the PSD window function in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    ///
    /// # Returns
    /// Window function (0 = None, 1 = Hann, 2 = Hamming, 3 = Blackman-Harris, 4 = Flat Top).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_window_get(&mut self, osci_index: i32) -> Result<u16, NanonisError> {
        let result = self.quick_send(
            "OsciHR.PSDWindowGet",
            vec![NanonisValue::I32(osci_index)],
            vec!["i"],
            vec!["H"],
        )?;
        match result.first() {
            Some(value) => Ok(value.as_u16()?),
            None => Err(NanonisError::Protocol(
                "No PSD window returned".to_string(),
            )),
        }
    }

    /// Set the PSD averaging type in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    /// * `averaging_type` - Averaging type (0 = None, 1 = Linear, 2 = Exponential)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_avrg_type_set(
        &mut self,
        osci_index: i32,
        averaging_type: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.PSDAvrgTypeSet",
            vec![
                NanonisValue::I32(osci_index),
                NanonisValue::U16(averaging_type),
            ],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the PSD averaging type in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    ///
    /// # Returns
    /// Averaging type (0 = None, 1 = Linear, 2 = Exponential).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_avrg_type_get(&mut self, osci_index: i32) -> Result<u16, NanonisError> {
        let result = self.quick_send(
            "OsciHR.PSDAvrgTypeGet",
            vec![NanonisValue::I32(osci_index)],
            vec!["i"],
            vec!["H"],
        )?;
        match result.first() {
            Some(value) => Ok(value.as_u16()?),
            None => Err(NanonisError::Protocol(
                "No PSD averaging type returned".to_string(),
            )),
        }
    }

    /// Set the PSD averaging count in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    /// * `count` - Number of averages
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_avrg_count_set(
        &mut self,
        osci_index: i32,
        count: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.PSDAvrgCountSet",
            vec![
                NanonisValue::I32(osci_index),
                NanonisValue::I32(count),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the PSD averaging count in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    ///
    /// # Returns
    /// Number of averages.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_avrg_count_get(&mut self, osci_index: i32) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "OsciHR.PSDAvrgCountGet",
            vec![NanonisValue::I32(osci_index)],
            vec!["i"],
            vec!["i"],
        )?;
        match result.first() {
            Some(value) => Ok(value.as_i32()?),
            None => Err(NanonisError::Protocol(
                "No PSD averaging count returned".to_string(),
            )),
        }
    }

    /// Restart PSD averaging in the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_avrg_restart(&mut self, osci_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "OsciHR.PSDAvrgRestart",
            vec![NanonisValue::I32(osci_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the PSD data from the Oscilloscope High Resolution.
    ///
    /// # Arguments
    /// * `osci_index` - Oscilloscope channel index
    /// * `data_to_get` - 0 = Current data, 1 = Wait for next acquisition
    /// * `timeout_s` - Timeout in seconds
    ///
    /// # Returns
    /// Tuple of (frequency_start, frequency_delta, psd_data, timeout_occurred).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn osci_hr_psd_data_get(
        &mut self,
        osci_index: i32,
        data_to_get: u16,
        timeout_s: f64,
    ) -> Result<(f64, f64, Vec<f32>, bool), NanonisError> {
        let result = self.quick_send(
            "OsciHR.PSDDataGet",
            vec![
                NanonisValue::I32(osci_index),
                NanonisValue::U16(data_to_get),
                NanonisValue::F64(timeout_s),
            ],
            vec!["i", "H", "d"],
            vec!["d", "d", "i", "*f", "I"],
        )?;

        if result.len() >= 5 {
            let frequency_start = result[0].as_f64()?;
            let frequency_delta = result[1].as_f64()?;
            let psd_data = result[3].as_f32_array()?.to_vec();
            let timeout_occurred = result[4].as_u32()? == 1;
            Ok((frequency_start, frequency_delta, psd_data, timeout_occurred))
        } else {
            Err(NanonisError::Protocol(
                "Invalid PSD data response".to_string(),
            ))
        }
    }
}
