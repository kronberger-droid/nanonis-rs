pub mod types;
pub use types::*;

use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

impl NanonisClient {
    /// Set the piezo tilt correction parameters.
    ///
    /// # Arguments
    /// * `tilt_x_deg` - Tilt angle correction in X direction (degrees)
    /// * `tilt_y_deg` - Tilt angle correction in Y direction (degrees)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.piezo_tilt_set(0.5, -0.3)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn piezo_tilt_set(&mut self, tilt_x_deg: f32, tilt_y_deg: f32) -> Result<(), NanonisError> {
        self.quick_send(
            "Piezo.TiltSet",
            vec![NanonisValue::F32(tilt_x_deg), NanonisValue::F32(tilt_y_deg)],
            vec!["f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the piezo tilt correction parameters.
    ///
    /// # Returns
    /// A [`TiltCorrection`] struct with current tilt angles.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_tilt_get(&mut self) -> Result<TiltCorrection, NanonisError> {
        let result = self.quick_send("Piezo.TiltGet", vec![], vec![], vec!["f", "f"])?;

        if result.len() >= 2 {
            Ok(TiltCorrection {
                tilt_x_deg: result[0].as_f32()?,
                tilt_y_deg: result[1].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the piezo range values for all 3 axes.
    ///
    /// Changing the range will also change the sensitivity
    /// (HV gain will remain unchanged).
    ///
    /// # Arguments
    /// * `range` - A [`PiezoRange`] struct with X, Y, Z range values in meters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_range_set(&mut self, range: &PiezoRange) -> Result<(), NanonisError> {
        self.quick_send(
            "Piezo.RangeSet",
            vec![
                NanonisValue::F32(range.range_x_m),
                NanonisValue::F32(range.range_y_m),
                NanonisValue::F32(range.range_z_m),
            ],
            vec!["f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the piezo range values for all 3 axes.
    ///
    /// # Returns
    /// A [`PiezoRange`] struct with current range values in meters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_range_get(&mut self) -> Result<PiezoRange, NanonisError> {
        let result = self.quick_send("Piezo.RangeGet", vec![], vec![], vec!["f", "f", "f"])?;

        if result.len() >= 3 {
            Ok(PiezoRange {
                range_x_m: result[0].as_f32()?,
                range_y_m: result[1].as_f32()?,
                range_z_m: result[2].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the piezo sensitivity values for all 3 axes.
    ///
    /// Changing the sensitivity will also change the range
    /// (HV gain will remain unchanged).
    ///
    /// # Arguments
    /// * `sensitivity` - A [`PiezoSensitivity`] struct with X, Y, Z values in m/V
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_sens_set(&mut self, sensitivity: &PiezoSensitivity) -> Result<(), NanonisError> {
        self.quick_send(
            "Piezo.SensSet",
            vec![
                NanonisValue::F32(sensitivity.sens_x_m_per_v),
                NanonisValue::F32(sensitivity.sens_y_m_per_v),
                NanonisValue::F32(sensitivity.sens_z_m_per_v),
            ],
            vec!["f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the piezo sensitivity values for all 3 axes.
    ///
    /// # Returns
    /// A [`PiezoSensitivity`] struct with current sensitivity values in m/V.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_sens_get(&mut self) -> Result<PiezoSensitivity, NanonisError> {
        let result = self.quick_send("Piezo.SensGet", vec![], vec![], vec!["f", "f", "f"])?;

        if result.len() >= 3 {
            Ok(PiezoSensitivity {
                sens_x_m_per_v: result[0].as_f32()?,
                sens_y_m_per_v: result[1].as_f32()?,
                sens_z_m_per_v: result[2].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the drift compensation parameters.
    ///
    /// # Arguments
    /// * `config` - A [`DriftCompConfig`] struct with compensation settings
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::piezo::{DriftCompConfig, PiezoToggle};
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let config = DriftCompConfig {
    ///     enabled: PiezoToggle::On,
    ///     vx_m_s: 1e-12,
    ///     vy_m_s: 0.5e-12,
    ///     vz_m_s: 0.0,
    ///     saturation_limit: 0.1,
    /// };
    /// client.piezo_drift_comp_set(&config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn piezo_drift_comp_set(&mut self, config: &DriftCompConfig) -> Result<(), NanonisError> {
        self.quick_send(
            "Piezo.DriftCompSet",
            vec![
                NanonisValue::U32(config.enabled.into()),
                NanonisValue::F32(config.vx_m_s),
                NanonisValue::F32(config.vy_m_s),
                NanonisValue::F32(config.vz_m_s),
                NanonisValue::F32(config.saturation_limit),
            ],
            vec!["I", "f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the drift compensation settings and status.
    ///
    /// # Returns
    /// A [`DriftCompStatus`] struct with current settings and saturation status.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_drift_comp_get(&mut self) -> Result<DriftCompStatus, NanonisError> {
        let result = self.quick_send(
            "Piezo.DriftCompGet",
            vec![],
            vec![],
            vec!["I", "f", "f", "f", "I", "I", "I", "f"],
        )?;

        if result.len() >= 8 {
            Ok(DriftCompStatus {
                enabled: result[0].as_u32()? != 0,
                vx_m_s: result[1].as_f32()?,
                vy_m_s: result[2].as_f32()?,
                vz_m_s: result[3].as_f32()?,
                x_saturated: result[4].as_u32()? != 0,
                y_saturated: result[5].as_u32()? != 0,
                z_saturated: result[6].as_u32()? != 0,
                saturation_limit: result[7].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get the piezo calibration values for all 3 axes.
    ///
    /// The calibration returned is for the low voltage signals (±10V)
    /// before the HV amplifier.
    ///
    /// # Returns
    /// A [`PiezoSensitivity`] struct with calibration values in m/V.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_calibr_get(&mut self) -> Result<PiezoSensitivity, NanonisError> {
        let result = self.quick_send("Piezo.CalibrGet", vec![], vec![], vec!["f", "f", "f"])?;

        if result.len() >= 3 {
            Ok(PiezoSensitivity {
                sens_x_m_per_v: result[0].as_f32()?,
                sens_y_m_per_v: result[1].as_f32()?,
                sens_z_m_per_v: result[2].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get the HVA (High Voltage Amplifier) gain information.
    ///
    /// If HVA gain readout is not enabled, this function returns a warning.
    ///
    /// # Returns
    /// A [`HVAInfo`] struct with gain values and enabled status.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_hva_info_get(&mut self) -> Result<HVAInfo, NanonisError> {
        let result = self.quick_send(
            "Piezo.HVAInfoGet",
            vec![],
            vec![],
            vec!["f", "f", "f", "f", "I", "I", "I"],
        )?;

        if result.len() >= 7 {
            Ok(HVAInfo {
                gain_aux: result[0].as_f32()?,
                gain_x: result[1].as_f32()?,
                gain_y: result[2].as_f32()?,
                gain_z: result[3].as_f32()?,
                xy_enabled: result[4].as_u32()? != 0,
                z_enabled: result[5].as_u32()? != 0,
                aux_enabled: result[6].as_u32()? != 0,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get the HVA status LED indicators.
    ///
    /// # Returns
    /// A [`HVAStatusLED`] struct with LED status indicators.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_hva_status_led_get(&mut self) -> Result<HVAStatusLED, NanonisError> {
        let result = self.quick_send(
            "Piezo.HVAStatusLEDGet",
            vec![],
            vec![],
            vec!["I", "I", "I", "I"],
        )?;

        if result.len() >= 4 {
            Ok(HVAStatusLED {
                overheated: result[0].as_u32()? != 0,
                hv_supply: result[1].as_u32()? != 0,
                high_temperature: result[2].as_u32()? != 0,
                output_connector: result[3].as_u32()? != 0,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the XYZ voltage limits.
    ///
    /// # Arguments
    /// * `enable` - Enable/disable limits (use [`PiezoToggle`])
    /// * `limits` - A [`XYZLimits`] struct with voltage limits
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_xyz_limits_set(
        &mut self,
        enable: PiezoToggle,
        limits: &XYZLimits,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Piezo.XYZLimitsSet",
            vec![
                NanonisValue::U16(enable.into()),
                NanonisValue::F32(limits.x_low_v),
                NanonisValue::F32(limits.x_high_v),
                NanonisValue::F32(limits.y_low_v),
                NanonisValue::F32(limits.y_high_v),
                NanonisValue::F32(limits.z_low_v),
                NanonisValue::F32(limits.z_high_v),
            ],
            vec!["H", "f", "f", "f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the XYZ voltage limits.
    ///
    /// # Returns
    /// A [`XYZLimits`] struct with current voltage limits.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_xyz_limits_get(&mut self) -> Result<XYZLimits, NanonisError> {
        let result = self.quick_send(
            "Piezo.XYZLimitsGet",
            vec![],
            vec![],
            vec!["H", "f", "f", "f", "f", "f", "f"],
        )?;

        if result.len() >= 7 {
            Ok(XYZLimits {
                enabled: result[0].as_u16()? != 0,
                x_low_v: result[1].as_f32()?,
                x_high_v: result[2].as_f32()?,
                y_low_v: result[3].as_f32()?,
                y_high_v: result[4].as_f32()?,
                z_low_v: result[5].as_f32()?,
                z_high_v: result[6].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Enable or disable hysteresis compensation.
    ///
    /// # Arguments
    /// * `enabled` - True to enable, false to disable
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_hyst_on_off_set(&mut self, enabled: bool) -> Result<(), NanonisError> {
        let flag = if enabled { 1u32 } else { 0u32 };
        self.quick_send(
            "Piezo.HystOnOffSet",
            vec![NanonisValue::U32(flag)],
            vec!["I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get hysteresis compensation enabled status.
    ///
    /// # Returns
    /// True if hysteresis compensation is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_hyst_on_off_get(&mut self) -> Result<bool, NanonisError> {
        let result = self.quick_send("Piezo.HystOnOffGet", vec![], vec![], vec!["I"])?;

        if !result.is_empty() {
            Ok(result[0].as_u32()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set and apply the hysteresis compensation values.
    ///
    /// # Arguments
    /// * `values` - A [`HysteresisValues`] struct with hysteresis points
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_hyst_vals_set(&mut self, values: &HysteresisValues) -> Result<(), NanonisError> {
        self.quick_send(
            "Piezo.HystValsSet",
            vec![
                NanonisValue::I32(values.fast_axis.x_points.len() as i32),
                NanonisValue::ArrayF32(values.fast_axis.x_points.clone()),
                NanonisValue::I32(values.fast_axis.y_points.len() as i32),
                NanonisValue::ArrayF32(values.fast_axis.y_points.clone()),
                NanonisValue::I32(values.slow_axis.x_points.len() as i32),
                NanonisValue::ArrayF32(values.slow_axis.x_points.clone()),
                NanonisValue::I32(values.slow_axis.y_points.len() as i32),
                NanonisValue::ArrayF32(values.slow_axis.y_points.clone()),
            ],
            vec!["i", "*f", "i", "*f", "i", "*f", "i", "*f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the hysteresis compensation values.
    ///
    /// # Returns
    /// A [`HysteresisValues`] struct with current hysteresis points.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn piezo_hyst_vals_get(&mut self) -> Result<HysteresisValues, NanonisError> {
        let result = self.quick_send(
            "Piezo.HystValsGet",
            vec![],
            vec![],
            vec!["i", "*f", "i", "*f", "i", "*f", "i", "*f"],
        )?;

        if result.len() >= 8 {
            Ok(HysteresisValues {
                fast_axis: HysteresisAxisPoints {
                    x_points: result[1].as_f32_array()?.to_vec(),
                    y_points: result[3].as_f32_array()?.to_vec(),
                },
                slow_axis: HysteresisAxisPoints {
                    x_points: result[5].as_f32_array()?.to_vec(),
                    y_points: result[7].as_f32_array()?.to_vec(),
                },
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Load hysteresis compensation values from a CSV file.
    ///
    /// # Arguments
    /// * `file_path` - Path to the CSV file to load
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or file cannot be loaded.
    pub fn piezo_hyst_file_load(&mut self, file_path: &str) -> Result<(), NanonisError> {
        self.quick_send(
            "Piezo.HystFileLoad",
            vec![NanonisValue::String(file_path.to_string())],
            vec!["+*c"],
            vec![],
        )?;
        Ok(())
    }

    /// Save hysteresis compensation values to a CSV file.
    ///
    /// # Arguments
    /// * `file_path` - Path to the CSV file to save
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails or file cannot be saved.
    pub fn piezo_hyst_file_save(&mut self, file_path: &str) -> Result<(), NanonisError> {
        self.quick_send(
            "Piezo.HystFileSave",
            vec![NanonisValue::String(file_path.to_string())],
            vec!["+*c"],
            vec![],
        )?;
        Ok(())
    }
}
