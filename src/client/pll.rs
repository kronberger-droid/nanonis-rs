use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// PLL excitation output range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PLLExcRange {
    /// 10V range
    #[default]
    V10 = 0,
    /// 1V range
    V1 = 1,
    /// 0.1V range
    V01 = 2,
    /// 0.01V range
    V001 = 3,
    /// 0.001V range
    V0001 = 4,
}

impl From<PLLExcRange> for u16 {
    fn from(r: PLLExcRange) -> Self {
        r as u16
    }
}

impl TryFrom<u16> for PLLExcRange {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PLLExcRange::V10),
            1 => Ok(PLLExcRange::V1),
            2 => Ok(PLLExcRange::V01),
            3 => Ok(PLLExcRange::V001),
            4 => Ok(PLLExcRange::V0001),
            _ => Err(NanonisError::Type(format!(
                "Invalid PLLExcRange value: {}",
                value
            ))),
        }
    }
}

/// PLL input properties.
#[derive(Debug, Clone, Copy, Default)]
pub struct PLLInputProps {
    /// Differential input enabled
    pub differential_input: bool,
    /// 1/10 divider enabled
    pub divider_1_10: bool,
}

/// PLL demodulator input configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct PLLDemodInput {
    /// Input selection
    pub input: u16,
    /// Frequency generator selection
    pub freq_generator: u16,
}

/// PLL frequency/excitation overwrite configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct PLLOverwrite {
    /// Excitation overwrite signal index (-1 for none)
    pub excitation_signal_index: i32,
    /// Frequency overwrite signal index (-1 for none)
    pub frequency_signal_index: i32,
}

/// PLL amplitude controller gain parameters.
#[derive(Debug, Clone, Copy, Default)]
pub struct PLLAmpCtrlGain {
    /// Proportional gain in V/m
    pub p_gain_v_per_m: f32,
    /// Time constant in seconds
    pub time_constant_s: f32,
    /// Integral gain in V/m/s (read-only, computed)
    pub integral_gain_v_per_m_s: f32,
}

/// PLL phase controller gain parameters.
#[derive(Debug, Clone, Copy, Default)]
pub struct PLLPhasCtrlGain {
    /// Proportional gain in Hz/deg
    pub p_gain_hz_per_deg: f32,
    /// Time constant in seconds
    pub time_constant_s: f32,
}

impl NanonisClient {
    // ==================== Input Configuration ====================

