mod types;
pub use types::*;

use super::NanonisClient;
use crate::error::NanonisError;

impl NanonisClient {
    /// Set the mode (User Output, Monitor, Calculated signal) of the selected user output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to be used (1 to number of available outputs)
    /// * `output_mode` - Output mode to set
    ///
    /// # Errors
    /// Returns `NanonisError` if:
    /// - Invalid output index is provided
    /// - Communication timeout or protocol error
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::{NanonisClient, user_out::OutputMode};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set output 1 to Monitor mode
    /// client.user_out_mode_set(1, OutputMode::Monitor)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_mode_set(
        &mut self,
        output_index: i32,
        output_mode: OutputMode,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "UserOut.ModeSet",
            vec![output_index.into(), output_mode.into()],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the mode (User Output, Monitor, Calculated signal) of the selected user output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to query (1 to number of available outputs)
    ///
    /// # Returns
    /// The current output mode (UserOutput, Monitor, CalcSignal, or Override)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid response received.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let mode = client.user_out_mode_get(1)?;
    /// println!("Output 1 mode: {:?}", mode);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_mode_get(&mut self, output_index: i32) -> Result<OutputMode, NanonisError> {
        let result = self.quick_send(
            "UserOut.ModeGet",
            vec![output_index.into()],
            vec!["i"],
            vec!["H"],
        )?;
        match result.first() {
            Some(value) => {
                let mode_val = value.as_u16()?;
                OutputMode::try_from(mode_val)
            }
            None => Err(NanonisError::Protocol(
                "No output mode value returned".to_string(),
            )),
        }
    }

    /// Set the monitor channel of the selected output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to configure (1 to number of available outputs)
    /// * `monitor_channel_index` - Index of the channel to monitor (0-127)
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid indices provided or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set output 1 to monitor channel 5
    /// client.user_out_monitor_ch_set(1, 5)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_monitor_ch_set(
        &mut self,
        output_index: i32,
        monitor_channel_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "UserOut.MonitorChSet",
            vec![output_index.into(), monitor_channel_index.into()],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the monitor channel of the selected output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to query (1 to number of available outputs)
    ///
    /// # Returns
    /// The monitor channel index (0-127)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid response received.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let channel = client.user_out_monitor_ch_get(1)?;
    /// println!("Output 1 is monitoring channel {}", channel);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_monitor_ch_get(&mut self, output_index: i32) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "UserOut.MonitorChGet",
            vec![output_index.into()],
            vec!["i"],
            vec!["i"],
        )?;
        match result.first() {
            Some(value) => Ok(value.as_i32()?),
            None => Err(NanonisError::Protocol(
                "No monitor channel value returned".to_string(),
            )),
        }
    }

    /// Set the value of the selected user output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to set (1 to number of available outputs)
    /// * `output_value` - Value to apply in physical units
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid output index or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set output 1 to 2.5V (or appropriate physical unit)
    /// client.user_out_val_set(1, 2.5)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_val_set(
        &mut self,
        output_index: i32,
        output_value: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "UserOut.ValSet",
            vec![output_index.into(), output_value.into()],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Set the calibration of the selected user output or monitor channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to configure (1 to number of available outputs)
    /// * `calibration_per_volt` - Calibration factor per volt
    /// * `offset_in_physical_units` - Offset in physical units
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid parameters or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set calibration for output 1: 10 units/V with 0.5 unit offset
    /// client.user_out_calibr_set(1, 10.0, 0.5)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_calibr_set(
        &mut self,
        output_index: i32,
        calibration_per_volt: f32,
        offset_in_physical_units: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "UserOut.CalibrSet",
            vec![
                output_index.into(),
                calibration_per_volt.into(),
                offset_in_physical_units.into(),
            ],
            vec!["i", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Set the Calculated Signal name of the selected output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to configure (1 to number of available outputs)
    /// * `calculated_signal_name` - Name of the calculated signal
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid output index or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// client.user_out_calc_signal_name_set(1, "MyCalcSignal".to_string())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_calc_signal_name_set(
        &mut self,
        output_index: i32,
        calculated_signal_name: String,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "UserOut.CalcSignalNameSet",
            vec![output_index.into(), calculated_signal_name.into()],
            vec!["i", "+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Calculated Signal name of the selected output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to query (1 to number of available outputs)
    ///
    /// # Returns
    /// The calculated signal name
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid response received.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let name = client.user_out_calc_signal_name_get(1)?;
    /// println!("Calculated signal name: {}", name);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_calc_signal_name_get(
        &mut self,
        output_index: i32,
    ) -> Result<String, NanonisError> {
        let result = self.quick_send(
            "UserOut.CalcSignalNameGet",
            vec![output_index.into()],
            vec!["i"],
            vec!["i", "*-c"],
        )?;
        if result.len() >= 2 {
            Ok(result[1].as_string()?.to_string())
        } else {
            Err(NanonisError::Protocol(
                "Invalid calc signal name response".to_string(),
            ))
        }
    }

    /// Set the configuration of the Calculated Signal for the selected output channel.
    ///
    /// The configuration is a math operation between 2 signals, or the logarithmic value of one signal.
    ///
    /// # Arguments
    /// * `output_index` - Output to configure (1 to number of available outputs)
    /// * `config` - Calculated signal configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid parameters or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::{NanonisClient, user_out::{CalcSignalConfig, CalcOperation}};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Configure output 1 to add signals 5 and 10
    /// let config = CalcSignalConfig::new(5, CalcOperation::Add, 10);
    /// client.user_out_calc_signal_config_set(1, config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_calc_signal_config_set(
        &mut self,
        output_index: i32,
        config: CalcSignalConfig,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "UserOut.CalcSignalConfigSet",
            vec![
                output_index.into(),
                config.signal_1.into(),
                config.operation.into(),
                config.signal_2.into(),
            ],
            vec!["i", "H", "H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the configuration of the Calculated Signal for the selected output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to query (1 to number of available outputs)
    ///
    /// # Returns
    /// The calculated signal configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid response received.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let config = client.user_out_calc_signal_config_get(1)?;
    /// println!("Signal 1: {}, Operation: {:?}, Signal 2: {}",
    ///          config.signal_1, config.operation, config.signal_2);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_calc_signal_config_get(
        &mut self,
        output_index: i32,
    ) -> Result<CalcSignalConfig, NanonisError> {
        let result = self.quick_send(
            "UserOut.CalcSignalConfigGet",
            vec![output_index.into()],
            vec!["i"],
            vec!["H", "H", "H"],
        )?;
        if result.len() >= 3 {
            let signal_1 = result[0].as_u16()?;
            let operation = CalcOperation::try_from(result[1].as_u16()?)?;
            let signal_2 = result[2].as_u16()?;
            Ok(CalcSignalConfig::new(signal_1, operation, signal_2))
        } else {
            Err(NanonisError::Protocol(
                "Invalid calc signal config response".to_string(),
            ))
        }
    }

    /// Set the physical limits (in calibrated units) of the selected output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to configure (1 to number of available outputs)
    /// * `upper_limit` - Upper physical limit of the user output
    /// * `lower_limit` - Lower physical limit of the user output
    /// * `raw_limits` - Whether to set physical limits (false) or raw limits (true)
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid parameters or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set physical limits: -10V to +10V
    /// client.user_out_limits_set(1, 10.0, -10.0, false)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_limits_set(
        &mut self,
        output_index: i32,
        upper_limit: f32,
        lower_limit: f32,
        raw_limits: bool,
    ) -> Result<(), NanonisError> {
        let raw_flag = if raw_limits { 1u32 } else { 0u32 };
        self.quick_send(
            "UserOut.LimitsSet",
            vec![
                output_index.into(),
                upper_limit.into(),
                lower_limit.into(),
                raw_flag.into(),
            ],
            vec!["i", "f", "f", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the physical limits (in calibrated units) of the selected output channel.
    ///
    /// # Arguments
    /// * `output_index` - Output to query (1 to number of available outputs)
    /// * `raw_limits` - Whether to get physical limits (false) or raw limits (true)
    ///
    /// # Returns
    /// A tuple containing (upper_limit, lower_limit)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or invalid response received.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let (upper, lower) = client.user_out_limits_get(1, false)?;
    /// println!("Limits: {} to {}", lower, upper);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn user_out_limits_get(
        &mut self,
        output_index: i32,
        raw_limits: bool,
    ) -> Result<(f32, f32), NanonisError> {
        let raw_flag = if raw_limits { 1u32 } else { 0u32 };
        let result = self.quick_send(
            "UserOut.LimitsGet",
            vec![output_index.into(), raw_flag.into()],
            vec!["i", "I"],
            vec!["f", "f"],
        )?;
        if result.len() >= 2 {
            let upper_limit = result[0].as_f32()?;
            let lower_limit = result[1].as_f32()?;
            Ok((upper_limit, lower_limit))
        } else {
            Err(NanonisError::Protocol(
                "Invalid limits response".to_string(),
            ))
        }
    }
}
