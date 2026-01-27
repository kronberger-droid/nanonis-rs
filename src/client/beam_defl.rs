use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Deflection signal type for beam deflection module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeflectionSignal {
    #[default]
    Horizontal = 0,
    Vertical = 1,
    Intensity = 2,
}

impl From<DeflectionSignal> for u16 {
    fn from(sig: DeflectionSignal) -> Self {
        sig as u16
    }
}

/// Beam deflection configuration.
#[derive(Debug, Clone, Default)]
pub struct BeamDeflConfig {
    /// Signal name
    pub name: String,
    /// Physical units
    pub units: String,
    /// Calibration value
    pub calibration: f32,
    /// Offset value
    pub offset: f32,
}

impl NanonisClient {
    // ==================== Beam Deflection ====================

    /// Set the horizontal deflection configuration.
    ///
    /// # Arguments
    /// * `config` - Beam deflection configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn beam_defl_hor_config_set(&mut self, config: &BeamDeflConfig) -> Result<(), NanonisError> {
        self.quick_send(
            "BeamDefl.HorConfigSet",
            vec![
                NanonisValue::String(config.name.clone()),
                NanonisValue::String(config.units.clone()),
                NanonisValue::F32(config.calibration),
                NanonisValue::F32(config.offset),
            ],
            vec!["+*c", "+*c", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the horizontal deflection configuration.
    ///
    /// # Returns
    /// Beam deflection configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn beam_defl_hor_config_get(&mut self) -> Result<BeamDeflConfig, NanonisError> {
        let result = self.quick_send(
            "BeamDefl.HorConfigGet",
            vec![],
            vec![],
            vec!["i", "*-c", "i", "*-c", "f", "f"],
        )?;

        Ok(BeamDeflConfig {
            name: result[1].as_string()?.to_string(),
            units: result[3].as_string()?.to_string(),
            calibration: result[4].as_f32()?,
            offset: result[5].as_f32()?,
        })
    }

    /// Set the vertical deflection configuration.
    ///
    /// # Arguments
    /// * `config` - Beam deflection configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn beam_defl_ver_config_set(&mut self, config: &BeamDeflConfig) -> Result<(), NanonisError> {
        self.quick_send(
            "BeamDefl.VerConfigSet",
            vec![
                NanonisValue::String(config.name.clone()),
                NanonisValue::String(config.units.clone()),
                NanonisValue::F32(config.calibration),
                NanonisValue::F32(config.offset),
            ],
            vec!["+*c", "+*c", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the vertical deflection configuration.
    ///
    /// # Returns
    /// Beam deflection configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn beam_defl_ver_config_get(&mut self) -> Result<BeamDeflConfig, NanonisError> {
        let result = self.quick_send(
            "BeamDefl.VerConfigGet",
            vec![],
            vec![],
            vec!["i", "*-c", "i", "*-c", "f", "f"],
        )?;

        Ok(BeamDeflConfig {
            name: result[1].as_string()?.to_string(),
            units: result[3].as_string()?.to_string(),
            calibration: result[4].as_f32()?,
            offset: result[5].as_f32()?,
        })
    }

    /// Set the intensity signal configuration.
    ///
    /// # Arguments
    /// * `config` - Beam deflection configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn beam_defl_int_config_set(&mut self, config: &BeamDeflConfig) -> Result<(), NanonisError> {
        self.quick_send(
            "BeamDefl.IntConfigSet",
            vec![
                NanonisValue::String(config.name.clone()),
                NanonisValue::String(config.units.clone()),
                NanonisValue::F32(config.calibration),
                NanonisValue::F32(config.offset),
            ],
            vec!["+*c", "+*c", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the intensity signal configuration.
    ///
    /// # Returns
    /// Beam deflection configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn beam_defl_int_config_get(&mut self) -> Result<BeamDeflConfig, NanonisError> {
        let result = self.quick_send(
            "BeamDefl.IntConfigGet",
            vec![],
            vec![],
            vec!["i", "*-c", "i", "*-c", "f", "f"],
        )?;

        Ok(BeamDeflConfig {
            name: result[1].as_string()?.to_string(),
            units: result[3].as_string()?.to_string(),
            calibration: result[4].as_f32()?,
            offset: result[5].as_f32()?,
        })
    }

    /// Auto-offset the beam deflection signal.
    ///
    /// Adds the current deflection value to the offset so the signal is close to 0.
    ///
    /// # Arguments
    /// * `signal` - Which deflection signal to offset
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn beam_defl_auto_offset(&mut self, signal: DeflectionSignal) -> Result<(), NanonisError> {
        self.quick_send(
            "BeamDefl.AutoOffset",
            vec![NanonisValue::U16(signal.into())],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }
}
