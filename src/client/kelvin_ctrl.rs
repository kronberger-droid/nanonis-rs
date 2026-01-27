use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Kelvin controller slope direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KelvinSlope {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// Positive slope
    Positive = 1,
    /// Negative slope
    Negative = 2,
}

impl From<KelvinSlope> for u16 {
    fn from(slope: KelvinSlope) -> Self {
        slope as u16
    }
}

impl TryFrom<u16> for KelvinSlope {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KelvinSlope::Negative),
            1 => Ok(KelvinSlope::Positive),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid KelvinSlope value: {}",
                value
            ))),
        }
    }
}

/// AC mode toggle for Kelvin controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KelvinACMode {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// AC mode on
    On = 1,
    /// AC mode off
    Off = 2,
}

impl From<KelvinACMode> for u16 {
    fn from(mode: KelvinACMode) -> Self {
        mode as u16
    }
}

/// Kelvin controller gain parameters.
#[derive(Debug, Clone, Copy, Default)]
pub struct KelvinGain {
    /// Proportional gain
    pub p_gain: f32,
    /// Time constant in seconds
    pub time_constant_s: f32,
    /// Slope direction
    pub slope: KelvinSlope,
}

/// Kelvin controller modulation parameters.
#[derive(Debug, Clone, Copy, Default)]
pub struct KelvinModParams {
    /// Modulation frequency in Hz
    pub frequency_hz: f32,
    /// Modulation amplitude
    pub amplitude: f32,
    /// Modulation phase in degrees
    pub phase_deg: f32,
}

/// Kelvin controller modulation status.
#[derive(Debug, Clone, Copy, Default)]
pub struct KelvinModStatus {
    /// AC mode enabled
    pub ac_mode: bool,
    /// Modulation enabled
    pub modulation: bool,
}

/// Kelvin controller bias limits.
#[derive(Debug, Clone, Copy, Default)]
pub struct KelvinBiasLimits {
    /// High bias limit in volts
    pub high_limit_v: f32,
    /// Low bias limit in volts
    pub low_limit_v: f32,
}

