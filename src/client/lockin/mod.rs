mod types;
pub use types::*;

use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

impl NanonisClient {
    // ==================== Modulator Methods ====================

    /// Turn the specified Lock-In modulator on or off.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    /// * `on` - `true` to turn on, `false` to turn off
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.lockin_mod_on_off_set(1, true)?; // Turn on modulator 1
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lockin_mod_on_off_set(
        &mut self,
        modulator_num: i32,
        on: bool,
    ) -> Result<(), NanonisError> {
        let on_flag = if on { 1u32 } else { 0u32 };
        self.quick_send(
            "LockIn.ModOnOffSet",
            vec![NanonisValue::I32(modulator_num), NanonisValue::U32(on_flag)],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the on/off status of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    ///
    /// # Returns
    /// `true` if modulator is on, `false` if off.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let is_on = client.lockin_mod_on_off_get(1)?;
    /// println!("Modulator 1 is {}", if is_on { "on" } else { "off" });
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lockin_mod_on_off_get(&mut self, modulator_num: i32) -> Result<bool, NanonisError> {
        let result = self.quick_send(
            "LockIn.ModOnOffGet",
            vec![NanonisValue::I32(modulator_num)],
            vec!["i"],
            vec!["I"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the modulated signal of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    /// * `signal_index` - Signal index (0-127)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.lockin_mod_signal_set(1, 14)?; // Set modulator 1 to signal 14
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lockin_mod_signal_set(
        &mut self,
        modulator_num: i32,
        signal_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.ModSignalSet",
            vec![
                NanonisValue::I32(modulator_num),
                NanonisValue::I32(signal_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the modulated signal index of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    ///
    /// # Returns
    /// Signal index (0-127).
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_mod_signal_get(&mut self, modulator_num: i32) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "LockIn.ModSignalGet",
            vec![NanonisValue::I32(modulator_num)],
            vec!["i"],
            vec!["i"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the phase register index of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    /// * `phase_register_index` - Phase register index (1-8)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.lockin_mod_phas_reg_set(1, 1)?; // Assign modulator 1 to phase register 1
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lockin_mod_phas_reg_set(
        &mut self,
        modulator_num: i32,
        phase_register_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.ModPhasRegSet",
            vec![
                NanonisValue::I32(modulator_num),
                NanonisValue::I32(phase_register_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the phase register index of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    ///
    /// # Returns
    /// Phase register index (1-8).
    pub fn lockin_mod_phas_reg_get(&mut self, modulator_num: i32) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "LockIn.ModPhasRegGet",
            vec![NanonisValue::I32(modulator_num)],
            vec!["i"],
            vec!["i"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the harmonic of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    /// * `harmonic` - Harmonic number (1 = base frequency)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_mod_harmonic_set(
        &mut self,
        modulator_num: i32,
        harmonic: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.ModHarmonicSet",
            vec![
                NanonisValue::I32(modulator_num),
                NanonisValue::I32(harmonic),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the harmonic of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    ///
    /// # Returns
    /// Harmonic number (1 = base frequency).
    pub fn lockin_mod_harmonic_get(&mut self, modulator_num: i32) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "LockIn.ModHarmonicGet",
            vec![NanonisValue::I32(modulator_num)],
            vec!["i"],
            vec!["i"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the modulation phase offset of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    /// * `phase_deg` - Phase offset in degrees
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_mod_phas_set(
        &mut self,
        modulator_num: i32,
        phase_deg: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.ModPhasSet",
            vec![
                NanonisValue::I32(modulator_num),
                NanonisValue::F32(phase_deg),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the modulation phase offset of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    ///
    /// # Returns
    /// Phase offset in degrees.
    pub fn lockin_mod_phas_get(&mut self, modulator_num: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "LockIn.ModPhasGet",
            vec![NanonisValue::I32(modulator_num)],
            vec!["i"],
            vec!["f"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the modulation amplitude of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    /// * `amplitude` - Modulation amplitude
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_mod_amp_set(
        &mut self,
        modulator_num: i32,
        amplitude: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.ModAmpSet",
            vec![
                NanonisValue::I32(modulator_num),
                NanonisValue::F32(amplitude),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the modulation amplitude of the specified Lock-In modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Modulator number (1-8)
    ///
    /// # Returns
    /// Modulation amplitude.
    pub fn lockin_mod_amp_get(&mut self, modulator_num: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "LockIn.ModAmpGet",
            vec![NanonisValue::I32(modulator_num)],
            vec!["i"],
            vec!["f"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the frequency of the specified Lock-In phase register/modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Phase register/modulator number (1-8)
    /// * `frequency_hz` - Frequency in Hz
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.lockin_mod_phas_freq_set(1, 1000.0)?; // Set 1kHz
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lockin_mod_phas_freq_set(
        &mut self,
        modulator_num: i32,
        frequency_hz: f64,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.ModPhasFreqSet",
            vec![
                NanonisValue::I32(modulator_num),
                NanonisValue::F64(frequency_hz),
            ],
            vec!["i", "d"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the frequency of the specified Lock-In phase register/modulator.
    ///
    /// # Arguments
    /// * `modulator_num` - Phase register/modulator number (1-8)
    ///
    /// # Returns
    /// Frequency in Hz.
    pub fn lockin_mod_phas_freq_get(&mut self, modulator_num: i32) -> Result<f64, NanonisError> {
        let result = self.quick_send(
            "LockIn.ModPhasFreqGet",
            vec![NanonisValue::I32(modulator_num)],
            vec!["i"],
            vec!["d"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_f64()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    // ==================== Demodulator Methods ====================

    /// Set the demodulated signal of the specified Lock-In demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    /// * `signal_index` - Signal index (0-127)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn lockin_demod_signal_set(
        &mut self,
        demodulator_num: i32,
        signal_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.DemodSignalSet",
            vec![
                NanonisValue::I32(demodulator_num),
                NanonisValue::I32(signal_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the demodulated signal index of the specified Lock-In demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    ///
    /// # Returns
    /// Signal index (0-127).
    pub fn lockin_demod_signal_get(&mut self, demodulator_num: i32) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "LockIn.DemodSignalGet",
            vec![NanonisValue::I32(demodulator_num)],
            vec!["i"],
            vec!["i"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the harmonic of the specified Lock-In demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    /// * `harmonic` - Harmonic number (1 = base frequency)
    pub fn lockin_demod_harmonic_set(
        &mut self,
        demodulator_num: i32,
        harmonic: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.DemodHarmonicSet",
            vec![
                NanonisValue::I32(demodulator_num),
                NanonisValue::I32(harmonic),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the harmonic of the specified Lock-In demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    ///
    /// # Returns
    /// Harmonic number (1 = base frequency).
    pub fn lockin_demod_harmonic_get(&mut self, demodulator_num: i32) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "LockIn.DemodHarmonicGet",
            vec![NanonisValue::I32(demodulator_num)],
            vec!["i"],
            vec!["i"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the high-pass filter properties for the specified demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    /// * `filter_order` - Filter order (-1=no change, 0=off, 1-8=active)
    /// * `cutoff_hz` - Cutoff frequency in Hz (0=no change)
    pub fn lockin_demod_hp_filter_set(
        &mut self,
        demodulator_num: i32,
        filter_order: i32,
        cutoff_hz: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.DemodHPFilterSet",
            vec![
                NanonisValue::I32(demodulator_num),
                NanonisValue::I32(filter_order),
                NanonisValue::F32(cutoff_hz),
            ],
            vec!["i", "i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the high-pass filter properties for the specified demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    ///
    /// # Returns
    /// A [`FilterConfig`] with order and cutoff frequency.
    pub fn lockin_demod_hp_filter_get(
        &mut self,
        demodulator_num: i32,
    ) -> Result<FilterConfig, NanonisError> {
        let result = self.quick_send(
            "LockIn.DemodHPFilterGet",
            vec![NanonisValue::I32(demodulator_num)],
            vec!["i"],
            vec!["i", "f"],
        )?;
        if result.len() >= 2 {
            Ok(FilterConfig {
                order: result[0].as_i32()?,
                cutoff_hz: result[1].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the low-pass filter properties for the specified demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    /// * `filter_order` - Filter order (-1=no change, 0=off, 1-8=active)
    /// * `cutoff_hz` - Cutoff frequency in Hz (0=no change)
    pub fn lockin_demod_lp_filter_set(
        &mut self,
        demodulator_num: i32,
        filter_order: i32,
        cutoff_hz: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.DemodLPFilterSet",
            vec![
                NanonisValue::I32(demodulator_num),
                NanonisValue::I32(filter_order),
                NanonisValue::F32(cutoff_hz),
            ],
            vec!["i", "i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the low-pass filter properties for the specified demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    ///
    /// # Returns
    /// A [`FilterConfig`] with order and cutoff frequency.
    pub fn lockin_demod_lp_filter_get(
        &mut self,
        demodulator_num: i32,
    ) -> Result<FilterConfig, NanonisError> {
        let result = self.quick_send(
            "LockIn.DemodLPFilterGet",
            vec![NanonisValue::I32(demodulator_num)],
            vec!["i"],
            vec!["i", "f"],
        )?;
        if result.len() >= 2 {
            Ok(FilterConfig {
                order: result[0].as_i32()?,
                cutoff_hz: result[1].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the phase register index of the specified Lock-In demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    /// * `phase_register_index` - Phase register index (1-8)
    pub fn lockin_demod_phas_reg_set(
        &mut self,
        demodulator_num: i32,
        phase_register_index: i32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.DemodPhasRegSet",
            vec![
                NanonisValue::I32(demodulator_num),
                NanonisValue::I32(phase_register_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the phase register index of the specified Lock-In demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    ///
    /// # Returns
    /// Phase register index (1-8).
    pub fn lockin_demod_phas_reg_get(
        &mut self,
        demodulator_num: i32,
    ) -> Result<i32, NanonisError> {
        let result = self.quick_send(
            "LockIn.DemodPhasRegGet",
            vec![NanonisValue::I32(demodulator_num)],
            vec!["i"],
            vec!["i"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_i32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the reference phase of the specified Lock-In demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    /// * `phase_deg` - Reference phase in degrees
    pub fn lockin_demod_phas_set(
        &mut self,
        demodulator_num: i32,
        phase_deg: f32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.DemodPhasSet",
            vec![
                NanonisValue::I32(demodulator_num),
                NanonisValue::F32(phase_deg),
            ],
            vec!["i", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the reference phase of the specified Lock-In demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    ///
    /// # Returns
    /// Reference phase in degrees.
    pub fn lockin_demod_phas_get(&mut self, demodulator_num: i32) -> Result<f32, NanonisError> {
        let result = self.quick_send(
            "LockIn.DemodPhasGet",
            vec![NanonisValue::I32(demodulator_num)],
            vec!["i"],
            vec!["f"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_f32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the sync filter on/off for the specified demodulator.
    ///
    /// The synchronous filter suppresses harmonic components very well
    /// but only updates after each period of the demodulation frequency.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    /// * `on` - `true` to enable sync filter, `false` to disable
    pub fn lockin_demod_sync_filter_set(
        &mut self,
        demodulator_num: i32,
        on: bool,
    ) -> Result<(), NanonisError> {
        let on_flag = if on { 1u32 } else { 0u32 };
        self.quick_send(
            "LockIn.DemodSyncFilterSet",
            vec![
                NanonisValue::I32(demodulator_num),
                NanonisValue::U32(on_flag),
            ],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the sync filter status for the specified demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    ///
    /// # Returns
    /// `true` if sync filter is on.
    pub fn lockin_demod_sync_filter_get(
        &mut self,
        demodulator_num: i32,
    ) -> Result<bool, NanonisError> {
        let result = self.quick_send(
            "LockIn.DemodSyncFilterGet",
            vec![NanonisValue::I32(demodulator_num)],
            vec!["i"],
            vec!["I"],
        )?;
        if let Some(val) = result.first() {
            Ok(val.as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the RT signals mode for the specified demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    /// * `mode` - [`RTSignalMode::XY`] or [`RTSignalMode::RPhi`]
    pub fn lockin_demod_rt_signals_set(
        &mut self,
        demodulator_num: i32,
        mode: RTSignalMode,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "LockIn.DemodRTSignalsSet",
            vec![
                NanonisValue::I32(demodulator_num),
                NanonisValue::U32(mode.into()),
            ],
            vec!["i", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the RT signals mode for the specified demodulator.
    ///
    /// # Arguments
    /// * `demodulator_num` - Demodulator number (1-8)
    ///
    /// # Returns
    /// The [`RTSignalMode`] (X/Y or R/phi).
    pub fn lockin_demod_rt_signals_get(
        &mut self,
        demodulator_num: i32,
    ) -> Result<RTSignalMode, NanonisError> {
        let result = self.quick_send(
            "LockIn.DemodRTSignalsGet",
            vec![NanonisValue::I32(demodulator_num)],
            vec!["i"],
            vec!["I"],
        )?;
        if let Some(val) = result.first() {
            RTSignalMode::try_from(val.as_u32()?)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
