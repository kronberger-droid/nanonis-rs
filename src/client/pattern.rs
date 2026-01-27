use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Pattern type for grid experiments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternType {
    /// No change to current pattern
    #[default]
    NoChange = 0,
    /// Grid pattern
    Grid = 1,
    /// Line pattern
    Line = 2,
    /// Cloud pattern
    Cloud = 3,
}

impl From<PatternType> for u16 {
    fn from(p: PatternType) -> Self {
        p as u16
    }
}

/// Grid pattern configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct GridConfig {
    /// Number of points in X direction
    pub num_points_x: i32,
    /// Number of points in Y direction
    pub num_points_y: i32,
    /// X coordinate of grid center (meters)
    pub center_x_m: f32,
    /// Y coordinate of grid center (meters)
    pub center_y_m: f32,
    /// Grid width (meters)
    pub width_m: f32,
    /// Grid height (meters)
    pub height_m: f32,
    /// Grid rotation angle (degrees)
    pub angle_deg: f32,
}

/// Line pattern configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct LineConfig {
    /// Number of points on line
    pub num_points: i32,
    /// X coordinate of point 1 (meters)
    pub point1_x_m: f32,
    /// Y coordinate of point 1 (meters)
    pub point1_y_m: f32,
    /// X coordinate of point 2 (meters)
    pub point2_x_m: f32,
    /// Y coordinate of point 2 (meters)
    pub point2_y_m: f32,
}

/// Cloud pattern configuration.
#[derive(Debug, Clone, Default)]
pub struct CloudConfig {
    /// X coordinates of points (meters)
    pub x_coords_m: Vec<f32>,
    /// Y coordinates of points (meters)
    pub y_coords_m: Vec<f32>,
}

/// Pattern experiment properties.
#[derive(Debug, Clone)]
pub struct PatternProps {
    /// List of available experiments
    pub available_experiments: Vec<String>,
    /// Currently selected experiment
    pub selected_experiment: String,
    /// Path to external VI
    pub external_vi_path: String,
    /// Pre-measure delay in seconds
    pub pre_measure_delay_s: f32,
    /// Save scan channels to file
    pub save_scan_channels: bool,
}

impl Default for PatternProps {
    fn default() -> Self {
        Self {
            available_experiments: vec![],
            selected_experiment: String::new(),
            external_vi_path: String::new(),
            pre_measure_delay_s: 0.0,
            save_scan_channels: false,
        }
    }
}