impl NanonisClient {
    /// Enable or disable the Kelvin controller.
    ///
    /// # Arguments
    /// * `enabled` - True to enable, false to disable
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.kelvin_ctrl_on_off_set(true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn kelvin_ctrl_on_off_set(&mut self, enabled: bool) -> Result<(), NanonisError> {
        let flag = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "KelvinCtrl.CtrlOnOffSet",
            vec![NanonisValue::U32(flag)],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Kelvin controller on/off status.
    ///
    /// # Returns
    /// True if controller is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_on_off_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("KelvinCtrl.CtrlOnOffGet", vec![], vec![], vec!["I"])?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Kelvin controller setpoint.
    ///
    /// # Arguments
    /// * `setpoint` - Setpoint value
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_setpnt_set(&mut self, setpoint: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "KelvinCtrl.SetpntSet",
            vec![NanonisValue::F32(setpoint)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Kelvin controller setpoint.
    ///
    /// # Returns
    /// The current setpoint value.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_setpnt_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("KelvinCtrl.SetpntGet", vec![], vec![], vec!["f"])?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Kelvin controller gain parameters.
    ///
    /// # Arguments
    /// * `gain` - A [`KelvinGain`] struct with gain parameters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_gain_set(&mut self, gain: &KelvinGain) -> Result<(), NanonisError> {
        self.quick_send(
            "KelvinCtrl.GainSet",
            vec![
                NanonisValue::F32(gain.p_gain),
                NanonisValue::F32(gain.time_constant_s),
                NanonisValue::U16(gain.slope.into()),
            ],
            vec!["f", "f", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Kelvin controller gain parameters.
    ///
    /// # Returns
    /// A [`KelvinGain`] struct with current gain parameters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_gain_get(&mut self) -> Result<KelvinGain, NanonisError> {
        let result =
            self.quick_send("KelvinCtrl.GainGet", vec![], vec![], vec!["f", "f", "H"])?;

        if result.len() >= 3 {
            Ok(KelvinGain {
                p_gain: result[0].as_f32()?,
                time_constant_s: result[1].as_f32()?,
                slope: result[2].as_u16()?.try_into()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Kelvin controller modulation parameters.
    ///
    /// # Arguments
    /// * `params` - A [`KelvinModParams`] struct with modulation parameters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_mod_params_set(
        &mut self,
        params: &KelvinModParams,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "KelvinCtrl.ModParamsSet",
            vec![
                NanonisValue::F32(params.frequency_hz),
                NanonisValue::F32(params.amplitude),
                NanonisValue::F32(params.phase_deg),
            ],
            vec!["f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Kelvin controller modulation parameters.
    ///
    /// # Returns
    /// A [`KelvinModParams`] struct with current modulation parameters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_mod_params_get(&mut self) -> Result<KelvinModParams, NanonisError> {
        let result =
            self.quick_send("KelvinCtrl.ModParamsGet", vec![], vec![], vec!["f", "f", "f"])?;

        if result.len() >= 3 {
            Ok(KelvinModParams {
                frequency_hz: result[0].as_f32()?,
                amplitude: result[1].as_f32()?,
                phase_deg: result[2].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Kelvin controller AC mode and modulation status.
    ///
    /// # Arguments
    /// * `ac_mode` - AC mode setting
    /// * `modulation` - True to enable modulation, false to disable
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_mod_on_off_set(
        &mut self,
        ac_mode: KelvinACMode,
        modulation: bool,
    ) -> Result<(), NanonisError> {
        let mod_flag = if modulation { 1u16 } else { 0u16 };
        self.quick_send(
            "KelvinCtrl.ModOnOffSet",
            vec![
                NanonisValue::U16(ac_mode.into()),
                NanonisValue::U16(mod_flag),
            ],
            vec!["H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Kelvin controller AC mode and modulation status.
    ///
    /// # Returns
    /// A [`KelvinModStatus`] struct with current status.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_mod_on_off_get(&mut self) -> Result<KelvinModStatus, NanonisError> {
        let result =
            self.quick_send("KelvinCtrl.ModOnOffGet", vec![], vec![], vec!["H", "H"])?;

        if result.len() >= 2 {
            Ok(KelvinModStatus {
                ac_mode: result[0].as_u16()? != 0,
                modulation: result[1].as_u16()? != 0,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Kelvin controller demodulated/control signal index.
    ///
    /// # Arguments
    /// * `signal_index` - Signal index
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_signal_set(&mut self, signal_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "KelvinCtrl.CtrlSignalSet",
            vec![NanonisValue::I32(signal_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Kelvin controller demodulated/control signal index.
    ///
    /// # Returns
    /// The signal index.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_signal_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("KelvinCtrl.CtrlSignalGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get the amplitude of the demodulated/control signal.
    ///
    /// # Returns
    /// The amplitude value.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_amp_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("KelvinCtrl.AmpGet", vec![], vec![], vec!["f"])?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Kelvin controller bias limits.
    ///
    /// The bias voltage will be limited to these values as long as the controller is on.
    ///
    /// # Arguments
    /// * `limits` - A [`KelvinBiasLimits`] struct with bias limits
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_bias_limits_set(
        &mut self,
        limits: &KelvinBiasLimits,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "KelvinCtrl.BiasLimitsSet",
            vec![
                NanonisValue::F32(limits.high_limit_v),
                NanonisValue::F32(limits.low_limit_v),
            ],
            vec!["f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Kelvin controller bias limits.
    ///
    /// # Returns
    /// A [`KelvinBiasLimits`] struct with current bias limits.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn kelvin_ctrl_bias_limits_get(&mut self) -> Result<KelvinBiasLimits, NanonisError> {
        let result =
            self.quick_send("KelvinCtrl.BiasLimitsGet", vec![], vec![], vec!["f", "f"])?;

        if result.len() >= 2 {
            Ok(KelvinBiasLimits {
                high_limit_v: result[0].as_f32()?,
                low_limit_v: result[1].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
