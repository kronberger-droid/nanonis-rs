use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Acquire buffer selection for Script module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcquireBuffer {
    /// Both buffers for autosave
    Both = 0,
    #[default]
    /// Acquire Buffer 1
    Buffer1 = 1,
    /// Acquire Buffer 2
    Buffer2 = 2,
}

impl From<AcquireBuffer> for u16 {
    fn from(buf: AcquireBuffer) -> Self {
        buf as u16
    }
}

/// Script data returned from a sweep.
#[derive(Debug, Clone, Default)]
pub struct ScriptData {
    /// Data rows (one per channel)
    pub data: Vec<Vec<f32>>,
}

impl NanonisClient {
    // ==================== Script Module ====================

    /// Open the Script module.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Script.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Load a script in the Script module.
    ///
    /// # Arguments
    /// * `script_index` - Script slot (1 to total scripts, -1 for current)
    /// * `file_path` - Path to the script file
    /// * `load_session` - If true, loads from session file instead of file_path
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_load(
        &mut self,
        script_index: i32,
        file_path: &str,
        load_session: bool,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.Load",
            vec![
                NanonisValue::I32(script_index),
                NanonisValue::String(file_path.to_string()),
                NanonisValue::U32(if load_session { 1 } else { 0 }),
            ],
            vec!["i", "+*c", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Save the current script to a file.
    ///
    /// # Arguments
    /// * `script_index` - Script slot (1 to total scripts, -1 for current)
    /// * `file_path` - Path to save the script file
    /// * `save_session` - If true, saves to session file instead
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_save(
        &mut self,
        script_index: i32,
        file_path: &str,
        save_session: bool,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.Save",
            vec![
                NanonisValue::I32(script_index),
                NanonisValue::String(file_path.to_string()),
                NanonisValue::U32(if save_session { 1 } else { 0 }),
            ],
            vec!["i", "+*c", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Deploy a script in the Script module.
    ///
    /// # Arguments
    /// * `script_index` - Script slot (1 to total scripts, -1 for current)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_deploy(&mut self, script_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.Deploy",
            vec![NanonisValue::I32(script_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Undeploy a script in the Script module.
    ///
    /// # Arguments
    /// * `script_index` - Script slot (1 to total scripts, -1 for current)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_undeploy(&mut self, script_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.Undeploy",
            vec![NanonisValue::I32(script_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Run a script in the Script module.
    ///
    /// # Arguments
    /// * `script_index` - Script slot (1 to total scripts, -1 for current)
    /// * `wait` - If true, waits until script finishes
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_run(&mut self, script_index: i32, wait: bool) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.Run",
            vec![
                NanonisValue::I32(script_index),
                NanonisValue::U32(if wait { 1 } else { 0 }),
            ],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Stop the running script.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_stop(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Script.Stop", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Get the list of acquired channels in the Script module.
    ///
    /// # Arguments
    /// * `buffer` - Acquire buffer to read from
    ///
    /// # Returns
    /// Vector of channel indexes (0-23).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_chs_get(&mut self, buffer: AcquireBuffer) -> Result<Vec<i32>, NanonisError> {
        let result = self.quick_send(
            "Script.ChsGet",
            vec![NanonisValue::U16(buffer.into())],
            vec!["H"],
            vec!["i", "*i"],
        )?;

        result[1].as_i32_array().map(|a| a.to_vec())
    }

    /// Set the list of acquired channels in the Script module.
    ///
    /// # Arguments
    /// * `buffer` - Acquire buffer to configure
    /// * `channel_indexes` - Channel indexes (0-23)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_chs_set(
        &mut self,
        buffer: AcquireBuffer,
        channel_indexes: &[i32],
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.ChsSet",
            vec![
                NanonisValue::U16(buffer.into()),
                NanonisValue::ArrayI32(channel_indexes.to_vec()),
            ],
            vec!["H", "+*i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the data acquired in the Script module.
    ///
    /// # Arguments
    /// * `buffer` - Acquire buffer to read from
    /// * `sweep_number` - Sweep number (starts at 0)
    ///
    /// # Returns
    /// 2D array of script data.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_data_get(
        &mut self,
        buffer: AcquireBuffer,
        sweep_number: i32,
    ) -> Result<ScriptData, NanonisError> {
        let result = self.quick_send(
            "Script.DataGet",
            vec![
                NanonisValue::U16(buffer.into()),
                NanonisValue::I32(sweep_number),
            ],
            vec!["H", "i"],
            vec!["i", "i", "2f"],
        )?;

        Ok(ScriptData {
            data: result[2].as_f32_2d_array()?.to_vec(),
        })
    }

    /// Autosave script data to file.
    ///
    /// # Arguments
    /// * `buffer` - Acquire buffer(s) to save
    /// * `sweep_number` - Sweep number (-1 for all sweeps)
    /// * `all_sweeps_to_same_file` - If true, saves all sweeps to one file
    /// * `folder_path` - Folder path (empty for last used)
    /// * `basename` - File basename (empty for last used)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_autosave(
        &mut self,
        buffer: AcquireBuffer,
        sweep_number: i32,
        all_sweeps_to_same_file: bool,
        folder_path: &str,
        basename: &str,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.Autosave",
            vec![
                NanonisValue::U16(buffer.into()),
                NanonisValue::I32(sweep_number),
                NanonisValue::U32(if all_sweeps_to_same_file { 1 } else { 0 }),
                NanonisValue::String(folder_path.to_string()),
                NanonisValue::String(basename.to_string()),
            ],
            vec!["H", "i", "I", "+*c", "+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Open the LUT (Look Up Table) Editor from the Script module.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_lut_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("Script.LUTOpen", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Load a LUT from file or from values.
    ///
    /// # Arguments
    /// * `lut_index` - LUT index (1 to total LUTs)
    /// * `file_path` - Path to .luts file (empty to use values)
    /// * `values` - LUT values (used if file_path is empty)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_lut_load(
        &mut self,
        lut_index: i32,
        file_path: &str,
        values: &[f32],
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.LUTLoad",
            vec![
                NanonisValue::I32(lut_index),
                NanonisValue::String(file_path.to_string()),
                NanonisValue::ArrayF32(values.to_vec()),
            ],
            vec!["i", "+*c", "+*f"],
            vec![],
        )?;
        Ok(())
    }

    /// Save a LUT to file.
    ///
    /// # Arguments
    /// * `lut_index` - LUT index (1 to total LUTs)
    /// * `file_path` - Path to save .luts file
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_lut_save(&mut self, lut_index: i32, file_path: &str) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.LUTSave",
            vec![
                NanonisValue::I32(lut_index),
                NanonisValue::String(file_path.to_string()),
            ],
            vec!["i", "+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Deploy a LUT from the LUT Editor.
    ///
    /// # Arguments
    /// * `lut_index` - LUT index (1 to total LUTs)
    /// * `wait` - If true, waits until deployment finishes
    /// * `timeout_ms` - Timeout in milliseconds (-1 for forever)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn script_lut_deploy(
        &mut self,
        lut_index: i32,
        wait: bool,
        timeout_ms: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Script.LUTDeploy",
            vec![
                NanonisValue::I32(lut_index),
                NanonisValue::U32(if wait { 1 } else { 0 }),
                NanonisValue::I32(timeout_ms),
            ],
            vec!["i", "I", "i"],
            vec![],
        )?;
        Ok(())
    }
}
