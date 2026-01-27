use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Interferometer controller properties.
#[derive(Debug, Clone, Copy, Default)]
pub struct InterfCtrlProps {
    /// Integral gain
    pub integral: f32,
    /// Proportional gain
    pub proportional: f32,
    /// Sign (true = positive, false = negative)
    pub positive_sign: bool,
}

impl NanonisClient {
    // ==================== Interferometer ====================

    /// Switch the interferometer controller on or off.
    ///
    /// # Arguments
    /// * `on` - True to enable, false to disable
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_ctrl_on_off_set(&mut self, on: bool) -> Result<(), NanonisError> {
        self.quick_send(
            "Interf.CtrlOnOffSet",
            vec![NanonisValue::U32(if on { 1 } else { 0 })],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the status of the interferometer controller.
    ///
    /// # Returns
    /// True if controller is on.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_ctrl_on_off_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("Interf.CtrlOnOffGet", vec![], vec![], vec!["I"])?;

        Ok(result[0].as_u32()? != 0)
    }

    /// Set the interferometer controller properties.
    ///
    /// # Arguments
    /// * `props` - Controller properties
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_ctrl_props_set(&mut self, props: &InterfCtrlProps) -> Result<(), NanonisError> {
        self.quick_send(
            "Interf.CtrlPropsSet",
            vec![
                NanonisValue::F32(props.integral),
                NanonisValue::F32(props.proportional),
                NanonisValue::U32(if props.positive_sign { 1 } else { 0 }),
            ],
            vec!["f", "f", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the interferometer controller properties.
    ///
    /// # Returns
    /// Controller properties.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_ctrl_props_get(&mut self) -> Result<InterfCtrlProps, NanonisError> {
        let result = self.quick_send("Interf.CtrlPropsGet", vec![], vec![], vec!["f", "f", "I"])?;

        Ok(InterfCtrlProps {
            integral: result[0].as_f32()?,
            proportional: result[1].as_f32()?,
            positive_sign: result[2].as_u32()? != 0,
        })
    }

    /// Set the W-piezo position.
    ///
    /// The interferometer controller must be off to change this.
    ///
    /// # Arguments
    /// * `w_piezo` - W-piezo position value
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_w_piezo_set(&mut self, w_piezo: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "Interf.WPiezoSet",
            vec![NanonisValue::F32(w_piezo)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the W-piezo position.
    ///
    /// # Returns
    /// W-piezo position value.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_w_piezo_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("Interf.WPiezoGet", vec![], vec![], vec!["f"])?;

        result[0].as_f32()
    }

    /// Get the interferometer value.
    ///
    /// # Returns
    /// Interferometer value.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_val_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("Interf.ValGet", vec![], vec![], vec!["f"])?;

        result[0].as_f32()
    }

    /// Open the calibration panel for the interferometer controller.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_ctrl_calibr_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Interf.CtrlCalibrOpen", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Reset the interferometer controller.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_ctrl_reset(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Interf.CtrlReset", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Apply null deflection to the interferometer controller.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn interf_ctrl_null_defl(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Interf.CtrlNullDefl", vec![], vec![], vec![])?;
        Ok(())
    }
}
