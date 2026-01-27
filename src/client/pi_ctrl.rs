use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// PI Controller slope direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PISlope {
    /// No change to current setting
    #[default]
    NoChange = 0,
    /// Positive slope
    Positive = 1,
    /// Negative slope
    Negative = 2,
}

impl From<PISlope> for u16 {
    fn from(slope: PISlope) -> Self {
        slope as u16
    }
}

impl TryFrom<u16> for PISlope {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PISlope::Positive),
            1 => Ok(PISlope::Negative),
            _ => Err(NanonisError::Type(format!(
                "Invalid PISlope value: {}",
                value
            ))),
        }
    }
}

/// PI Controller properties.
#[derive(Debug, Clone, Copy, Default)]
pub struct PICtrlProps {
    /// Setpoint value
    pub setpoint: f32,
    /// Proportional gain
    pub p_gain: f32,
    /// Integral gain
    pub i_gain: f32,
    /// Slope direction
    pub slope: PISlope,
}

/// PI Controller output limits.
#[derive(Debug, Clone, Copy, Default)]
pub struct PICtrlLimits {
    /// Lower output limit
    pub lower_limit: f32,
    /// Upper output limit
    pub upper_limit: f32,
}

/// Information about available control signals.
#[derive(Debug, Clone)]
pub struct ControlSignalInfo {
    /// Currently selected signal index
    pub current_index: i32,
    /// Names of available signals
    pub signal_names: Vec<String>,
    /// Indexes of available signals
    pub signal_indexes: Vec<i32>,
}

impl NanonisClient {
    /// Enable or disable a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
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
    /// client.pi_ctrl_on_off_set(1, true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pi_ctrl_on_off_set(
        &mut self,
        controller_index: i32,
        enabled: bool,
    ) -> Result<(), NanonisError> {
        let status = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "PICtrl.OnOffSet",
            vec![
                NanonisValue::I32(controller_index),
                NanonisValue::U32(status),
            ],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the on/off status of a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    ///
    /// # Returns
    /// True if controller is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pi_ctrl_on_off_get(&mut self, controller_index: i32) -> Result<bool, NanonisError> {
        let result = self.quick_send(
            "PICtrl.OnOffGet",
            vec![NanonisValue::I32(controller_index)],
            vec!["i"],
            vec!["I"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the control channel for a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    /// * `signal_index` - Control signal index
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pi_ctrl_ctrl_ch_set(
        &mut self,
        controller_index: i32,
        signal_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PICtrl.CtrlChSet",
            vec![
                NanonisValue::I32(controller_index),
                NanonisValue::I32(signal_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the control channel information for a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    ///
    /// # Returns
    /// A [`ControlSignalInfo`] struct with current and available signals.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pi_ctrl_ctrl_ch_get(
        &mut self,
        controller_index: i32,
    ) -> Result<ControlSignalInfo, NanonisError> {
        let result = self.quick_send(
            "PICtrl.CtrlChGet",
            vec![NanonisValue::I32(controller_index)],
            vec!["i"],
            vec!["i", "i", "i", "*+c", "i", "*i"],
        )?;

        if result.len() >= 6 {
            Ok(ControlSignalInfo {
                current_index: result[0].as_i32()?,
                signal_names: result[3].as_string_array()?.to_vec(),
                signal_indexes: result[5].as_i32_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the input channel for a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    /// * `input_index` - Input signal index
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pi_ctrl_input_ch_set(
        &mut self,
        controller_index: i32,
        input_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PICtrl.InputChSet",
            vec![
                NanonisValue::I32(controller_index),
                NanonisValue::I32(input_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the input channel information for a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    ///
    /// # Returns
    /// A [`ControlSignalInfo`] struct with current and available input signals.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pi_ctrl_input_ch_get(
        &mut self,
        controller_index: i32,
    ) -> Result<ControlSignalInfo, NanonisError> {
        let result = self.quick_send(
            "PICtrl.InputChGet",
            vec![NanonisValue::I32(controller_index)],
            vec!["i"],
            vec!["i", "i", "i", "*+c", "i", "*i"],
        )?;

        if result.len() >= 6 {
            Ok(ControlSignalInfo {
                current_index: result[0].as_i32()?,
                signal_names: result[3].as_string_array()?.to_vec(),
                signal_indexes: result[5].as_i32_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the properties of a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    /// * `props` - A [`PICtrlProps`] struct with controller properties
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::pi_ctrl::{PICtrlProps, PISlope};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let props = PICtrlProps {
    ///     setpoint: 0.0,
    ///     p_gain: 1.0,
    ///     i_gain: 100.0,
    ///     slope: PISlope::Positive,
    /// };
    /// client.pi_ctrl_props_set(1, &props)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pi_ctrl_props_set(
        &mut self,
        controller_index: i32,
        props: &PICtrlProps,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PICtrl.PropsSet",
            vec![
                NanonisValue::I32(controller_index),
                NanonisValue::F32(props.setpoint),
                NanonisValue::F32(props.p_gain),
                NanonisValue::F32(props.i_gain),
                NanonisValue::U16(props.slope.into()),
            ],
            vec!["i", "f", "f", "f", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the properties of a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    ///
    /// # Returns
    /// A [`PICtrlProps`] struct with current controller properties.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pi_ctrl_props_get(&mut self, controller_index: i32) -> Result<PICtrlProps, NanonisError> {
        let result = self.quick_send(
            "PICtrl.PropsGet",
            vec![NanonisValue::I32(controller_index)],
            vec!["i"],
            vec!["f", "f", "f", "H"],
        )?;

        if result.len() >= 4 {
            Ok(PICtrlProps {
                setpoint: result[0].as_f32()?,
                p_gain: result[1].as_f32()?,
                i_gain: result[2].as_f32()?,
                slope: result[3].as_u16()?.try_into()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the control channel output limits for a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    /// * `limits` - A [`PICtrlLimits`] struct with output limits
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pi_ctrl_ctrl_ch_props_set(
        &mut self,
        controller_index: i32,
        limits: &PICtrlLimits,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "PICtrl.CtrlChPropsSet",
            vec![
                NanonisValue::I32(controller_index),
                NanonisValue::F32(limits.lower_limit),
                NanonisValue::F32(limits.upper_limit),
            ],
            vec!["i", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the control channel output limits for a PI controller.
    ///
    /// # Arguments
    /// * `controller_index` - Controller index (1-8)
    ///
    /// # Returns
    /// A [`PICtrlLimits`] struct with current output limits.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pi_ctrl_ctrl_ch_props_get(
        &mut self,
        controller_index: i32,
    ) -> Result<PICtrlLimits, NanonisError> {
        let result = self.quick_send(
            "PICtrl.CtrlChPropsGet",
            vec![NanonisValue::I32(controller_index)],
            vec!["i"],
            vec!["f", "f"],
        )?;

        if result.len() >= 2 {
            Ok(PICtrlLimits {
                lower_limit: result[0].as_f32()?,
                upper_limit: result[1].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
