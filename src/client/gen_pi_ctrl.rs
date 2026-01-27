use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Slope direction for Generic PI controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GenPISlope {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// Positive slope
    Positive = 1,
    /// Negative slope
    Negative = 2,
}

impl From<GenPISlope> for u16 {
    fn from(slope: GenPISlope) -> Self {
        slope as u16
    }
}

impl TryFrom<u16> for GenPISlope {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GenPISlope::Negative),
            1 => Ok(GenPISlope::Positive),
            _ => Err(NanonisError::Type(format!(
                "Invalid GenPISlope value: {}",
                value
            ))),
        }
    }
}

/// AC mode toggle for demodulator channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ACMode {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// AC mode on
    On = 1,
    /// AC mode off
    Off = 2,
}

impl From<ACMode> for u16 {
    fn from(mode: ACMode) -> Self {
        mode as u16
    }
}

/// Generic PI Controller properties.
#[derive(Debug, Clone, Copy, Default)]
pub struct GenPICtrlProps {
    /// Setpoint value
    pub setpoint: f32,
    /// Proportional gain
    pub p_gain: f32,
    /// Time constant
    pub time_constant: f32,
    /// Slope direction
    pub slope: GenPISlope,
}

/// Analog output properties for Generic PI controller.
#[derive(Debug, Clone)]
pub struct AOProps {
    /// Signal name
    pub signal_name: String,
    /// Physical units
    pub units: String,
    /// Upper physical limit
    pub upper_limit: f32,
    /// Lower physical limit
    pub lower_limit: f32,
    /// Calibration per volt
    pub calibration_per_volt: f32,
    /// Offset in physical units
    pub offset: f32,
}

impl Default for AOProps {
    fn default() -> Self {
        Self {
            signal_name: String::new(),
            units: String::new(),
            upper_limit: 10.0,
            lower_limit: -10.0,
            calibration_per_volt: 1.0,
            offset: 0.0,
        }
    }
}

impl NanonisClient {
    /// Enable or disable the Generic PI Controller.
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
    /// client.gen_pi_ctrl_on_off_set(true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gen_pi_ctrl_on_off_set(&mut self, enabled: bool) -> Result<(), NanonisError> {
        let status = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "GenPICtrl.OnOffSet",
            vec![NanonisValue::U32(status)],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the on/off status of the Generic PI Controller.
    ///
    /// # Returns
    /// True if controller is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_on_off_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("GenPICtrl.OnOffGet", vec![], vec![], vec!["I"])?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the output signal value of the User Output controlled by the Generic PI controller.
    ///
    /// # Arguments
    /// * `output_value` - Output value
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_ao_val_set(&mut self, output_value: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "GenPICtrl.AOValSet",
            vec![NanonisValue::F32(output_value)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the output signal value of the User Output controlled by the Generic PI controller.
    ///
    /// # Returns
    /// The current output value.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_ao_val_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("GenPICtrl.AOValGet", vec![], vec![], vec!["f"])?;

        if !result.is_empty() {
            Ok(result[0].as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the properties of the User Output controlled by the Generic PI controller.
    ///
    /// # Arguments
    /// * `props` - An [`AOProps`] struct with analog output properties
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_ao_props_set(&mut self, props: &AOProps) -> Result<(), NanonisError> {
        self.quick_send(
            "GenPICtrl.AOPropsSet",
            vec![
                NanonisValue::String(props.signal_name.clone()),
                NanonisValue::String(props.units.clone()),
                NanonisValue::F32(props.upper_limit),
                NanonisValue::F32(props.lower_limit),
                NanonisValue::F32(props.calibration_per_volt),
                NanonisValue::F32(props.offset),
            ],
            vec!["+*c", "+*c", "f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the properties of the User Output controlled by the Generic PI controller.
    ///
    /// # Returns
    /// An [`AOProps`] struct with current analog output properties.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_ao_props_get(&mut self) -> Result<AOProps, NanonisError> {
        let result = self.quick_send(
            "GenPICtrl.AOPropsGet",
            vec![],
            vec![],
            vec!["i", "*-c", "i", "*-c", "f", "f", "f", "f"],
        )?;

        if result.len() >= 8 {
            Ok(AOProps {
                signal_name: result[1].as_string()?.to_string(),
                units: result[3].as_string()?.to_string(),
                upper_limit: result[4].as_f32()?,
                lower_limit: result[5].as_f32()?,
                calibration_per_volt: result[6].as_f32()?,
                offset: result[7].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the index of the User Output controlled by the Generic PI controller.
    ///
    /// # Arguments
    /// * `output_index` - Output index (1 to number of available outputs)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_mod_ch_set(&mut self, output_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "GenPICtrl.ModChSet",
            vec![NanonisValue::I32(output_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the index of the User Output controlled by the Generic PI controller.
    ///
    /// # Returns
    /// The output index (0 means no output selected).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_mod_ch_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("GenPICtrl.ModChGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the index of the signal demodulated by the Generic PI controller.
    ///
    /// # Arguments
    /// * `input_index` - Input index (0-127 for signals, -1 for no change)
    /// * `ac_mode` - AC mode setting
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_demod_ch_set(
        &mut self,
        input_index: i32,
        ac_mode: ACMode,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "GenPICtrl.DemodChSet",
            vec![
                NanonisValue::I32(input_index),
                NanonisValue::U16(ac_mode.into()),
            ],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the index of the signal demodulated by the Generic PI controller.
    ///
    /// # Returns
    /// The input index.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_demod_ch_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("GenPICtrl.DemodChGet", vec![], vec![], vec!["i"])?;

        if !result.is_empty() {
            Ok(result[0].as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the properties of the Generic PI controller.
    ///
    /// # Arguments
    /// * `props` - A [`GenPICtrlProps`] struct with controller properties
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::gen_pi_ctrl::{GenPICtrlProps, GenPISlope};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let props = GenPICtrlProps {
    ///     setpoint: 0.0,
    ///     p_gain: 1.0,
    ///     time_constant: 0.001,
    ///     slope: GenPISlope::Positive,
    /// };
    /// client.gen_pi_ctrl_props_set(&props)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gen_pi_ctrl_props_set(&mut self, props: &GenPICtrlProps) -> Result<(), NanonisError> {
        self.quick_send(
            "GenPICtrl.PropsSet",
            vec![
                NanonisValue::F32(props.setpoint),
                NanonisValue::F32(props.p_gain),
                NanonisValue::F32(props.time_constant),
                NanonisValue::U16(props.slope.into()),
            ],
            vec!["f", "f", "f", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the properties of the Generic PI controller.
    ///
    /// # Returns
    /// A [`GenPICtrlProps`] struct with current controller properties.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_pi_ctrl_props_get(&mut self) -> Result<GenPICtrlProps, NanonisError> {
        let result =
            self.quick_send("GenPICtrl.PropsGet", vec![], vec![], vec!["f", "f", "f", "H"])?;

        if result.len() >= 4 {
            Ok(GenPICtrlProps {
                setpoint: result[0].as_f32()?,
                p_gain: result[1].as_f32()?,
                time_constant: result[2].as_f32()?,
                slope: result[3].as_u16()?.try_into()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
