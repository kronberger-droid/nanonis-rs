use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Data Logger acquisition mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DataLogAcqMode {
    /// No change to current mode
    #[default]
    NoChange = 0,
    /// Continuous acquisition until stopped
    Continuous = 1,
    /// Timed acquisition for set duration
    Timed = 2,
}

impl From<DataLogAcqMode> for u16 {
    fn from(mode: DataLogAcqMode) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for DataLogAcqMode {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DataLogAcqMode::Continuous),
            1 => Ok(DataLogAcqMode::Timed),
            _ => Err(NanonisError::Type(format!(
                "Invalid DataLogAcqMode value: {}",
                value
            ))),
        }
    }
}

/// Data Logger status information.
#[derive(Debug, Clone)]
pub struct DataLogStatus {
    /// Timestamp when acquisition started
    pub start_time: String,
    /// Hours elapsed since acquisition started
    pub elapsed_hours: u16,
    /// Minutes displayed
    pub elapsed_minutes: u16,
    /// Seconds displayed
    pub elapsed_seconds: f32,
    /// Timestamp when acquisition stopped
    pub stop_time: String,
    /// Path to last saved file
    pub saved_file_path: String,
    /// Number of points saved
    pub saved_points: i32,
}

/// Data Logger acquisition configuration.
#[derive(Debug, Clone)]
pub struct DataLogProps {
    /// Acquisition mode
    pub mode: DataLogAcqMode,
    /// Acquisition duration hours
    pub duration_hours: i32,
    /// Acquisition duration minutes
    pub duration_minutes: i32,
    /// Acquisition duration seconds
    pub duration_seconds: f32,
    /// Averaging count (samples averaged per data point)
    pub averaging: i32,
    /// Base filename for saved files
    pub basename: String,
    /// Comment saved in file
    pub comment: String,
}

impl Default for DataLogProps {
    fn default() -> Self {
        Self {
            mode: DataLogAcqMode::Continuous,
            duration_hours: 0,
            duration_minutes: 0,
            duration_seconds: 0.0,
            averaging: 1,
            basename: String::new(),
            comment: String::new(),
        }
    }
}

impl NanonisClient {
    /// Open the Data Logger module.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.data_log_open()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn data_log_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("DataLog.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Start the acquisition in the Data Logger module.
    ///
    /// Before using this function, select the channels to record.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn data_log_start(&mut self) -> Result<(), NanonisError> {
        self.quick_send("DataLog.Start", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Stop the acquisition in the Data Logger module.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn data_log_stop(&mut self) -> Result<(), NanonisError> {
        self.quick_send("DataLog.Stop", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get the status of the Data Logger module.
    ///
    /// # Returns
    /// A [`DataLogStatus`] struct with current status information.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn data_log_status_get(&mut self) -> Result<DataLogStatus, NanonisError> {
        let result = self.quick_send(
            "DataLog.StatusGet",
            vec![],
            vec![],
            vec!["i", "*-c", "H", "H", "f", "i", "*-c", "i", "*-c", "i"],
        )?;

        if result.len() >= 10 {
            Ok(DataLogStatus {
                start_time: result[1].as_string()?.to_string(),
                elapsed_hours: result[2].as_u16()?,
                elapsed_minutes: result[3].as_u16()?,
                elapsed_seconds: result[4].as_f32()?,
                stop_time: result[6].as_string()?.to_string(),
                saved_file_path: result[8].as_string()?.to_string(),
                saved_points: result[9].as_i32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the list of recorded channels in the Data Logger.
    ///
    /// # Arguments
    /// * `channel_indexes` - Channel indexes (0-23 for signals in the Signals Manager)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn data_log_chs_set(&mut self, channel_indexes: &[i32]) -> Result<(), NanonisError> {
        self.quick_send(
            "DataLog.ChsSet",
            vec![NanonisValue::ArrayI32(channel_indexes.to_vec())],
            vec!["+*i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the list of recorded channels in the Data Logger.
    ///
    /// # Returns
    /// A vector of channel indexes.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn data_log_chs_get(&mut self) -> Result<Vec<i32>, NanonisError> {
        let result = self.quick_send("DataLog.ChsGet", vec![], vec![], vec!["i", "*i"])?;

        if result.len() >= 2 {
            Ok(result[1].as_i32_array()?.to_vec())
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the acquisition configuration for the Data Logger.
    ///
    /// # Arguments
    /// * `props` - A [`DataLogProps`] struct with configuration
    /// * `modules` - List of module names whose parameters will be saved in file header
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::data_log::{DataLogProps, DataLogAcqMode};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let props = DataLogProps {
    ///     mode: DataLogAcqMode::Timed,
    ///     duration_hours: 0,
    ///     duration_minutes: 10,
    ///     duration_seconds: 0.0,
    ///     averaging: 10,
    ///     basename: "measurement".to_string(),
    ///     comment: "Test measurement".to_string(),
    /// };
    /// client.data_log_props_set(&props, &["Z-Controller".to_string()])?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn data_log_props_set(
        &mut self,
        props: &DataLogProps,
        modules: &[String],
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "DataLog.PropsSet",
            vec![
                NanonisValue::U16(props.mode.into()),
                NanonisValue::I32(props.duration_hours),
                NanonisValue::I32(props.duration_minutes),
                NanonisValue::F32(props.duration_seconds),
                NanonisValue::I32(props.averaging),
                NanonisValue::String(props.basename.clone()),
                NanonisValue::String(props.comment.clone()),
                NanonisValue::ArrayString(modules.to_vec()),
            ],
            vec!["H", "i", "i", "f", "i", "+*c", "+*c", "+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the acquisition configuration for the Data Logger.
    ///
    /// # Returns
    /// A [`DataLogProps`] struct with current configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn data_log_props_get(&mut self) -> Result<DataLogProps, NanonisError> {
        let result = self.quick_send(
            "DataLog.PropsGet",
            vec![],
            vec![],
            vec!["H", "i", "i", "f", "i", "i", "*-c", "i", "*-c"],
        )?;

        if result.len() >= 9 {
            Ok(DataLogProps {
                mode: result[0].as_u16()?.try_into()?,
                duration_hours: result[1].as_i32()?,
                duration_minutes: result[2].as_i32()?,
                duration_seconds: result[3].as_f32()?,
                averaging: result[4].as_i32()?,
                basename: result[6].as_string()?.to_string(),
                comment: result[8].as_string()?.to_string(),
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
