use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Return type for Z spectroscopy start operation (channel names, data, bias values)
pub type ZSpectroscopyResult = (Vec<String>, Vec<Vec<f32>>, Vec<f32>);

impl NanonisClient {
    /// Open the Z Spectroscopy module.
    ///
    /// Opens and initializes the Z Spectroscopy module for distance-dependent
    /// measurements. This must be called before performing spectroscopy operations.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or module cannot be opened.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Open Z spectroscopy module
    /// client.z_spectr_open()?;
    /// println!("Z Spectroscopy module opened");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("ZSpectr.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Start a Z spectroscopy measurement.
    ///
    /// Initiates a Z spectroscopy measurement with the configured parameters.
    /// The tip is moved through a range of Z positions while recording selected channels.
    ///
    /// # Arguments
    /// * `get_data` - If `true`, returns measurement data; if `false`, only starts measurement
    /// * `save_base_name` - Base filename for saving data (empty for no change)
    ///
    /// # Returns
    /// If `get_data` is true, returns a tuple containing:
    /// - `Vec<String>` - Channel names
    /// - `Vec<Vec<f32>>` - 2D measurement data \[rows\]\[columns\]
    /// - `Vec<f32>` - Fixed parameters and settings
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or measurement cannot start.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Start measurement and get data
    /// let (channels, data, params) = client.z_spectr_start(true, "approach_001")?;
    /// println!("Recorded {} channels with {} points", channels.len(), data.len());
    ///
    /// // Just start measurement without getting data
    /// let (_, _, _) = client.z_spectr_start(false, "")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_start(
        &mut self,
        get_data: bool,
        save_base_name: &str,
    ) -> Result<ZSpectroscopyResult, NanonisError> {
        let get_data_flag = if get_data { 1u32 } else { 0u32 };

        let result = self.quick_send(
            "ZSpectr.Start",
            vec![
                NanonisValue::U32(get_data_flag),
                NanonisValue::String(save_base_name.to_string()),
            ],
            vec!["I", "+*c"],
            vec!["i", "i", "*+c", "i", "i", "2f", "i", "*f"],
        )?;

        if result.len() >= 8 {
            let channel_names = result[2].as_string_array()?.to_vec();
            let rows = result[3].as_i32()? as usize;
            let cols = result[4].as_i32()? as usize;

            // Parse 2D data array
            let flat_data = result[5].as_f32_array()?;
            let mut data_2d = Vec::with_capacity(rows);
            for row in 0..rows {
                let start_idx = row * cols;
                let end_idx = start_idx + cols;
                data_2d.push(flat_data[start_idx..end_idx].to_vec());
            }

            let parameters = result[7].as_f32_array()?.to_vec();
            Ok((channel_names, data_2d, parameters))
        } else {
            Err(NanonisError::Protocol(
                "Invalid Z spectroscopy start response".to_string(),
            ))
        }
    }

    /// Stop the current Z spectroscopy measurement.
    ///
    /// Immediately stops any running Z spectroscopy measurement and returns
    /// the tip to its original position.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Start a measurement
    /// let (_, _, _) = client.z_spectr_start(false, "")?;
    ///
    /// // Stop it after some condition
    /// client.z_spectr_stop()?;
    /// println!("Z spectroscopy stopped");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_stop(&mut self) -> Result<(), NanonisError> {
        self.quick_send("ZSpectr.Stop", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get the status of Z spectroscopy measurement.
    ///
    /// Returns whether a Z spectroscopy measurement is currently running.
    ///
    /// # Returns
    /// `true` if measurement is running, `false` if stopped.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// if client.z_spectr_status_get()? {
    ///     println!("Z spectroscopy is running");
    /// } else {
    ///     println!("Z spectroscopy is stopped");
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_status_get(&mut self) -> Result<bool, NanonisError> {
        let result =
            self.quick_send("ZSpectr.StatusGet", vec![], vec![], vec!["I"])?;

        match result.first() {
            Some(value) => Ok(value.as_u32()? == 1),
            None => Err(NanonisError::Protocol(
                "No Z spectroscopy status returned".to_string(),
            )),
        }
    }

    /// Set the channels to record during Z spectroscopy.
    ///
    /// Configures which signals will be recorded during the Z spectroscopy measurement.
    /// Channel indexes correspond to the 24 signals assigned in the Signals Manager (0-23).
    ///
    /// # Arguments
    /// * `channel_indexes` - Vector of channel indexes to record (0-23)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid channel indexes provided.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Record current (0), Z position (1), and bias voltage (2)
    /// client.z_spectr_chs_set(vec![0, 1, 2])?;
    ///
    /// // Record more comprehensive dataset
    /// client.z_spectr_chs_set(vec![0, 1, 2, 3, 4, 5])?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_chs_set(
        &mut self,
        channel_indexes: Vec<i32>,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.ChsSet",
            vec![NanonisValue::ArrayI32(channel_indexes)],
            vec!["+*i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the channels configured for Z spectroscopy recording.
    ///
    /// Returns the channel indexes and names that will be recorded during measurements.
    ///
    /// # Returns
    /// A tuple containing:
    /// - `Vec<i32>` - Channel indexes (0-23 for Signals Manager slots)
    /// - `Vec<String>` - Channel names corresponding to the indexes
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let (indexes, names) = client.z_spectr_chs_get()?;
    /// println!("Recording {} channels:", indexes.len());
    /// for (idx, name) in indexes.iter().zip(names.iter()) {
    ///     println!("  Channel {}: {}", idx, name);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_chs_get(
        &mut self,
    ) -> Result<(Vec<i32>, Vec<String>), NanonisError> {
        let result = self.quick_send(
            "ZSpectr.ChsGet",
            vec![],
            vec![],
            vec!["i", "*i", "i", "i", "*+c"],
        )?;

        if result.len() >= 5 {
            let channel_indexes = result[1].as_i32_array()?.to_vec();
            let channel_names = result[4].as_string_array()?.to_vec();
            Ok((channel_indexes, channel_names))
        } else {
            Err(NanonisError::Protocol(
                "Invalid Z spectroscopy channels response".to_string(),
            ))
        }
    }

    /// Set the Z range for spectroscopy measurements.
    ///
    /// Configures the Z offset and sweep distance for the spectroscopy measurement.
    /// The tip will move from (offset - distance/2) to (offset + distance/2).
    ///
    /// # Arguments
    /// * `z_offset_m` - Z offset position in meters (center of sweep)
    /// * `z_sweep_distance_m` - Total sweep distance in meters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid range parameters.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Sweep ±5 nm around current position
    /// client.z_spectr_range_set(0.0, 10e-9)?;
    ///
    /// // Sweep from current position up to +20 nm
    /// client.z_spectr_range_set(10e-9, 20e-9)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_range_set(
        &mut self,
        z_offset_m: f32,
        z_sweep_distance_m: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.RangeSet",
            vec![
                NanonisValue::F32(z_offset_m),
                NanonisValue::F32(z_sweep_distance_m),
            ],
            vec!["f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the current Z range configuration for spectroscopy.
    ///
    /// Returns the configured Z offset and sweep distance.
    ///
    /// # Returns
    /// A tuple containing:
    /// - `f32` - Z offset in meters (center position)
    /// - `f32` - Z sweep distance in meters (total range)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let (offset, distance) = client.z_spectr_range_get()?;
    /// println!("Z sweep: {:.1} nm ± {:.1} nm", offset * 1e9, distance * 1e9 / 2.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_range_get(&mut self) -> Result<(f32, f32), NanonisError> {
        let result =
            self.quick_send("ZSpectr.RangeGet", vec![], vec![], vec!["f", "f"])?;

        if result.len() >= 2 {
            Ok((result[0].as_f32()?, result[1].as_f32()?))
        } else {
            Err(NanonisError::Protocol(
                "Invalid Z spectroscopy range response".to_string(),
            ))
        }
    }

    /// Set the timing parameters for Z spectroscopy.
    ///
    /// Configures timing-related parameters that control the speed and quality
    /// of the Z spectroscopy measurement.
    ///
    /// # Arguments
    /// * `z_averaging_time_s` - Time to average signals at each Z position
    /// * `initial_settling_time_s` - Initial settling time before measurement
    /// * `maximum_slew_rate_vdivs` - Maximum slew rate in V/s
    /// * `settling_time_s` - Settling time between measurement points
    /// * `integration_time_s` - Integration time for each measurement point
    /// * `end_settling_time_s` - Settling time at the end of sweep
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid timing parameters.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Fast spectroscopy settings
    /// client.z_spectr_timing_set(0.01, 0.1, 1000.0, 0.01, 0.01, 0.1)?;
    ///
    /// // High-quality slow spectroscopy
    /// client.z_spectr_timing_set(0.1, 0.5, 100.0, 0.05, 0.05, 0.2)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_timing_set(
        &mut self,
        z_averaging_time_s: f32,
        initial_settling_time_s: f32,
        maximum_slew_rate_vdivs: f32,
        settling_time_s: f32,
        integration_time_s: f32,
        end_settling_time_s: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.TimingSet",
            vec![
                NanonisValue::F32(z_averaging_time_s),
                NanonisValue::F32(initial_settling_time_s),
                NanonisValue::F32(maximum_slew_rate_vdivs),
                NanonisValue::F32(settling_time_s),
                NanonisValue::F32(integration_time_s),
                NanonisValue::F32(end_settling_time_s),
            ],
            vec!["f", "f", "f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the current timing parameters for Z spectroscopy.
    ///
    /// Returns all timing-related configuration parameters.
    ///
    /// # Returns
    /// A tuple containing:
    /// - `f32` - Z averaging time (s)
    /// - `f32` - Initial settling time (s)
    /// - `f32` - Maximum slew rate (V/s)
    /// - `f32` - Settling time (s)
    /// - `f32` - Integration time (s)
    /// - `f32` - End settling time (s)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let (z_avg, init_settle, slew_rate, settle, integrate, end_settle) =
    ///     client.z_spectr_timing_get()?;
    /// println!("Integration time: {:.3} s, settling: {:.3} s", integrate, settle);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_timing_get(
        &mut self,
    ) -> Result<(f32, f32, f32, f32, f32, f32), NanonisError> {
        let result = self.quick_send(
            "ZSpectr.TimingGet",
            vec![],
            vec![],
            vec!["f", "f", "f", "f", "f", "f"],
        )?;

        if result.len() >= 6 {
            Ok((
                result[0].as_f32()?,
                result[1].as_f32()?,
                result[2].as_f32()?,
                result[3].as_f32()?,
                result[4].as_f32()?,
                result[5].as_f32()?,
            ))
        } else {
            Err(NanonisError::Protocol(
                "Invalid Z spectroscopy timing response".to_string(),
            ))
        }
    }

    /// Set the retraction parameters for tip protection during Z spectroscopy.
    ///
    /// Configures automatic tip retraction based on signal thresholds to prevent
    /// tip crashes during approach spectroscopy.
    ///
    /// # Arguments
    /// * `enable` - Enable/disable automatic retraction
    /// * `threshold` - Signal threshold value for retraction trigger
    /// * `signal_index` - Index of signal to monitor (0-23)
    /// * `comparison` - Comparison type: 0=greater than, 1=less than
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid parameters.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Enable retraction when current exceeds 1 nA (signal 0, greater than)
    /// client.z_spectr_retract_set(true, 1e-9, 0, 0)?;
    ///
    /// // Disable retraction
    /// client.z_spectr_retract_set(false, 0.0, 0, 0)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_retract_set(
        &mut self,
        enable: bool,
        threshold: f32,
        signal_index: i32,
        comparison: u16,
    ) -> Result<(), NanonisError> {
        let enable_flag = if enable { 1u16 } else { 0u16 };

        self.quick_send(
            "ZSpectr.RetractSet",
            vec![
                NanonisValue::U16(enable_flag),
                NanonisValue::F32(threshold),
                NanonisValue::I32(signal_index),
                NanonisValue::U16(comparison),
            ],
            vec!["H", "f", "i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the current retraction configuration for Z spectroscopy.
    ///
    /// Returns the tip protection settings that prevent crashes during measurements.
    ///
    /// # Returns
    /// A tuple containing:
    /// - `bool` - Retraction enabled/disabled
    /// - `f32` - Threshold value for retraction
    /// - `i32` - Signal index being monitored
    /// - `u16` - Comparison type (0=greater, 1=less than)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let (enabled, threshold, signal_idx, comparison) = client.z_spectr_retract_get()?;
    /// if enabled {
    ///     let comp_str = if comparison == 0 { ">" } else { "<" };
    ///     println!("Retraction: signal[{}] {} {:.3e}", signal_idx, comp_str, threshold);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_retract_get(
        &mut self,
    ) -> Result<(bool, f32, i32, u16), NanonisError> {
        let result = self.quick_send(
            "ZSpectr.RetractGet",
            vec![],
            vec![],
            vec!["H", "f", "i", "H"],
        )?;

        if result.len() >= 4 {
            let enabled = result[0].as_u16()? == 1;
            let threshold = result[1].as_f32()?;
            let signal_index = result[2].as_i32()?;
            let comparison = result[3].as_u16()?;
            Ok((enabled, threshold, signal_index, comparison))
        } else {
            Err(NanonisError::Protocol(
                "Invalid Z spectroscopy retract response".to_string(),
            ))
        }
    }

    /// Set the Z spectroscopy properties.
    ///
    /// # Arguments
    /// * `backward_sweep` - 0=no change, 1=enable backward sweep, 2=disable
    /// * `num_points` - Number of points (0=no change)
    /// * `num_sweeps` - Number of sweeps to average (0=no change)
    /// * `autosave` - 0=no change, 1=enable autosave, 2=disable
    /// * `show_save_dialog` - 0=no change, 1=show dialog, 2=don't show
    /// * `save_all` - 0=no change, 1=save individual sweeps, 2=don't save
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_props_set(
        &mut self,
        backward_sweep: u16,
        num_points: i32,
        num_sweeps: u16,
        autosave: u16,
        show_save_dialog: u16,
        save_all: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.PropsSet",
            vec![
                NanonisValue::U16(backward_sweep),
                NanonisValue::I32(num_points),
                NanonisValue::U16(num_sweeps),
                NanonisValue::U16(autosave),
                NanonisValue::U16(show_save_dialog),
                NanonisValue::U16(save_all),
            ],
            vec!["H", "i", "H", "H", "H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Z spectroscopy properties.
    ///
    /// Returns the current property configuration.
    ///
    /// # Returns
    /// A tuple containing:
    /// - `bool` - Backward sweep enabled
    /// - `i32` - Number of points
    /// - `u16` - Number of sweeps to average
    /// - `bool` - Autosave enabled
    /// - `bool` - Show save dialog
    /// - `bool` - Save all individual sweeps
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let (backward, points, sweeps, autosave, dialog, save_all) =
    ///     client.z_spectr_props_get()?;
    /// println!("Points: {}, Sweeps: {}, Backward: {}", points, sweeps, backward);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn z_spectr_props_get(
        &mut self,
    ) -> Result<(bool, i32, u16, bool, bool, bool), NanonisError> {
        let result = self.quick_send(
            "ZSpectr.PropsGet",
            vec![],
            vec![],
            vec!["H", "i", "H", "I", "I", "I"],
        )?;

        if result.len() >= 6 {
            Ok((
                result[0].as_u16()? != 0,
                result[1].as_i32()?,
                result[2].as_u16()?,
                result[3].as_u32()? != 0,
                result[4].as_u32()? != 0,
                result[5].as_u32()? != 0,
            ))
        } else {
            Err(NanonisError::Protocol(
                "Invalid Z spectroscopy props response".to_string(),
            ))
        }
    }

    /// Set the advanced Z spectroscopy properties.
    ///
    /// # Arguments
    /// * `time_between_sweeps_s` - Time between forward and backward sweep
    /// * `record_final_z` - 0=no change, 1=on, 2=off
    /// * `lockin_run` - 0=no change, 1=on, 2=off
    /// * `reset_z` - 0=no change, 1=on, 2=off
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_adv_props_set(
        &mut self,
        time_between_sweeps_s: f32,
        record_final_z: u16,
        lockin_run: u16,
        reset_z: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.AdvPropsSet",
            vec![
                NanonisValue::F32(time_between_sweeps_s),
                NanonisValue::U16(record_final_z),
                NanonisValue::U16(lockin_run),
                NanonisValue::U16(reset_z),
            ],
            vec!["f", "H", "H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the advanced Z spectroscopy properties.
    ///
    /// # Returns
    /// Tuple of (time_between_sweeps, record_final_z, lockin_run, reset_z).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_adv_props_get(&mut self) -> Result<(f32, bool, bool, bool), NanonisError> {
        let result = self.quick_send(
            "ZSpectr.AdvPropsGet",
            vec![],
            vec![],
            vec!["f", "H", "H", "H"],
        )?;

        Ok((
            result[0].as_f32()?,
            result[1].as_u16()? != 0,
            result[2].as_u16()? != 0,
            result[3].as_u16()? != 0,
        ))
    }

    /// Set the retract delay.
    ///
    /// # Arguments
    /// * `delay_s` - Delay in seconds between forward and backward sweep
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_retract_delay_set(&mut self, delay_s: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.RetractDelaySet",
            vec![NanonisValue::F32(delay_s)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the retract delay.
    ///
    /// # Returns
    /// Delay in seconds.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_retract_delay_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("ZSpectr.RetractDelayGet", vec![], vec![], vec!["f"])?;
        result[0].as_f32()
    }

    /// Set the second retraction condition.
    ///
    /// # Arguments
    /// * `condition` - 0=no change, 1=disabled, 2=OR, 3=AND, 4=THEN
    /// * `threshold` - Threshold value
    /// * `signal_index` - Signal index (0-127, -1 for no change)
    /// * `comparison` - 0=greater than, 1=less than, 2=no change
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_retract_second_set(
        &mut self,
        condition: i32,
        threshold: f32,
        signal_index: i32,
        comparison: u16,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.RetractSecondSet",
            vec![
                NanonisValue::I32(condition),
                NanonisValue::F32(threshold),
                NanonisValue::I32(signal_index),
                NanonisValue::U16(comparison),
            ],
            vec!["i", "f", "i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the second retraction condition.
    ///
    /// # Returns
    /// Tuple of (condition, threshold, signal_index, comparison).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_retract_second_get(&mut self) -> Result<(i32, f32, i32, u16), NanonisError> {
        let result = self.quick_send(
            "ZSpectr.RetractSecondGet",
            vec![],
            vec![],
            vec!["i", "f", "i", "H"],
        )?;

        Ok((
            result[0].as_i32()?,
            result[1].as_f32()?,
            result[2].as_i32()?,
            result[3].as_u16()?,
        ))
    }

    /// Set the digital synchronization mode.
    ///
    /// # Arguments
    /// * `dig_sync` - 0=no change, 1=off, 2=TTL sync, 3=pulse sequence
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_dig_sync_set(&mut self, dig_sync: u16) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.DigSyncSet",
            vec![NanonisValue::U16(dig_sync)],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the digital synchronization mode.
    ///
    /// # Returns
    /// Sync mode (0=off, 1=TTL sync, 2=pulse sequence).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_dig_sync_get(&mut self) -> Result<u16, NanonisError> {
        let result = self.quick_send("ZSpectr.DigSyncGet", vec![], vec![], vec!["H"])?;
        result[0].as_u16()
    }

    /// Set the TTL synchronization parameters.
    ///
    /// # Arguments
    /// * `ttl_line` - 0=no change, 1-4=HS line number
    /// * `polarity` - 0=no change, 1=low active, 2=high active
    /// * `time_to_on_s` - Time to wait before activation
    /// * `on_duration_s` - Duration of activation
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_ttl_sync_set(
        &mut self,
        ttl_line: u16,
        polarity: u16,
        time_to_on_s: f32,
        on_duration_s: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.TTLSyncSet",
            vec![
                NanonisValue::U16(ttl_line),
                NanonisValue::U16(polarity),
                NanonisValue::F32(time_to_on_s),
                NanonisValue::F32(on_duration_s),
            ],
            vec!["H", "H", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the TTL synchronization parameters.
    ///
    /// # Returns
    /// Tuple of (ttl_line, polarity, time_to_on_s, on_duration_s).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_ttl_sync_get(&mut self) -> Result<(u16, u16, f32, f32), NanonisError> {
        let result = self.quick_send(
            "ZSpectr.TTLSyncGet",
            vec![],
            vec![],
            vec!["H", "H", "f", "f"],
        )?;

        Ok((
            result[0].as_u16()?,
            result[1].as_u16()?,
            result[2].as_f32()?,
            result[3].as_f32()?,
        ))
    }

    /// Set the pulse sequence synchronization.
    ///
    /// # Arguments
    /// * `pulse_seq_nr` - Pulse sequence number (0=no change)
    /// * `num_periods` - Number of periods
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_pulse_seq_sync_set(
        &mut self,
        pulse_seq_nr: u16,
        num_periods: u32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "ZSpectr.PulseSeqSyncSet",
            vec![
                NanonisValue::U16(pulse_seq_nr),
                NanonisValue::U32(num_periods),
            ],
            vec!["H", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the pulse sequence synchronization.
    ///
    /// # Returns
    /// Tuple of (pulse_seq_nr, num_periods).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn z_spectr_pulse_seq_sync_get(&mut self) -> Result<(u16, u32), NanonisError> {
        let result = self.quick_send(
            "ZSpectr.PulseSeqSyncGet",
            vec![],
            vec![],
            vec!["H", "I"],
        )?;

        Ok((result[0].as_u16()?, result[1].as_u32()?))
    }
}
