use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::{NanonisValue, Position};

/// Follow Me speed configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct FolMeSpeed {
    /// Speed in m/s
    pub speed_m_s: f32,
    /// True if using custom speed, false if using scan speed
    pub custom_speed: bool,
}

/// Follow Me oversampling configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct FolMeOversampling {
    /// Oversampling factor
    pub oversampling: i32,
    /// Sampling rate in samples/s
    pub sampling_rate: f32,
}

/// Point & Shoot experiment configuration.
#[derive(Debug, Clone, Default)]
pub struct FolMePSExperiment {
    /// Selected experiment index
    pub selected: u16,
    /// List of available experiments
    pub experiments: Vec<String>,
}

/// Point & Shoot properties.
#[derive(Debug, Clone, Default)]
pub struct FolMePSProps {
    /// True if scan resumes after experiment
    pub auto_resume: bool,
    /// True to use experiment's basename, false for P&S basename
    pub use_own_basename: bool,
    /// Basename for Point & Shoot files
    pub basename: String,
    /// Path to external VI
    pub external_vi_path: String,
    /// Delay before measurement in seconds
    pub pre_measure_delay_s: f32,
}

/// Auto resume mode for Point & Shoot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PSAutoResume {
    #[default]
    NoChange = 0,
    Resume = 1,
    DontResume = 2,
}

impl From<PSAutoResume> for u32 {
    fn from(mode: PSAutoResume) -> Self {
        mode as u32
    }
}

/// Basename mode for Point & Shoot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PSBasenameMode {
    #[default]
    NoChange = 0,
    UseExperimentBasename = 1,
    UsePSBasename = 2,
}

impl From<PSBasenameMode> for u32 {
    fn from(mode: PSBasenameMode) -> Self {
        mode as u32
    }
}

impl NanonisClient {
    // ==================== Follow Me ====================

