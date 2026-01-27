use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

impl NanonisClient {
    // ==================== Laser ====================

    /// Switch the laser on or off.
    ///
    /// # Arguments
    /// * `on` - True to enable, false to disable
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn laser_on_off_set(&mut self, on: bool) -> Result<(), NanonisError> {
        self.quick_send(
            "Laser.OnOffSet",
            vec![NanonisValue::U32(if on { 1 } else { 0 })],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the laser status.
    ///
    /// # Returns
    /// True if laser is on.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn laser_on_off_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("Laser.OnOffGet", vec![], vec![], vec!["I"])?;

        Ok(result[0].as_u32()? != 0)
    }

    /// Set the laser setpoint.
    ///
    /// # Arguments
    /// * `setpoint` - Laser setpoint value
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn laser_props_set(&mut self, setpoint: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "Laser.PropsSet",
            vec![NanonisValue::F32(setpoint)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the laser setpoint.
    ///
    /// # Returns
    /// Laser setpoint value.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn laser_props_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("Laser.PropsGet", vec![], vec![], vec!["f"])?;

        result[0].as_f32()
    }

    /// Get the current laser power.
    ///
    /// # Returns
    /// Laser power value.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn laser_power_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("Laser.PowerGet", vec![], vec![], vec!["f"])?;

        result[0].as_f32()
    }
}