    /// Set the input calibration of the oscillation control module.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `calibration_m_per_v` - Input calibration in m/V
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_inp_calibr_set(
        &mut self,
        modulator_index: i32,
        calibration_m_per_v: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.InpCalibrSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(calibration_m_per_v),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the input calibration of the oscillation control module.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Input calibration in m/V.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_inp_calibr_get(&mut self, modulator_index: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "PLL.InpCalibrGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the input range of the oscillation control module.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `input_range_m` - Input range in meters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_inp_range_set(
        &mut self,
        modulator_index: i32,
        input_range_m: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.InpRangeSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(input_range_m),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the input range of the oscillation control module.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Input range in meters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_inp_range_get(&mut self, modulator_index: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "PLL.InpRangeGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the input properties of the oscillation control module.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `props` - Input properties
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_inp_props_set(
        &mut self,
        modulator_index: i32,
        props: &PLLInputProps,
    ) -> Result<(), NanonisError> {
        let diff_flag = if props.differential_input { 1u16 } else { 0u16 };
        let div_flag = if props.divider_1_10 { 1u16 } else { 0u16 };
        self.quick_send(
            "PLL.InpPropsSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::U16(diff_flag),
                NanonisValue::U16(div_flag),
            ],
            vec!["i", "H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the input properties of the oscillation control module.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Input properties.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_inp_props_get(&mut self, modulator_index: i32) -> Result<PLLInputProps, NanonisError> {
        let result = self.quick_send(
            "PLL.InpPropsGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["H", "H"],
        )?;

        if result.len() >= 2 {
            Ok(PLLInputProps {
                differential_input: result[0].as_u16()? != 0,
                divider_1_10: result[1].as_u16()? != 0,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    // ==================== Output Configuration ====================

    /// Set the add external signal status.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `enabled` - True to add external signal to output
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_add_on_off_set(
        &mut self,
        modulator_index: i32,
        enabled: bool,
    ) -> Result<(), NanonisError> {
        let flag = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "PLL.AddOnOffSet",
            vec![NanonisValue::I32(modulator_index), NanonisValue::U32(flag)],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the add external signal status.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// True if external signal is being added.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_add_on_off_get(&mut self, modulator_index: i32) -> Result<bool, NanonisError> {
        let result = self.quick_send(
            "PLL.AddOnOffGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["I"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the PLL output on/off status.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `enabled` - True to enable PLL output
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_out_on_off_set(
        &mut self,
        modulator_index: i32,
        enabled: bool,
    ) -> Result<(), NanonisError> {
        let flag = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "PLL.OutOnOffSet",
            vec![NanonisValue::I32(modulator_index), NanonisValue::U32(flag)],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the PLL output on/off status.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// True if PLL output is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_out_on_off_get(&mut self, modulator_index: i32) -> Result<bool, NanonisError> {
        let result = self.quick_send(
            "PLL.OutOnOffGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["I"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    // ==================== Excitation ====================

    /// Set the excitation range.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `range` - Excitation output range
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_exc_range_set(
        &mut self,
        modulator_index: i32,
        range: PLLExcRange,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.ExcRangeSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::U16(range.into()),
            ],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the excitation range.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Excitation output range.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_exc_range_get(&mut self, modulator_index: i32) -> Result<PLLExcRange, NanonisError> {
        let result = self.quick_send(
            "PLL.ExcRangeGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["H"],
        )?;

        if !result.is_empty() {
            result[0].as_u16()?.try_into()
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the excitation value (drive amplitude).
    ///
    /// Only works when amplitude controller is off.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `excitation_v` - Excitation value in volts
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_excitation_set(
        &mut self,
        modulator_index: i32,
        excitation_v: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.ExcitationSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(excitation_v),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the excitation value (drive amplitude).
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Excitation value in volts.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_excitation_get(&mut self, modulator_index: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "PLL.ExcitationGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    // ==================== Amplitude Controller ====================
    /// Set the amplitude controller setpoint for a PLL modulator.
    ///
    /// Sets the amplitude controller setpoint value for the specified PLL modulator.
    /// This controls the target oscillation amplitude for the phase-locked loop.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `setpoint_m` - Amplitude setpoint in meters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid modulator index.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set amplitude setpoint for first PLL to 1 nanometer
    /// client.pll_amp_ctrl_setpnt_set(1, 1e-9)?;
    ///
    /// // Set amplitude setpoint for second PLL to 500 picometers
    /// client.pll_amp_ctrl_setpnt_set(2, 500e-12)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pll_amp_ctrl_setpnt_set(
        &mut self,
        modulator_index: i32,
        setpoint_m: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.AmpCtrlSetpntSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(setpoint_m),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the amplitude controller setpoint for a PLL modulator.
    ///
    /// Returns the current amplitude controller setpoint value for the specified
    /// PLL modulator.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// * `f32` - Current amplitude setpoint in meters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid modulator index.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Get current amplitude setpoint for first PLL
    /// let setpoint = client.pll_amp_ctrl_setpnt_get(1)?;
    /// println!("Current amplitude setpoint: {:.2e} m", setpoint);
    ///
    /// // Check if setpoint is within expected range
    /// if setpoint > 1e-9 {
    ///     println!("Amplitude setpoint is greater than 1 nm");
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pll_amp_ctrl_setpnt_get(&mut self, modulator_index: i32) -> Result<f32, NanonisError> {
        let response = self.quick_send(
            "PLL.AmpCtrlSetpntGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f"],
        )?;

        match response.first() {
            Some(NanonisValue::F32(setpoint)) => Ok(*setpoint),
            _ => Err(NanonisError::SerializationError(
                "Expected f32 amplitude setpoint".to_string(),
            )),
        }
    }

    /// Set the amplitude controller on/off status for a PLL modulator.
    ///
    /// Switches the amplitude controller for the specified PLL modulator on or off.
    /// When enabled, the amplitude controller actively maintains the oscillation
    /// amplitude at the setpoint value.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `status` - true to turn on, false to turn off
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid modulator index.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Turn on amplitude controller for first PLL
    /// client.pll_amp_ctrl_on_off_set(1, true)?;
    ///
    /// // Turn off amplitude controller for second PLL
    /// client.pll_amp_ctrl_on_off_set(2, false)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pll_amp_ctrl_on_off_set(
        &mut self,
        modulator_index: i32,
        status: bool,
    ) -> Result<(), NanonisError> {
        let status_u32 = if status { 1u32 } else { 0u32 };

        self.quick_send(
            "PLL.AmpCtrlOnOffSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::U32(status_u32),
            ],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the amplitude controller on/off status for a PLL modulator.
    ///
    /// Returns the current on/off status of the amplitude controller for the
    /// specified PLL modulator.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// * `bool` - true if controller is on, false if off
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid modulator index.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Check amplitude controller status for first PLL
    /// let is_on = client.pll_amp_ctrl_on_off_get(1)?;
    /// if is_on {
    ///     println!("Amplitude controller is active");
    /// } else {
    ///     println!("Amplitude controller is inactive");
    /// }
    ///
    /// // Enable controller if it's off
    /// if !is_on {
    ///     client.pll_amp_ctrl_on_off_set(1, true)?;
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pll_amp_ctrl_on_off_get(&mut self, modulator_index: i32) -> Result<bool, NanonisError> {
        let response = self.quick_send(
            "PLL.AmpCtrlOnOffGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["I"],
        )?;

        match response.first() {
            Some(NanonisValue::U32(status)) => Ok(*status != 0),
            _ => Err(NanonisError::InvalidResponse(
                "Expected u32 amplitude controller status".to_string(),
            )),
        }
    }

    /// Set the amplitude controller gain parameters for a PLL modulator.
    ///
    /// Sets the proportional gain and time constant for the amplitude controller
    /// of the specified PLL modulator. These parameters control the response
    /// characteristics of the amplitude control loop.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `p_gain_v_div_m` - Proportional gain in V/m
    /// * `time_constant_s` - Time constant in seconds
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid modulator index.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set moderate gain and fast response for first PLL
    /// client.pll_amp_ctrl_gain_set(1, 1e6, 0.01)?;
    ///
    /// // Set higher gain and slower response for second PLL
    /// client.pll_amp_ctrl_gain_set(2, 5e6, 0.1)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pll_amp_ctrl_gain_set(
        &mut self,
        modulator_index: i32,
        p_gain_v_div_m: f32,
        time_constant_s: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.AmpCtrlGainSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(p_gain_v_div_m),
                NanonisValue::F32(time_constant_s),
            ],
            vec!["i", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the amplitude controller gain parameters for a PLL modulator.
    ///
    /// Returns the current proportional gain and time constant settings for the
    /// amplitude controller of the specified PLL modulator.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// * `(f32, f32)` - Tuple of (proportional gain in V/m, time constant in seconds)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid modulator index.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Get current gain parameters for first PLL
    /// let (p_gain, time_const) = client.pll_amp_ctrl_gain_get(1)?;
    /// println!("P gain: {:.2e} V/m, Time constant: {:.3} s", p_gain, time_const);
    ///
    /// // Check if parameters are within acceptable range
    /// if p_gain < 1e5 {
    ///     println!("Warning: Low proportional gain");
    /// }
    /// if time_const > 1.0 {
    ///     println!("Warning: Slow time constant");
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pll_amp_ctrl_gain_get(
        &mut self,
        modulator_index: i32,
    ) -> Result<(f32, f32), NanonisError> {
        let response = self.quick_send(
            "PLL.AmpCtrlGainGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f", "f"],
        )?;

        match (response.first(), response.get(1)) {
            (Some(NanonisValue::F32(p_gain)), Some(NanonisValue::F32(time_const))) => {
                Ok((*p_gain, *time_const))
            }
            _ => Err(NanonisError::Protocol(
                "Expected f32 gain parameters (p_gain, time_constant)".to_string(),
            )),
        }
    }

    /// Set the amplitude controller bandwidth.
    ///
    /// Uses current Q factor and amplitude to excitation ratio.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `bandwidth_hz` - Bandwidth in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_amp_ctrl_bandwidth_set(
        &mut self,
        modulator_index: i32,
        bandwidth_hz: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.AmpCtrlBandwidthSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(bandwidth_hz),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the amplitude controller bandwidth.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Bandwidth in Hz.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_amp_ctrl_bandwidth_get(&mut self, modulator_index: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "PLL.AmpCtrlBandwidthGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    // ==================== Phase Controller ====================

    /// Set the phase controller on/off status.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `enabled` - True to enable phase controller
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_phas_ctrl_on_off_set(
        &mut self,
        modulator_index: i32,
        enabled: bool,
    ) -> Result<(), NanonisError> {
        let flag = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "PLL.PhasCtrlOnOffSet",
            vec![NanonisValue::I32(modulator_index), NanonisValue::U32(flag)],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the phase controller on/off status.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// True if phase controller is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_phas_ctrl_on_off_get(&mut self, modulator_index: i32) -> Result<bool, NanonisError> {
        let result = self.quick_send(
            "PLL.PhasCtrlOnOffGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["I"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the phase controller gain parameters.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `p_gain_hz_per_deg` - Proportional gain in Hz/deg
    /// * `time_constant_s` - Time constant in seconds
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_phas_ctrl_gain_set(
        &mut self,
        modulator_index: i32,
        p_gain_hz_per_deg: f32,
        time_constant_s: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.PhasCtrlGainSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(p_gain_hz_per_deg),
                NanonisValue::F32(time_constant_s),
            ],
            vec!["i", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the phase controller gain parameters.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Phase controller gain parameters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_phas_ctrl_gain_get(
        &mut self,
        modulator_index: i32,
    ) -> Result<PLLPhasCtrlGain, NanonisError> {
        let result = self.quick_send(
            "PLL.PhasCtrlGainGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f", "f"],
        )?;

        if result.len() >= 2 {
            Ok(PLLPhasCtrlGain {
                p_gain_hz_per_deg: result[0].as_f32()?,
                time_constant_s: result[1].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the phase controller bandwidth.
    ///
    /// Uses current Q factor.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `bandwidth_hz` - Bandwidth in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_phas_ctrl_bandwidth_set(
        &mut self,
        modulator_index: i32,
        bandwidth_hz: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.PhasCtrlBandwidthSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(bandwidth_hz),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the phase controller bandwidth.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Bandwidth in Hz.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_phas_ctrl_bandwidth_get(
        &mut self,
        modulator_index: i32,
    ) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "PLL.PhasCtrlBandwidthGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    // ==================== Frequency ====================

    /// Set the frequency range.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `freq_range_hz` - Frequency range in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_range_set(
        &mut self,
        modulator_index: i32,
        freq_range_hz: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.FreqRangeSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(freq_range_hz),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the frequency range.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Frequency range in Hz.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_range_get(&mut self, modulator_index: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "PLL.FreqRangeGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the center frequency.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `center_freq_hz` - Center frequency in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_center_freq_set(
        &mut self,
        modulator_index: i32,
        center_freq_hz: f64,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.CenterFreqSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F64(center_freq_hz),
            ],
            vec!["i", "d"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the center frequency.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Center frequency in Hz.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_center_freq_get(&mut self, modulator_index: i32) -> Result<f64, NanonisError> {
        let result = self.quick_send(
            "PLL.CenterFreqGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["d"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f64()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the frequency shift.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `freq_shift_hz` - Frequency shift in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_shift_set(
        &mut self,
        modulator_index: i32,
        freq_shift_hz: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.FreqShiftSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::F32(freq_shift_hz),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the frequency shift.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Frequency shift in Hz.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_shift_get(&mut self, modulator_index: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "PLL.FreqShiftGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["f"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Auto-center frequency shift.
    ///
    /// Adds current frequency shift to center frequency and sets frequency shift to zero.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_shift_auto_center(&mut self, modulator_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.FreqShiftAutoCenter",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Set the frequency/excitation overwrite signals.
    ///
    /// Works when corresponding controller is not active.
    /// Use -2 for no change.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    /// * `overwrite` - Overwrite configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_exc_overwrite_set(
        &mut self,
        modulator_index: i32,
        overwrite: &PLLOverwrite,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.FreqExcOverwriteSet",
            vec![
                NanonisValue::I32(modulator_index),
                NanonisValue::I32(overwrite.excitation_signal_index),
                NanonisValue::I32(overwrite.frequency_signal_index),
            ],
            vec!["i", "i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the frequency/excitation overwrite signals.
    ///
    /// # Arguments
    /// * `modulator_index` - PLL modulator index (starts from 1)
    ///
    /// # Returns
    /// Overwrite configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_freq_exc_overwrite_get(
        &mut self,
        modulator_index: i32,
    ) -> Result<PLLOverwrite, NanonisError> {
        let result = self.quick_send(
            "PLL.FreqExcOverwriteGet",
            vec![NanonisValue::I32(modulator_index)],
            vec!["i"],
            vec!["i", "i"],
        )?;

        if result.len() >= 2 {
            Ok(PLLOverwrite {
                excitation_signal_index: result[0].as_i32()?,
                frequency_signal_index: result[1].as_i32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    // ==================== Demodulator ====================

    /// Set the demodulator input and frequency generator.
    ///
    /// # Arguments
    /// * `demodulator_index` - Demodulator index (starts from 1)
    /// * `input` - Demodulator input configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_demod_input_set(
        &mut self,
        demodulator_index: u16,
        input: &PLLDemodInput,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.DemodInputSet",
            vec![
                NanonisValue::U16(demodulator_index),
                NanonisValue::U16(input.input),
                NanonisValue::U16(input.freq_generator),
            ],
            vec!["H", "H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the demodulator input and frequency generator.
    ///
    /// # Arguments
    /// * `demodulator_index` - Demodulator index (starts from 1)
    ///
    /// # Returns
    /// Demodulator input configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_demod_input_get(
        &mut self,
        demodulator_index: u16,
    ) -> Result<PLLDemodInput, NanonisError> {
        let result = self.quick_send(
            "PLL.DemodInputGet",
            vec![NanonisValue::U16(demodulator_index)],
            vec!["H"],
            vec!["H", "H"],
        )?;

        if result.len() >= 2 {
            Ok(PLLDemodInput {
                input: result[0].as_u16()?,
                freq_generator: result[1].as_u16()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the demodulator harmonic.
    ///
    /// Harmonic 1 corresponds to modulation frequency.
    ///
    /// # Arguments
    /// * `demodulator_index` - Demodulator index (starts from 1)
    /// * `harmonic` - Harmonic number
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_demod_harmonic_set(
        &mut self,
        demodulator_index: u16,
        harmonic: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.DemodHarmonicSet",
            vec![
                NanonisValue::U16(demodulator_index),
                NanonisValue::U16(harmonic),
            ],
            vec!["H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the demodulator harmonic.
    ///
    /// # Arguments
    /// * `demodulator_index` - Demodulator index (starts from 1)
    ///
    /// # Returns
    /// Harmonic number.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_demod_harmonic_get(&mut self, demodulator_index: u16) -> Result<u16, NanonisError> {
        let result = self.quick_send(
            "PLL.DemodHarmonicGet",
            vec![NanonisValue::U16(demodulator_index)],
            vec!["H"],
            vec!["H"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_u16()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the demodulator phase reference.
    ///
    /// # Arguments
    /// * `demodulator_index` - Demodulator index (starts from 1)
    /// * `phase_deg` - Phase reference in degrees
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_demod_phas_ref_set(
        &mut self,
        demodulator_index: u16,
        phase_deg: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.DemodPhasRefSet",
            vec![
                NanonisValue::U16(demodulator_index),
                NanonisValue::F32(phase_deg),
            ],
            vec!["H", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the demodulator phase reference.
    ///
    /// # Arguments
    /// * `demodulator_index` - Demodulator index (starts from 1)
    ///
    /// # Returns
    /// Phase reference in degrees.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_demod_phas_ref_get(&mut self, demodulator_index: u16) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "PLL.DemodPhasRefGet",
            vec![NanonisValue::U16(demodulator_index)],
            vec!["H"],
            vec!["f"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the demodulator filter order.
    ///
    /// # Arguments
    /// * `demodulator_index` - Demodulator index (starts from 1)
    /// * `filter_order` - Low-pass filter order
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_demod_filter_set(
        &mut self,
        demodulator_index: u16,
        filter_order: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PLL.DemodFilterSet",
            vec![
                NanonisValue::U16(demodulator_index),
                NanonisValue::U16(filter_order),
            ],
            vec!["H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the demodulator filter order.
    ///
    /// # Arguments
    /// * `demodulator_index` - Demodulator index (starts from 1)
    ///
    /// # Returns
    /// Low-pass filter order.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pll_demod_filter_get(&mut self, demodulator_index: u16) -> Result<u16, NanonisError> {
        let result = self.quick_send(
            "PLL.DemodFilterGet",
            vec![NanonisValue::U16(demodulator_index)],
            vec!["H"],
            vec!["H"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_u16()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
