use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

impl NanonisClient {
    // ==================== Multi-Pass ====================

    /// Activate or deactivate Multi-Pass in the Scan Control module.
    ///
    /// # Arguments
    /// * `on` - True to activate, false to deactivate
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn mpass_activate(&mut self, on: bool) -> Result<(), NanonisError> {
        self.quick_send(
            "MPass.Activate",
            vec![NanonisValue::U32(if on { 1 } else { 0 })],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Load a Multi-Pass configuration file (.mpas).
    ///
    /// # Arguments
    /// * `file_path` - Path to the .mpas file (empty to load from session)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn mpass_load(&mut self, file_path: &str) -> Result<(), NanonisError> {
        self.quick_send(
            "MPass.Load",
            vec![NanonisValue::String(file_path.to_string())],
            vec!["+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Save the current Multi-Pass configuration to a file (.mpas).
    ///
    /// # Arguments
    /// * `file_path` - Path to save the .mpas file (empty to save to session)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn mpass_save(&mut self, file_path: &str) -> Result<(), NanonisError> {
        self.quick_send(
            "MPass.Save",
            vec![NanonisValue::String(file_path.to_string())],
            vec!["+*c"],
            vec![],
        )?;
        Ok(())
    }
}