    /// Get the current x-y position.
    ///
    /// # Arguments
    /// * `wait_for_newest_data` - If true, waits for newest data
    ///
    /// # Returns
    /// Current position.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_xy_pos_get(
        &mut self,
        wait_for_newest_data: bool,
    ) -> Result<Position, NanonisError> {
        let wait_flag = if wait_for_newest_data { 1u32 } else { 0u32 };
        let result = self.quick_send(
            "FolMe.XYPosGet",
            vec![NanonisValue::U32(wait_flag)],
            vec!["I"],
            vec!["d", "d"],
        )?;

        if result.len() >= 2 {
            Ok(Position {
                x: result[0].as_f64()?,
                y: result[1].as_f64()?,
            })
        } else {
            Err(NanonisError::Protocol(
                "Invalid position response".to_string(),
            ))
        }
    }

    /// Set the x-y position.
    ///
    /// # Arguments
    /// * `position` - Target position
    /// * `wait_until_finished` - If true, waits until move completes
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_xy_pos_set(
        &mut self,
        position: Position,
        wait_until_finished: bool,
    ) -> Result<(), NanonisError> {
        let wait_flag = if wait_until_finished { 1u32 } else { 0u32 };
        self.quick_send(
            "FolMe.XYPosSet",
            vec![
                NanonisValue::F64(position.x),
                NanonisValue::F64(position.y),
                NanonisValue::U32(wait_flag),
            ],
            vec!["d", "d", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Set the tip speed when moving in Follow Me mode.
    ///
    /// # Arguments
    /// * `speed_m_s` - Speed in m/s
    /// * `custom_speed` - True to use custom speed, false for scan speed
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_speed_set(&mut self, speed_m_s: f32, custom_speed: bool) -> Result<(), NanonisError> {
        let custom_speed_flag = if custom_speed { 1u32 } else { 0u32 };
        self.quick_send(
            "FolMe.SpeedSet",
            vec![
                NanonisValue::F32(speed_m_s),
                NanonisValue::U32(custom_speed_flag),
            ],
            vec!["f", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the tip speed configuration for Follow Me mode.
    ///
    /// # Returns
    /// Speed configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_speed_get(&mut self) -> Result<FolMeSpeed, NanonisError> {
        let result = self.quick_send("FolMe.SpeedGet", vec![], vec![], vec!["f", "I"])?;

        Ok(FolMeSpeed {
            speed_m_s: result[0].as_f32()?,
            custom_speed: result[1].as_u32()? != 0,
        })
    }

    /// Set the oversampling for data acquisition in Follow Me mode.
    ///
    /// # Arguments
    /// * `oversampling` - Oversampling factor
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_oversampl_set(&mut self, oversampling: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "FolMe.OversamplSet",
            vec![NanonisValue::I32(oversampling)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the oversampling and sampling rate for Follow Me mode.
    ///
    /// # Returns
    /// Oversampling configuration including sampling rate.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_oversampl_get(&mut self) -> Result<FolMeOversampling, NanonisError> {
        let result = self.quick_send("FolMe.OversamplGet", vec![], vec![], vec!["i", "f"])?;

        Ok(FolMeOversampling {
            oversampling: result[0].as_i32()?,
            sampling_rate: result[1].as_f32()?,
        })
    }

    /// Stop the tip movement in Follow Me mode.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_stop(&mut self) -> Result<(), NanonisError> {
        self.quick_send("FolMe.Stop", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get the Point & Shoot status in Follow Me mode.
    ///
    /// # Returns
    /// True if Point & Shoot is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_ps_on_off_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("FolMe.PSOnOffGet", vec![], vec![], vec!["I"])?;

        Ok(result[0].as_u32()? != 0)
    }

    /// Enable or disable Point & Shoot in Follow Me mode.
    ///
    /// # Arguments
    /// * `enabled` - True to enable, false to disable
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_ps_on_off_set(&mut self, enabled: bool) -> Result<(), NanonisError> {
        self.quick_send(
            "FolMe.PSOnOffSet",
            vec![NanonisValue::U32(if enabled { 1 } else { 0 })],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Point & Shoot experiment configuration.
    ///
    /// # Returns
    /// Selected experiment and list of available experiments.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_ps_exp_get(&mut self) -> Result<FolMePSExperiment, NanonisError> {
        let result = self.quick_send(
            "FolMe.PSExpGet",
            vec![],
            vec![],
            vec!["H", "i", "i", "*+c"],
        )?;

        Ok(FolMePSExperiment {
            selected: result[0].as_u16()?,
            experiments: result[3].as_string_array()?.to_vec(),
        })
    }

    /// Set the Point & Shoot experiment.
    ///
    /// # Arguments
    /// * `experiment_index` - Index of the experiment to select
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_ps_exp_set(&mut self, experiment_index: u16) -> Result<(), NanonisError> {
        self.quick_send(
            "FolMe.PSExpSet",
            vec![NanonisValue::U16(experiment_index)],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Point & Shoot properties.
    ///
    /// # Returns
    /// Point & Shoot configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_ps_props_get(&mut self) -> Result<FolMePSProps, NanonisError> {
        let result = self.quick_send(
            "FolMe.PSPropsGet",
            vec![],
            vec![],
            vec!["I", "I", "i", "*-c", "i", "*-c", "f"],
        )?;

        Ok(FolMePSProps {
            auto_resume: result[0].as_u32()? != 0,
            use_own_basename: result[1].as_u32()? != 0,
            basename: result[3].as_string()?.to_string(),
            external_vi_path: result[5].as_string()?.to_string(),
            pre_measure_delay_s: result[6].as_f32()?,
        })
    }

    /// Set the Point & Shoot properties.
    ///
    /// # Arguments
    /// * `auto_resume` - Resume mode after experiment
    /// * `basename_mode` - Basename selection mode
    /// * `basename` - Basename for Point & Shoot files
    /// * `external_vi_path` - Path to external VI
    /// * `pre_measure_delay_s` - Delay before measurement in seconds
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn folme_ps_props_set(
        &mut self,
        auto_resume: PSAutoResume,
        basename_mode: PSBasenameMode,
        basename: &str,
        external_vi_path: &str,
        pre_measure_delay_s: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "FolMe.PSPropsSet",
            vec![
                NanonisValue::U32(auto_resume.into()),
                NanonisValue::U32(basename_mode.into()),
                NanonisValue::String(basename.to_string()),
                NanonisValue::String(external_vi_path.to_string()),
                NanonisValue::F32(pre_measure_delay_s),
            ],
            vec!["I", "I", "+*c", "+*c", "f"],
            vec![],
        )?;
        Ok(())
    }
}