impl NanonisClient {
    /// Open the selected grid experiment.
    ///
    /// Required to configure the experiment and be able to start it.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.pattern_exp_open()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pattern_exp_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Pattern.ExpOpen", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Start the selected grid experiment.
    ///
    /// Before using this function, select the experiment through `pattern_props_set`,
    /// and be sure to have it open.
    ///
    /// # Arguments
    /// * `pattern` - Pattern type to switch to before starting
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_exp_start(&mut self, pattern: PatternType) -> Result<(), NanonisError> {
        self.quick_send(
            "Pattern.ExpStart",
            vec![NanonisValue::U16(pattern.into())],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Pause or resume the selected grid experiment.
    ///
    /// # Arguments
    /// * `pause` - True to pause, false to resume
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_exp_pause(&mut self, pause: bool) -> Result<(), NanonisError> {
        let flag = if pause { 1u32 } else { 0u32 };
        self.quick_send(
            "Pattern.ExpPause",
            vec![NanonisValue::U32(flag)],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Stop the selected grid experiment.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_exp_stop(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Pattern.ExpStop", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get the status of the selected grid experiment.
    ///
    /// # Returns
    /// True if the experiment is running.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_exp_status_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("Pattern.ExpStatusGet", vec![], vec![], vec!["I"])?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the grid pattern parameters.
    ///
    /// # Arguments
    /// * `set_active` - If true, switch to grid pattern
    /// * `config` - Grid configuration
    /// * `use_scan_frame` - If true, use scan frame size
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_grid_set(
        &mut self,
        set_active: bool,
        config: &GridConfig,
        use_scan_frame: bool,
    ) -> Result<(), NanonisError> {
        let active_flag = if set_active { 1u32 } else { 0u32 };
        let frame_flag = if use_scan_frame { 1u32 } else { 0u32 };

        self.quick_send(
            "Pattern.GridSet",
            vec![
                NanonisValue::U32(active_flag),
                NanonisValue::I32(config.num_points_x),
                NanonisValue::I32(config.num_points_y),
                NanonisValue::U32(frame_flag),
                NanonisValue::F32(config.center_x_m),
                NanonisValue::F32(config.center_y_m),
                NanonisValue::F32(config.width_m),
                NanonisValue::F32(config.height_m),
                NanonisValue::F32(config.angle_deg),
            ],
            vec!["I", "i", "i", "I", "f", "f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the grid pattern parameters.
    ///
    /// # Returns
    /// A [`GridConfig`] struct with current grid parameters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_grid_get(&mut self) -> Result<GridConfig, NanonisError> {
        let result = self.quick_send(
            "Pattern.GridGet",
            vec![],
            vec![],
            vec!["i", "i", "f", "f", "f", "f", "f"],
        )?;

        if result.len() >= 7 {
            Ok(GridConfig {
                num_points_x: result[0].as_i32()?,
                num_points_y: result[1].as_i32()?,
                center_x_m: result[2].as_f32()?,
                center_y_m: result[3].as_f32()?,
                width_m: result[4].as_f32()?,
                height_m: result[5].as_f32()?,
                angle_deg: result[6].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the line pattern parameters.
    ///
    /// # Arguments
    /// * `set_active` - If true, switch to line pattern
    /// * `config` - Line configuration
    /// * `use_scan_frame` - If true, use scan frame diagonal
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_line_set(
        &mut self,
        set_active: bool,
        config: &LineConfig,
        use_scan_frame: bool,
    ) -> Result<(), NanonisError> {
        let active_flag = if set_active { 1u32 } else { 0u32 };
        let frame_flag = if use_scan_frame { 1u32 } else { 0u32 };

        self.quick_send(
            "Pattern.LineSet",
            vec![
                NanonisValue::U32(active_flag),
                NanonisValue::I32(config.num_points),
                NanonisValue::U32(frame_flag),
                NanonisValue::F32(config.point1_x_m),
                NanonisValue::F32(config.point1_y_m),
                NanonisValue::F32(config.point2_x_m),
                NanonisValue::F32(config.point2_y_m),
            ],
            vec!["I", "i", "I", "f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the line pattern parameters.
    ///
    /// # Returns
    /// A [`LineConfig`] struct with current line parameters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_line_get(&mut self) -> Result<LineConfig, NanonisError> {
        let result = self.quick_send(
            "Pattern.LineGet",
            vec![],
            vec![],
            vec!["i", "f", "f", "f", "f"],
        )?;

        if result.len() >= 5 {
            Ok(LineConfig {
                num_points: result[0].as_i32()?,
                point1_x_m: result[1].as_f32()?,
                point1_y_m: result[2].as_f32()?,
                point2_x_m: result[3].as_f32()?,
                point2_y_m: result[4].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the cloud pattern parameters.
    ///
    /// # Arguments
    /// * `set_active` - If true, switch to cloud pattern
    /// * `config` - Cloud configuration with point coordinates
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_cloud_set(
        &mut self,
        set_active: bool,
        config: &CloudConfig,
    ) -> Result<(), NanonisError> {
        let active_flag = if set_active { 1u32 } else { 0u32 };
        let num_points = config.x_coords_m.len() as i32;

        self.quick_send(
            "Pattern.CloudSet",
            vec![
                NanonisValue::U32(active_flag),
                NanonisValue::I32(num_points),
                NanonisValue::ArrayF32(config.x_coords_m.clone()),
                NanonisValue::ArrayF32(config.y_coords_m.clone()),
            ],
            vec!["I", "i", "*f", "*f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the cloud pattern parameters.
    ///
    /// # Returns
    /// A [`CloudConfig`] struct with current cloud point coordinates.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_cloud_get(&mut self) -> Result<CloudConfig, NanonisError> {
        let result = self.quick_send(
            "Pattern.CloudGet",
            vec![],
            vec![],
            vec!["i", "*f", "*f"],
        )?;

        if result.len() >= 3 {
            Ok(CloudConfig {
                x_coords_m: result[1].as_f32_array()?.to_vec(),
                y_coords_m: result[2].as_f32_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the pattern experiment properties.
    ///
    /// # Arguments
    /// * `selected_experiment` - Name of experiment to select
    /// * `basename` - Base filename for saved files
    /// * `external_vi_path` - Path to external VI
    /// * `pre_measure_delay_s` - Delay before measurement on each point
    /// * `save_scan_channels` - Save scan channels to file
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_props_set(
        &mut self,
        selected_experiment: &str,
        basename: &str,
        external_vi_path: &str,
        pre_measure_delay_s: f32,
        save_scan_channels: bool,
    ) -> Result<(), NanonisError> {
        let save_flag = if save_scan_channels { 1u32 } else { 0u32 };

        self.quick_send(
            "Pattern.PropsSet",
            vec![
                NanonisValue::String(selected_experiment.to_string()),
                NanonisValue::String(basename.to_string()),
                NanonisValue::String(external_vi_path.to_string()),
                NanonisValue::F32(pre_measure_delay_s),
                NanonisValue::U32(save_flag),
            ],
            vec!["+*c", "+*c", "+*c", "f", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the pattern experiment properties.
    ///
    /// # Returns
    /// A [`PatternProps`] struct with current experiment properties.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn pattern_props_get(&mut self) -> Result<PatternProps, NanonisError> {
        let result = self.quick_send(
            "Pattern.PropsGet",
            vec![],
            vec![],
            vec!["i", "i", "*+c", "i", "*-c", "i", "*-c", "f", "I"],
        )?;

        if result.len() >= 9 {
            Ok(PatternProps {
                available_experiments: result[2].as_string_array()?.to_vec(),
                selected_experiment: result[4].as_string()?.to_string(),
                external_vi_path: result[6].as_string()?.to_string(),
                pre_measure_delay_s: result[7].as_f32()?,
                save_scan_channels: result[8].as_u32()? != 0,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
