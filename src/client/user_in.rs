use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

impl NanonisClient {
    // ==================== User Inputs ====================

    /// Set the calibration of a user input.
    ///
    /// # Arguments
    /// * `input_index` - Input index (1 to available inputs)
    /// * `calibration_per_volt` - Calibration value per volt
    /// * `offset_physical_units` - Offset in physical units
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn user_in_calibr_set(
        &mut self,
        input_index: i32,
        calibration_per_volt: f32,
        offset_physical_units: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "UserIn.CalibrSet",
            vec![
                NanonisValue::I32(input_index),
                NanonisValue::F32(calibration_per_volt),
                NanonisValue::F32(offset_physical_units),
            ],
            vec!["i", "f", "f"],
            vec![],
        )?;
        Ok(())
    }
}
