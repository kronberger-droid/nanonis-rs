use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Version information returned by the Nanonis software.
///
/// Contains detailed version and release information about both the
/// host application and RT Engine components of the Nanonis system.
#[derive(Debug, Clone, PartialEq)]
pub struct VersionInfo {
    /// Product line name (e.g., "Nanonis SPM Control Software" or "Nanonis Tramea Software")
    pub product_line: String,
    /// Software version string (e.g., "Generic 5")
    pub version: String,
    /// Host application release number
    pub host_app_release: u32,
    /// RT Engine application release number
    pub rt_engine_release: u32,
}

impl NanonisClient {
    /// Get the session path from the Nanonis software.
    ///
    /// Returns the current session path where Nanonis stores configuration
    /// and data files for the active session.
    ///
    /// # Returns
    /// The session path as a String.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or protocol error occurs.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let session_path = client.util_session_path_get()?;
    /// println!("Session path: {}", session_path);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_session_path_get(&mut self) -> Result<String, NanonisError> {
        let result = self.quick_send("Util.SessionPathGet", vec![], vec![], vec!["i", "*-c"])?;

        match result.get(1) {
            Some(value) => Ok(value.as_string()?.to_string()),
            None => Err(NanonisError::Protocol(
                "No session path returned".to_string(),
            )),
        }
    }

    /// Load settings from a specified .ini file.
    ///
    /// Loads the Nanonis configuration settings from the specified file path.
    /// Can also automatically load settings from the session file.
    ///
    /// # Arguments
    /// * `settings_file_path` - Path to the settings file to load
    /// * `load_session_settings` - If true, automatically loads from session file (bypasses path argument)
    ///
    /// # Errors
    /// Returns `NanonisError` if the file doesn't exist, can't be read, or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Load from specific file
    /// client.util_settings_load("/path/to/settings.ini", false)?;
    ///
    /// // Or load from session file
    /// client.util_settings_load("", true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_settings_load(
        &mut self,
        settings_file_path: &str,
        load_session_settings: bool,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Util.SettingsLoad",
            vec![
                NanonisValue::String(settings_file_path.to_string()),
                NanonisValue::U32(if load_session_settings { 1 } else { 0 }),
            ],
            vec!["+*c", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Save current settings to a specified .ini file.
    ///
    /// Saves the current Nanonis configuration settings to the specified file path.
    /// Can also automatically save to the session file.
    ///
    /// # Arguments
    /// * `settings_file_path` - Path where the settings file will be saved
    /// * `save_session_settings` - If true, automatically saves to session file (bypasses path argument)
    ///
    /// # Errors
    /// Returns `NanonisError` if the file can't be written or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Save to specific file
    /// client.util_settings_save("/path/to/settings.ini", false)?;
    ///
    /// // Or save to session file
    /// client.util_settings_save("", true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_settings_save(
        &mut self,
        settings_file_path: &str,
        save_session_settings: bool,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Util.SettingsSave",
            vec![
                NanonisValue::String(settings_file_path.to_string()),
                NanonisValue::U32(if save_session_settings { 1 } else { 0 }),
            ],
            vec!["+*c", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Load a layout from a specified .ini file.
    ///
    /// Loads the Nanonis UI layout configuration from the specified file path.
    /// Can also automatically load the layout from the session file.
    ///
    /// # Arguments
    /// * `layout_file_path` - Path to the layout file to load
    /// * `load_session_layout` - If true, automatically loads from session file (bypasses path argument)
    ///
    /// # Errors
    /// Returns `NanonisError` if the file doesn't exist, can't be read, or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Load from specific file
    /// client.util_layout_load("/path/to/layout.ini", false)?;
    ///
    /// // Or load from session file
    /// client.util_layout_load("", true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_layout_load(
        &mut self,
        layout_file_path: &str,
        load_session_layout: bool,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Util.LayoutLoad",
            vec![
                NanonisValue::String(layout_file_path.to_string()),
                NanonisValue::U32(if load_session_layout { 1 } else { 0 }),
            ],
            vec!["+*c", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Save current layout to a specified .ini file.
    ///
    /// Saves the current Nanonis UI layout configuration to the specified file path.
    /// Can also automatically save to the session file.
    ///
    /// # Arguments
    /// * `layout_file_path` - Path where the layout file will be saved
    /// * `save_session_layout` - If true, automatically saves to session file (bypasses path argument)
    ///
    /// # Errors
    /// Returns `NanonisError` if the file can't be written or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Save to specific file
    /// client.util_layout_save("/path/to/layout.ini", false)?;
    ///
    /// // Or save to session file
    /// client.util_layout_save("", true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_layout_save(
        &mut self,
        layout_file_path: &str,
        save_session_layout: bool,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Util.LayoutSave",
            vec![
                NanonisValue::String(layout_file_path.to_string()),
                NanonisValue::U32(if save_session_layout { 1 } else { 0 }),
            ],
            vec!["+*c", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Lock the Nanonis software interface.
    ///
    /// Launches a lock modal window that prevents user interaction with the
    /// Nanonis software until it is unlocked manually or through `util_unlock()`.
    /// This is useful for automated experiments where you want to prevent
    /// accidental user interference.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or protocol error occurs.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Lock the interface during automated experiment
    /// client.util_lock()?;
    ///
    /// // Perform automated operations
    /// thread::sleep(Duration::from_secs(5));
    ///
    /// // Unlock when done
    /// client.util_unlock()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_lock(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Util.Lock", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Unlock the Nanonis software interface.
    ///
    /// Closes the lock modal window that prevents user interaction with the
    /// Nanonis software. Use this after `util_lock()` when automated operations
    /// are complete.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or protocol error occurs.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Unlock the interface
    /// client.util_unlock()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_unlock(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Util.UnLock", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set the Real Time controller frequency.
    ///
    /// Configures the frequency of the Real Time (RT) controller which determines
    /// the speed at which the feedback loop and other real-time operations run.
    ///
    /// # Arguments
    /// * `rt_frequency` - RT frequency in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid frequency provided or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set RT frequency to 20 kHz
    /// client.util_rt_freq_set(20000.0)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_rt_freq_set(&mut self, rt_frequency: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "Util.RTFreqSet",
            vec![NanonisValue::F32(rt_frequency)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Real Time controller frequency.
    ///
    /// Returns the current frequency of the Real Time (RT) controller.
    ///
    /// # Returns
    /// RT frequency in Hz.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or protocol error occurs.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let frequency = client.util_rt_freq_get()?;
    /// println!("RT frequency: {} Hz", frequency);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_rt_freq_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("Util.RTFreqGet", vec![], vec![], vec!["f"])?;

        match result.first() {
            Some(value) => Ok(value.as_f32()?),
            None => Err(NanonisError::Protocol(
                "No RT frequency returned".to_string(),
            )),
        }
    }

    /// Set the Acquisition Period in the TCP Receiver.
    ///
    /// Configures the period at which data is acquired and transmitted
    /// via the TCP interface.
    ///
    /// # Arguments
    /// * `acquisition_period` - Acquisition period in seconds
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid period provided or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set acquisition period to 100 ms
    /// client.util_acq_period_set(0.1)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_acq_period_set(&mut self, acquisition_period: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "Util.AcqPeriodSet",
            vec![NanonisValue::F32(acquisition_period)],
            vec!["f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Acquisition Period from the TCP Receiver.
    ///
    /// Returns the current acquisition period configured in the TCP Receiver.
    ///
    /// # Returns
    /// Acquisition period in seconds.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or protocol error occurs.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let period = client.util_acq_period_get()?;
    /// println!("Acquisition period: {} s ({} ms)", period, period * 1000.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_acq_period_get(&mut self) -> Result<f32, NanonisError> {
        let result = self.quick_send("Util.AcqPeriodGet", vec![], vec![], vec!["f"])?;

        match result.first() {
            Some(value) => Ok(value.as_f32()?),
            None => Err(NanonisError::Protocol(
                "No acquisition period returned".to_string(),
            )),
        }
    }

    /// Set the Real-time oversampling in the TCP Receiver.
    ///
    /// Configures the oversampling factor for the 24 signals on the RT engine
    /// before they are sent to the host. The oversampling affects the maximum
    /// Spectrum Analyzer frequency and other displays.
    ///
    /// # Arguments
    /// * `rt_oversampling` - RT oversampling factor
    ///
    /// # Errors
    /// Returns `NanonisError` if invalid oversampling value or communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Set oversampling to 10x
    /// client.util_rt_oversampl_set(10)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_rt_oversampl_set(&mut self, rt_oversampling: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "Util.RTOversamplSet",
            vec![NanonisValue::I32(rt_oversampling)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Real-time oversampling from the TCP Receiver.
    ///
    /// Returns the current oversampling factor configured for RT engine signals.
    ///
    /// # Returns
    /// RT oversampling factor.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or protocol error occurs.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let oversampling = client.util_rt_oversampl_get()?;
    /// println!("RT oversampling: {}x", oversampling);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_rt_oversampl_get(&mut self) -> Result<i32, NanonisError> {
        let result = self.quick_send("Util.RTOversamplGet", vec![], vec![], vec!["i"])?;

        match result.first() {
            Some(value) => Ok(value.as_i32()?),
            None => Err(NanonisError::Protocol(
                "No RT oversampling returned".to_string(),
            )),
        }
    }

    /// Quit the Nanonis software with options to save settings, layout, and signals.
    ///
    /// Provides the same functionality as the dialog that appears when quitting
    /// through the File menu. Can save settings, layout, and signal configurations
    /// before exiting.
    ///
    /// **Warning**: This will close the Nanonis software and terminate the TCP connection.
    ///
    /// # Arguments
    /// * `use_stored_values` - If true, uses stored quit preferences (ignores other arguments)
    /// * `settings_name` - Name of settings file to save (empty string = don't save)
    /// * `layout_name` - Name of layout file to save (empty string = don't save)
    /// * `save_signals` - If true, saves signal configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails before quit completes.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// // Quit without saving anything
    /// client.util_quit(false, "", "", false)?;
    ///
    /// // Quit and save everything with stored preferences
    /// // client.util_quit(true, "", "", false)?;
    ///
    /// // Quit and save specific settings and layout
    /// // client.util_quit(false, "my_settings", "my_layout", true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_quit(
        &mut self,
        use_stored_values: bool,
        settings_name: &str,
        layout_name: &str,
        save_signals: bool,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Util.Quit",
            vec![
                NanonisValue::U32(if use_stored_values { 1 } else { 0 }),
                NanonisValue::String(settings_name.to_string()),
                NanonisValue::String(layout_name.to_string()),
                NanonisValue::U32(if save_signals { 1 } else { 0 }),
            ],
            vec!["I", "+*c", "+*c", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get version information from the Nanonis software.
    ///
    /// Returns detailed version information about the Nanonis system including
    /// product line, version string, and release numbers for both host application
    /// and RT Engine.
    ///
    /// # Returns
    /// A `VersionInfo` struct containing all version details.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or protocol error occurs.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    ///
    /// let version = client.util_version_get()?;
    /// println!("Product: {}", version.product_line);
    /// println!("Version: {}", version.version);
    /// println!("Host release: {}", version.host_app_release);
    /// println!("RT Engine release: {}", version.rt_engine_release);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn util_version_get(&mut self) -> Result<VersionInfo, NanonisError> {
        let result = self.quick_send(
            "Util.VersionGet",
            vec![],
            vec![],
            vec!["+*c", "+*c", "I", "I"],
        )?;

        if result.len() >= 4 {
            Ok(VersionInfo {
                product_line: result[0].as_string()?.to_string(),
                version: result[1].as_string()?.to_string(),
                host_app_release: result[2].as_u32()?,
                rt_engine_release: result[3].as_u32()?,
            })
        } else {
            Err(NanonisError::Protocol(
                "Invalid version info response".to_string(),
            ))
        }
    }
}
