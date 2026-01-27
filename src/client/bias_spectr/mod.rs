mod types;
pub use types::*;

use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;
use std::time::Duration;

impl NanonisClient {
    /// Open the Bias Spectroscopy module.
    ///
    /// Opens and initializes the Bias Spectroscopy module for STS measurements.
    /// This must be called before performing spectroscopy operations.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or module cannot be opened.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.bias_spectr_open()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("BiasSpectr.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Start a bias spectroscopy measurement.
    ///
    /// Starts a bias spectroscopy (STS) measurement with configured parameters.
    /// Select channels to record before calling this function.
    ///
    /// # Arguments
    /// * `get_data` - If true, returns measurement data
    /// * `save_base_name` - Base filename for saving (empty for no change)
    ///
    /// # Returns
    /// A [`BiasSpectrResult`] containing channel names, 2D data, and parameters.
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
    /// // Start spectroscopy and get data
    /// let result = client.bias_spectr_start(true, "sts_001")?;
    /// println!("Recorded {} channels", result.channel_names.len());
    /// println!("Data shape: {} x {}", result.data.len(),
    ///          result.data.first().map(|r| r.len()).unwrap_or(0));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_start(
        &mut self,
        get_data: bool,
        save_base_name: &str,
    ) -> Result<BiasSpectrResult, NanonisError> {
        let get_data_flag = if get_data { 1u32 } else { 0u32 };

        let result = self.quick_send(
            "BiasSpectr.Start",
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
                if end_idx <= flat_data.len() {
                    data_2d.push(flat_data[start_idx..end_idx].to_vec());
                }
            }

            let parameters = result[7].as_f32_array()?.to_vec();

            Ok(BiasSpectrResult {
                channel_names,
                data: data_2d,
                parameters,
            })
        } else {
            Ok(BiasSpectrResult {
                channel_names: vec![],
                data: vec![],
                parameters: vec![],
            })
        }
    }

    /// Stop the current bias spectroscopy measurement.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.bias_spectr_stop()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_stop(&mut self) -> Result<(), NanonisError> {
        self.quick_send("BiasSpectr.Stop", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get the status of the bias spectroscopy measurement.
    ///
    /// # Returns
    /// `true` if measurement is running, `false` otherwise.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// if client.bias_spectr_status_get()? {
    ///     println!("Spectroscopy is running");
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_status_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("BiasSpectr.StatusGet", vec![], vec![], vec!["I"])?;
        if let Some(val) = result.first() {
            Ok(val.as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol(
                "Invalid status response".to_string(),
            ))
        }
    }

    /// Set the list of recorded channels in bias spectroscopy.
    ///
    /// # Arguments
    /// * `channel_indexes` - Indexes of channels to record (0-23 for signals in Signals Manager)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid indexes provided.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// // Record channels 0, 1, and 5
    /// client.bias_spectr_chs_set(&[0, 1, 5])?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_chs_set(&mut self, channel_indexes: &[i32]) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.ChsSet",
            vec![NanonisValue::ArrayI32(channel_indexes.to_vec())],
            vec!["+*i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the list of recorded channels in bias spectroscopy.
    ///
    /// # Returns
    /// A tuple containing:
    /// - `Vec<i32>` - Indexes of recorded channels
    /// - `Vec<String>` - Names of recorded channels
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let (indexes, names) = client.bias_spectr_chs_get()?;
    /// for (idx, name) in indexes.iter().zip(names.iter()) {
    ///     println!("Channel {}: {}", idx, name);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_chs_get(&mut self) -> Result<(Vec<i32>, Vec<String>), NanonisError> {
        let result = self.quick_send(
            "BiasSpectr.ChsGet",
            vec![],
            vec![],
            vec!["i", "*i", "i", "i", "*+c"],
        )?;

        if result.len() >= 5 {
            let indexes = result[1].as_i32_array()?.to_vec();
            let names = result[4].as_string_array()?.to_vec();
            Ok((indexes, names))
        } else {
            Err(NanonisError::Protocol(
                "Invalid channels response".to_string(),
            ))
        }
    }

    /// Set the bias spectroscopy properties.
    ///
    /// Uses a builder pattern to configure only the properties you want to change.
    /// Properties not set in the builder will remain unchanged on the instrument.
    ///
    /// # Arguments
    /// * `config` - Configuration builder with properties to set
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::{BiasSpectrPropsBuilder, OptionalFlag};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Configure using builder pattern - only set what you need
    /// let config = BiasSpectrPropsBuilder::new()
    ///     .num_sweeps(10)
    ///     .num_points(200)
    ///     .backward_sweep(OptionalFlag::On)
    ///     .autosave(OptionalFlag::On);
    ///
    /// client.bias_spectr_props_set(config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_props_set(
        &mut self,
        config: BiasSpectrPropsBuilder,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.PropsSet",
            vec![
                NanonisValue::U16(config.save_all.into()),
                NanonisValue::I32(config.num_sweeps),
                NanonisValue::U16(config.backward_sweep.into()),
                NanonisValue::I32(config.num_points),
                NanonisValue::F32(config.z_offset_m),
                NanonisValue::U16(config.autosave.into()),
                NanonisValue::U16(config.show_save_dialog.into()),
            ],
            vec!["H", "i", "H", "i", "f", "H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the bias spectroscopy properties.
    ///
    /// # Returns
    /// A [`BiasSpectrProps`] struct with current configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let props = client.bias_spectr_props_get()?;
    /// println!("Sweeps: {}, Points: {}", props.num_sweeps, props.num_points);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_props_get(&mut self) -> Result<BiasSpectrProps, NanonisError> {
        let result = self.quick_send(
            "BiasSpectr.PropsGet",
            vec![],
            vec![],
            vec![
                "H", "i", "H", "i", "i", "i", "*+c", "i", "i", "*+c", "i", "i", "*+c",
            ],
        )?;

        if result.len() >= 13 {
            Ok(BiasSpectrProps {
                save_all: result[0].as_u16()? != 0,
                num_sweeps: result[1].as_i32()?,
                backward_sweep: result[2].as_u16()? != 0,
                num_points: result[3].as_i32()?,
                channels: result[6].as_string_array()?.to_vec(),
                parameters: result[9].as_string_array()?.to_vec(),
                fixed_parameters: result[12].as_string_array()?.to_vec(),
            })
        } else {
            Err(NanonisError::Protocol(
                "Invalid props response".to_string(),
            ))
        }
    }

    /// Set the advanced bias spectroscopy properties.
    ///
    /// # Arguments
    /// * `reset_bias` - Reset bias to initial value after sweep: NoChange/On/Off
    /// * `z_controller_hold` - Hold Z-controller during sweep: NoChange/On/Off
    /// * `record_final_z` - Record final Z position: NoChange/On/Off
    /// * `lockin_run` - Run lock-in during measurement: NoChange/On/Off
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::OptionalFlag;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.bias_spectr_adv_props_set(
    ///     OptionalFlag::On,   // reset_bias
    ///     OptionalFlag::On,   // z_controller_hold
    ///     OptionalFlag::Off,  // record_final_z
    ///     OptionalFlag::Off,  // lockin_run
    /// )?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_adv_props_set(
        &mut self,
        reset_bias: OptionalFlag,
        z_controller_hold: OptionalFlag,
        record_final_z: OptionalFlag,
        lockin_run: OptionalFlag,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.AdvPropsSet",
            vec![
                NanonisValue::U16(reset_bias.into()),
                NanonisValue::U16(z_controller_hold.into()),
                NanonisValue::U16(record_final_z.into()),
                NanonisValue::U16(lockin_run.into()),
            ],
            vec!["H", "H", "H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the advanced bias spectroscopy properties.
    ///
    /// # Returns
    /// A [`BiasSpectrAdvProps`] struct with current advanced settings.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let adv = client.bias_spectr_adv_props_get()?;
    /// println!("Z-controller hold: {}", adv.z_controller_hold);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_adv_props_get(&mut self) -> Result<BiasSpectrAdvProps, NanonisError> {
        let result = self.quick_send(
            "BiasSpectr.AdvPropsGet",
            vec![],
            vec![],
            vec!["H", "H", "H", "H"],
        )?;

        if result.len() >= 4 {
            Ok(BiasSpectrAdvProps {
                reset_bias: result[0].as_u16()? != 0,
                z_controller_hold: result[1].as_u16()? != 0,
                record_final_z: result[2].as_u16()? != 0,
                lockin_run: result[3].as_u16()? != 0,
            })
        } else {
            Err(NanonisError::Protocol(
                "Invalid adv props response".to_string(),
            ))
        }
    }

    /// Set the bias spectroscopy sweep limits.
    ///
    /// # Arguments
    /// * `start_value_v` - Starting bias voltage in volts
    /// * `end_value_v` - Ending bias voltage in volts
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// // Sweep from -2V to +2V
    /// client.bias_spectr_limits_set(-2.0, 2.0)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_limits_set(
        &mut self,
        start_value_v: f32,
        end_value_v: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.LimitsSet",
            vec![
                NanonisValue::F32(start_value_v),
                NanonisValue::F32(end_value_v),
            ],
            vec!["f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the bias spectroscopy sweep limits.
    ///
    /// # Returns
    /// A tuple `(start_v, end_v)` with the voltage limits.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let (start, end) = client.bias_spectr_limits_get()?;
    /// println!("Sweep range: {:.2}V to {:.2}V", start, end);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_limits_get(&mut self) -> Result<(f32, f32), NanonisError> {
        let result = self.quick_send("BiasSpectr.LimitsGet", vec![], vec![], vec!["f", "f"])?;

        if result.len() >= 2 {
            Ok((result[0].as_f32()?, result[1].as_f32()?))
        } else {
            Err(NanonisError::Protocol(
                "Invalid limits response".to_string(),
            ))
        }
    }

    /// Set the bias spectroscopy timing parameters.
    ///
    /// # Arguments
    /// * `timing` - A [`BiasSpectrTiming`] struct with timing configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::BiasSpectrTiming;
    /// use std::time::Duration;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let timing = BiasSpectrTiming {
    ///     settling_time: Duration::from_millis(20),
    ///     integration_time: Duration::from_millis(50),
    ///     ..Default::default()
    /// };
    /// client.bias_spectr_timing_set(&timing)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_timing_set(&mut self, timing: &BiasSpectrTiming) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.TimingSet",
            vec![
                NanonisValue::F32(timing.z_averaging_time.as_secs_f32()),
                NanonisValue::F32(timing.z_offset_m),
                NanonisValue::F32(timing.initial_settling_time.as_secs_f32()),
                NanonisValue::F32(timing.max_slew_rate),
                NanonisValue::F32(timing.settling_time.as_secs_f32()),
                NanonisValue::F32(timing.integration_time.as_secs_f32()),
                NanonisValue::F32(timing.end_settling_time.as_secs_f32()),
                NanonisValue::F32(timing.z_control_time.as_secs_f32()),
            ],
            vec!["f", "f", "f", "f", "f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the bias spectroscopy timing parameters.
    ///
    /// # Returns
    /// A [`BiasSpectrTiming`] struct with current timing settings.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let timing = client.bias_spectr_timing_get()?;
    /// println!("Integration time: {:?}", timing.integration_time);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_timing_get(&mut self) -> Result<BiasSpectrTiming, NanonisError> {
        let result = self.quick_send(
            "BiasSpectr.TimingGet",
            vec![],
            vec![],
            vec!["f", "f", "f", "f", "f", "f", "f", "f"],
        )?;

        if result.len() >= 8 {
            Ok(BiasSpectrTiming {
                z_averaging_time: Duration::from_secs_f32(result[0].as_f32()?),
                z_offset_m: result[1].as_f32()?,
                initial_settling_time: Duration::from_secs_f32(result[2].as_f32()?),
                max_slew_rate: result[3].as_f32()?,
                settling_time: Duration::from_secs_f32(result[4].as_f32()?),
                integration_time: Duration::from_secs_f32(result[5].as_f32()?),
                end_settling_time: Duration::from_secs_f32(result[6].as_f32()?),
                z_control_time: Duration::from_secs_f32(result[7].as_f32()?),
            })
        } else {
            Err(NanonisError::Protocol(
                "Invalid timing response".to_string(),
            ))
        }
    }

    /// Set the digital synchronization mode.
    ///
    /// # Arguments
    /// * `mode` - The [`DigitalSync`] mode to set
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::DigitalSync;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.bias_spectr_dig_sync_set(DigitalSync::TTLSync)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_dig_sync_set(&mut self, mode: DigitalSync) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.DigSyncSet",
            vec![NanonisValue::U16(mode.into())],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the digital synchronization mode.
    ///
    /// # Returns
    /// The current [`DigitalSync`] mode.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let mode = client.bias_spectr_dig_sync_get()?;
    /// println!("Digital sync mode: {:?}", mode);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_dig_sync_get(&mut self) -> Result<DigitalSync, NanonisError> {
        let result = self.quick_send("BiasSpectr.DigSyncGet", vec![], vec![], vec!["H"])?;

        if let Some(val) = result.first() {
            DigitalSync::try_from(val.as_u16()?)
        } else {
            Err(NanonisError::Protocol(
                "Invalid dig sync response".to_string(),
            ))
        }
    }

    /// Set the TTL synchronization configuration.
    ///
    /// # Arguments
    /// * `config` - A [`TTLSyncConfig`] struct with TTL settings
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::{TTLSyncConfig, TTLLine, TTLPolarity};
    /// use std::time::Duration;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = TTLSyncConfig {
    ///     line: TTLLine::HSLine1,
    ///     polarity: TTLPolarity::HighActive,
    ///     time_to_on: Duration::from_millis(10),
    ///     on_duration: Duration::from_millis(100),
    /// };
    /// client.bias_spectr_ttl_sync_set(&config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_ttl_sync_set(&mut self, config: &TTLSyncConfig) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.TTLSyncSet",
            vec![
                NanonisValue::U16(config.line.into()),
                NanonisValue::U16(config.polarity.into()),
                NanonisValue::F32(config.time_to_on.as_secs_f32()),
                NanonisValue::F32(config.on_duration.as_secs_f32()),
            ],
            vec!["H", "H", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the TTL synchronization configuration.
    ///
    /// # Returns
    /// A [`TTLSyncConfig`] struct with current TTL settings.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = client.bias_spectr_ttl_sync_get()?;
    /// println!("TTL line: {:?}, on duration: {:?}", config.line, config.on_duration);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_ttl_sync_get(&mut self) -> Result<TTLSyncConfig, NanonisError> {
        let result = self.quick_send(
            "BiasSpectr.TTLSyncGet",
            vec![],
            vec![],
            vec!["H", "H", "f", "f"],
        )?;

        if result.len() >= 4 {
            Ok(TTLSyncConfig {
                line: TTLLine::try_from(result[0].as_u16()?)?,
                polarity: TTLPolarity::try_from(result[1].as_u16()?)?,
                time_to_on: Duration::from_secs_f32(result[2].as_f32()?),
                on_duration: Duration::from_secs_f32(result[3].as_f32()?),
            })
        } else {
            Err(NanonisError::Protocol(
                "Invalid TTL sync response".to_string(),
            ))
        }
    }

    /// Set the pulse sequence synchronization configuration.
    ///
    /// # Arguments
    /// * `config` - A [`PulseSeqSyncConfig`] struct with pulse sequence settings
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::PulseSeqSyncConfig;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = PulseSeqSyncConfig {
    ///     sequence_nr: 1,
    ///     num_periods: 10,
    /// };
    /// client.bias_spectr_pulse_seq_sync_set(&config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_pulse_seq_sync_set(
        &mut self,
        config: &PulseSeqSyncConfig,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.PulseSeqSyncSet",
            vec![
                NanonisValue::U16(config.sequence_nr),
                NanonisValue::U32(config.num_periods),
            ],
            vec!["H", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the pulse sequence synchronization configuration.
    ///
    /// # Returns
    /// A [`PulseSeqSyncConfig`] struct with current settings.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = client.bias_spectr_pulse_seq_sync_get()?;
    /// println!("Sequence #{}, {} periods", config.sequence_nr, config.num_periods);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_pulse_seq_sync_get(&mut self) -> Result<PulseSeqSyncConfig, NanonisError> {
        let result =
            self.quick_send("BiasSpectr.PulseSeqSyncGet", vec![], vec![], vec!["H", "I"])?;

        if result.len() >= 2 {
            Ok(PulseSeqSyncConfig {
                sequence_nr: result[0].as_u16()?,
                num_periods: result[1].as_u32()?,
            })
        } else {
            Err(NanonisError::Protocol(
                "Invalid pulse seq sync response".to_string(),
            ))
        }
    }

    /// Set the alternate Z-controller setpoint configuration.
    ///
    /// # Arguments
    /// * `config` - An [`AltZCtrlConfig`] struct with alternate setpoint settings
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::AltZCtrlConfig;
    /// use std::time::Duration;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = AltZCtrlConfig {
    ///     enabled: true,
    ///     setpoint: 1e-9,  // 1 nA
    ///     settling_time: Duration::from_millis(200),
    /// };
    /// client.bias_spectr_alt_z_ctrl_set(&config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_alt_z_ctrl_set(&mut self, config: &AltZCtrlConfig) -> Result<(), NanonisError> {
        let enabled_flag = if config.enabled {
            OptionalFlag::On
        } else {
            OptionalFlag::Off
        };

        self.quick_send(
            "BiasSpectr.AltZCtrlSet",
            vec![
                NanonisValue::U16(enabled_flag.into()),
                NanonisValue::F32(config.setpoint),
                NanonisValue::F32(config.settling_time.as_secs_f32()),
            ],
            vec!["H", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the alternate Z-controller setpoint configuration.
    ///
    /// # Returns
    /// An [`AltZCtrlConfig`] struct with current settings.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = client.bias_spectr_alt_z_ctrl_get()?;
    /// println!("Alt Z-ctrl enabled: {}, setpoint: {}", config.enabled, config.setpoint);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_alt_z_ctrl_get(&mut self) -> Result<AltZCtrlConfig, NanonisError> {
        let result =
            self.quick_send("BiasSpectr.AltZCtrlGet", vec![], vec![], vec!["H", "f", "f"])?;

        if result.len() >= 3 {
            Ok(AltZCtrlConfig {
                enabled: result[0].as_u16()? != 0,
                setpoint: result[1].as_f32()?,
                settling_time: Duration::from_secs_f32(result[2].as_f32()?),
            })
        } else {
            Err(NanonisError::Protocol(
                "Invalid alt z ctrl response".to_string(),
            ))
        }
    }

    /// Set the Z offset revert flag.
    ///
    /// When enabled, the Z offset applied at the beginning is reverted at the end.
    ///
    /// # Arguments
    /// * `revert` - Whether to revert Z offset: NoChange/On/Off
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::OptionalFlag;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.bias_spectr_z_off_revert_set(OptionalFlag::On)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_z_off_revert_set(&mut self, revert: OptionalFlag) -> Result<(), NanonisError> {
        self.quick_send(
            "BiasSpectr.ZOffRevertSet",
            vec![NanonisValue::U16(revert.into())],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Z offset revert flag.
    ///
    /// # Returns
    /// `true` if Z offset revert is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let revert = client.bias_spectr_z_off_revert_get()?;
    /// println!("Z offset revert: {}", revert);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_z_off_revert_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("BiasSpectr.ZOffRevertGet", vec![], vec![], vec!["H"])?;

        if let Some(val) = result.first() {
            Ok(val.as_u16()? != 0)
        } else {
            Err(NanonisError::Protocol(
                "Invalid z off revert response".to_string(),
            ))
        }
    }

    /// Set the MLS lock-in per segment flag.
    ///
    /// When enabled, lock-in can be configured per segment in the MLS editor.
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable per-segment lock-in configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.bias_spectr_mls_lockin_per_seg_set(true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_mls_lockin_per_seg_set(&mut self, enabled: bool) -> Result<(), NanonisError> {
        let flag = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "BiasSpectr.MLSLockinPerSegSet",
            vec![NanonisValue::U32(flag)],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the MLS lock-in per segment flag.
    ///
    /// # Returns
    /// `true` if per-segment lock-in configuration is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let enabled = client.bias_spectr_mls_lockin_per_seg_get()?;
    /// println!("MLS lock-in per segment: {}", enabled);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_mls_lockin_per_seg_get(&mut self) -> Result<bool, NanonisError> {
        let result =
            self.quick_send("BiasSpectr.MLSLockinPerSegGet", vec![], vec![], vec!["I"])?;

        if let Some(val) = result.first() {
            Ok(val.as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol(
                "Invalid MLS lockin per seg response".to_string(),
            ))
        }
    }

    /// Set the MLS sweep mode.
    ///
    /// # Arguments
    /// * `mode` - The [`SweepMode`] to set
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::SweepMode;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.bias_spectr_mls_mode_set(SweepMode::MLS)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_mls_mode_set(&mut self, mode: SweepMode) -> Result<(), NanonisError> {
        let mode_str: &str = mode.into();
        self.quick_send(
            "BiasSpectr.MLSModeSet",
            vec![NanonisValue::String(mode_str.to_string())],
            vec!["+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the current MLS sweep mode.
    ///
    /// # Returns
    /// The current [`SweepMode`].
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let mode = client.bias_spectr_mls_mode_get()?;
    /// println!("Sweep mode: {:?}", mode);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_mls_mode_get(&mut self) -> Result<SweepMode, NanonisError> {
        let result = self.quick_send(
            "BiasSpectr.MLSModeGet",
            vec![],
            vec![],
            vec!["i", "i", "*+c"],
        )?;

        if result.len() >= 3 {
            let modes = result[2].as_string_array()?;
            if let Some(mode_str) = modes.first() {
                SweepMode::try_from(mode_str.as_str())
            } else {
                Ok(SweepMode::Linear)
            }
        } else {
            Err(NanonisError::Protocol(
                "Invalid MLS mode response".to_string(),
            ))
        }
    }

    /// Set the MLS segment values.
    ///
    /// # Arguments
    /// * `segments` - Vector of [`MLSSegment`] configurations
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::bias_spectr::MLSSegment;
    /// use std::time::Duration;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let segments = vec![
    ///     MLSSegment {
    ///         bias_start: -2.0,
    ///         bias_end: 0.0,
    ///         steps: 100,
    ///         ..Default::default()
    ///     },
    ///     MLSSegment {
    ///         bias_start: 0.0,
    ///         bias_end: 2.0,
    ///         steps: 100,
    ///         ..Default::default()
    ///     },
    /// ];
    /// client.bias_spectr_mls_vals_set(&segments)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_mls_vals_set(&mut self, segments: &[MLSSegment]) -> Result<(), NanonisError> {
        let num_segments = segments.len() as i32;
        let bias_start: Vec<f32> = segments.iter().map(|s| s.bias_start).collect();
        let bias_end: Vec<f32> = segments.iter().map(|s| s.bias_end).collect();
        let initial_settling: Vec<f32> = segments
            .iter()
            .map(|s| s.initial_settling_time.as_secs_f32())
            .collect();
        let settling: Vec<f32> = segments
            .iter()
            .map(|s| s.settling_time.as_secs_f32())
            .collect();
        let integration: Vec<f32> = segments
            .iter()
            .map(|s| s.integration_time.as_secs_f32())
            .collect();
        let slew_rate: Vec<f32> = segments.iter().map(|s| s.max_slew_rate).collect();
        let steps: Vec<i32> = segments.iter().map(|s| s.steps).collect();

        self.quick_send(
            "BiasSpectr.MLSValsSet",
            vec![
                NanonisValue::I32(num_segments),
                NanonisValue::ArrayF32(bias_start),
                NanonisValue::ArrayF32(bias_end),
                NanonisValue::ArrayF32(initial_settling),
                NanonisValue::ArrayF32(settling),
                NanonisValue::ArrayF32(integration),
                NanonisValue::ArrayF32(slew_rate),
                NanonisValue::ArrayI32(steps),
            ],
            vec!["i", "*f", "*f", "*f", "*f", "*f", "*f", "*i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the MLS segment values.
    ///
    /// # Returns
    /// A vector of [`MLSSegment`] configurations.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let segments = client.bias_spectr_mls_vals_get()?;
    /// for (i, seg) in segments.iter().enumerate() {
    ///     println!("Segment {}: {:.2}V to {:.2}V, {} steps",
    ///              i, seg.bias_start, seg.bias_end, seg.steps);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bias_spectr_mls_vals_get(&mut self) -> Result<Vec<MLSSegment>, NanonisError> {
        let result = self.quick_send(
            "BiasSpectr.MLSValsGet",
            vec![],
            vec![],
            vec!["i", "*f", "*f", "*f", "*f", "*f", "*f", "*i"],
        )?;

        if result.len() >= 8 {
            let num_segments = result[0].as_i32()? as usize;
            let bias_start = result[1].as_f32_array()?;
            let bias_end = result[2].as_f32_array()?;
            let initial_settling = result[3].as_f32_array()?;
            let settling = result[4].as_f32_array()?;
            let integration = result[5].as_f32_array()?;
            let slew_rate = result[6].as_f32_array()?;
            let steps = result[7].as_i32_array()?;

            let mut segments = Vec::with_capacity(num_segments);
            for i in 0..num_segments {
                segments.push(MLSSegment {
                    bias_start: *bias_start.get(i).unwrap_or(&0.0),
                    bias_end: *bias_end.get(i).unwrap_or(&0.0),
                    initial_settling_time: Duration::from_secs_f32(
                        *initial_settling.get(i).unwrap_or(&0.0),
                    ),
                    settling_time: Duration::from_secs_f32(*settling.get(i).unwrap_or(&0.0)),
                    integration_time: Duration::from_secs_f32(*integration.get(i).unwrap_or(&0.0)),
                    max_slew_rate: *slew_rate.get(i).unwrap_or(&1.0),
                    steps: *steps.get(i).unwrap_or(&100),
                });
            }

            Ok(segments)
        } else {
            Err(NanonisError::Protocol(
                "Invalid MLS vals response".to_string(),
            ))
        }
    }
}
