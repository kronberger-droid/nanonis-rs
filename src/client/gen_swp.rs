use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Generic sweeper properties configuration.
#[derive(Debug, Clone)]
pub struct GenSwpProps {
    /// Initial settling time in milliseconds
    pub initial_settling_time_ms: f32,
    /// Maximum slew rate in units/s
    pub max_slew_rate: f32,
    /// Number of sweep steps
    pub num_steps: i32,
    /// Period in milliseconds
    pub period_ms: u16,
    /// Autosave enabled
    pub autosave: bool,
    /// Show save dialog
    pub save_dialog: bool,
    /// Settling time in milliseconds
    pub settling_time_ms: f32,
}

impl Default for GenSwpProps {
    fn default() -> Self {
        Self {
            initial_settling_time_ms: 100.0,
            max_slew_rate: 1.0,
            num_steps: 100,
            period_ms: 50,
            autosave: true,
            save_dialog: false,
            settling_time_ms: 10.0,
        }
    }
}

/// Result data from a generic sweep measurement.
#[derive(Debug, Clone)]
pub struct GenSwpResult {
    /// Names of recorded channels
    pub channel_names: Vec<String>,
    /// 2D data array `[rows][columns]`
    pub data: Vec<Vec<f32>>,
}

impl NanonisClient {
    /// Open the Generic Sweeper module.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.gen_swp_open()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gen_swp_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("GenSwp.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set the list of recorded channels for the Generic Sweeper.
    ///
    /// # Arguments
    /// * `channel_indexes` - Indexes of channels to record
    /// * `channel_names` - Names of channels to record
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_swp_acq_chs_set(
        &mut self,
        channel_indexes: &[i32],
        channel_names: &[String],
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "GenSwp.AcqChsSet",
            vec![
                NanonisValue::ArrayI32(channel_indexes.to_vec()),
                NanonisValue::ArrayString(channel_names.to_vec()),
            ],
            vec!["+*i", "*+c"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the list of recorded channels for the Generic Sweeper.
    ///
    /// # Returns
    /// A tuple of (channel_indexes, channel_names).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_swp_acq_chs_get(&mut self) -> Result<(Vec<i32>, Vec<String>), NanonisError> {
        let result = self.quick_send(
            "GenSwp.AcqChsGet",
            vec![],
            vec![],
            vec!["i", "*i", "i", "i", "*+c"],
        )?;

        if result.len() >= 5 {
            let indexes = result[1].as_i32_array()?.to_vec();
            let names = result[4].as_string_array()?.to_vec();
            Ok((indexes, names))
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the sweep signal for the Generic Sweeper.
    ///
    /// # Arguments
    /// * `signal_name` - Name of the signal to sweep
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.gen_swp_swp_signal_set("Bias (V)")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gen_swp_swp_signal_set(&mut self, signal_name: &str) -> Result<(), NanonisError> {
        self.quick_send(
            "GenSwp.SwpSignalSet",
            vec![NanonisValue::String(signal_name.to_string())],
            vec!["+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the selected sweep signal for the Generic Sweeper.
    ///
    /// # Returns
    /// The name of the sweep signal.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_swp_swp_signal_get(&mut self) -> Result<String, NanonisError> {
        let result = self.quick_send("GenSwp.SwpSignalGet", vec![], vec![], vec!["i", "*-c"])?;

        if result.len() >= 2 {
            Ok(result[1].as_string()?.to_string())
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get the list of available sweep signals for the Generic Sweeper.
    ///
    /// # Returns
    /// A vector of available signal names.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_swp_swp_signal_list_get(&mut self) -> Result<Vec<String>, NanonisError> {
        let result =
            self.quick_send("GenSwp.SwpSignalListGet", vec![], vec![], vec!["i", "i", "*+c"])?;

        if result.len() >= 3 {
            Ok(result[2].as_string_array()?.to_vec())
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the sweep limits for the Generic Sweeper.
    ///
    /// # Arguments
    /// * `lower_limit` - Lower limit of sweep range
    /// * `upper_limit` - Upper limit of sweep range
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_swp_limits_set(
        &mut self,
        lower_limit: f32,
        upper_limit: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "GenSwp.LimitsSet",
            vec![
                NanonisValue::F32(lower_limit),
                NanonisValue::F32(upper_limit),
            ],
            vec!["f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the sweep limits for the Generic Sweeper.
    ///
    /// # Returns
    /// A tuple of (lower_limit, upper_limit).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_swp_limits_get(&mut self) -> Result<(f32, f32), NanonisError> {
        let result = self.quick_send("GenSwp.LimitsGet", vec![], vec![], vec!["f", "f"])?;

        if result.len() >= 2 {
            Ok((result[0].as_f32()?, result[1].as_f32()?))
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Generic Sweeper properties.
    ///
    /// # Arguments
    /// * `props` - A [`GenSwpProps`] struct with configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::gen_swp::GenSwpProps;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let props = GenSwpProps {
    ///     num_steps: 200,
    ///     period_ms: 100,
    ///     ..Default::default()
    /// };
    /// client.gen_swp_props_set(&props)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gen_swp_props_set(&mut self, props: &GenSwpProps) -> Result<(), NanonisError> {
        let autosave_flag = if props.autosave { 1i32 } else { 2i32 };
        let dialog_flag = if props.save_dialog { 1i32 } else { 2i32 };

        self.quick_send(
            "GenSwp.PropsSet",
            vec![
                NanonisValue::F32(props.initial_settling_time_ms),
                NanonisValue::F32(props.max_slew_rate),
                NanonisValue::I32(props.num_steps),
                NanonisValue::U16(props.period_ms),
                NanonisValue::I32(autosave_flag),
                NanonisValue::I32(dialog_flag),
                NanonisValue::F32(props.settling_time_ms),
            ],
            vec!["f", "f", "i", "H", "i", "i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Generic Sweeper properties.
    ///
    /// # Returns
    /// A [`GenSwpProps`] struct with current configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_swp_props_get(&mut self) -> Result<GenSwpProps, NanonisError> {
        let result = self.quick_send(
            "GenSwp.PropsGet",
            vec![],
            vec![],
            vec!["f", "f", "i", "H", "I", "I", "f"],
        )?;

        if result.len() >= 7 {
            Ok(GenSwpProps {
                initial_settling_time_ms: result[0].as_f32()?,
                max_slew_rate: result[1].as_f32()?,
                num_steps: result[2].as_i32()?,
                period_ms: result[3].as_u16()?,
                autosave: result[4].as_u32()? != 0,
                save_dialog: result[5].as_u32()? != 0,
                settling_time_ms: result[6].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Start a sweep in the Generic Sweeper.
    ///
    /// # Arguments
    /// * `get_data` - If true, returns measurement data
    /// * `sweep_direction` - `true` = lower to upper, `false` = upper to lower
    /// * `save_base_name` - Base filename for saving (empty for no change)
    /// * `reset_signal` - Reset signal after sweep
    /// * `z_controller` - Z-controller behavior: 0=no change, 1=turn off, 2=don't turn off
    ///
    /// # Returns
    /// A [`GenSwpResult`] with channel names and 2D data.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let result = client.gen_swp_start(true, true, "sweep_001", false, 0)?;
    /// println!("Channels: {:?}", result.channel_names);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gen_swp_start(
        &mut self,
        get_data: bool,
        sweep_direction: bool,
        save_base_name: &str,
        reset_signal: bool,
        z_controller: u16,
    ) -> Result<GenSwpResult, NanonisError> {
        let get_data_flag = if get_data { 1u32 } else { 0u32 };
        let direction_flag = if sweep_direction { 1u32 } else { 0u32 };
        let reset_flag = if reset_signal { 1u32 } else { 0u32 };

        let result = self.quick_send(
            "GenSwp.Start",
            vec![
                NanonisValue::U32(get_data_flag),
                NanonisValue::U32(direction_flag),
                NanonisValue::String(save_base_name.to_string()),
                NanonisValue::U32(reset_flag),
                NanonisValue::U16(z_controller),
            ],
            vec!["I", "I", "+*c", "I", "H"],
            vec!["i", "i", "*+c", "i", "i", "2f"],
        )?;

        if result.len() >= 6 {
            let channel_names = result[2].as_string_array()?.to_vec();
            let rows = result[3].as_i32()? as usize;
            let cols = result[4].as_i32()? as usize;

            let flat_data = result[5].as_f32_array()?;
            let mut data_2d = Vec::with_capacity(rows);
            for row in 0..rows {
                let start_idx = row * cols;
                let end_idx = start_idx + cols;
                if end_idx <= flat_data.len() {
                    data_2d.push(flat_data[start_idx..end_idx].to_vec());
                }
            }

            Ok(GenSwpResult {
                channel_names,
                data: data_2d,
            })
        } else {
            Ok(GenSwpResult {
                channel_names: vec![],
                data: vec![],
            })
        }
    }

    /// Stop the sweep in the Generic Sweeper.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn gen_swp_stop(&mut self) -> Result<(), NanonisError> {
        self.quick_send("GenSwp.Stop", vec![], vec![], vec![])?;
        Ok(())
    }
}
