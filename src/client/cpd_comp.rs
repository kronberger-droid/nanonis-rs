use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// CPD compensation parameters.
#[derive(Debug, Clone, Copy, Default)]
pub struct CPDCompParams {
    /// Sweep speed in Hz
    pub speed_hz: f32,
    /// Voltage range in volts
    pub range_v: f32,
    /// Number of averaging cycles
    pub averaging: i32,
}

/// CPD compensation fit coefficients.
///
/// The fit model is: df = a(U-Uo)^2 + b(U-Uo) + c
/// where Uo is the bias voltage.
#[derive(Debug, Clone, Copy, Default)]
pub struct CPDFitCoefficients {
    /// Quadratic coefficient 'a'
    pub a: f64,
    /// Linear coefficient 'b'
    pub b: f64,
}

/// CPD compensation sweep data for one direction.
#[derive(Debug, Clone, Default)]
pub struct CPDSweepData {
    /// Bias voltage data (X axis)
    pub bias_v: Vec<f32>,
    /// Frequency shift data
    pub freq_shift: Vec<f32>,
    /// Frequency shift fit data
    pub freq_shift_fit: Vec<f32>,
}

/// Complete CPD compensation data.
#[derive(Debug, Clone, Default)]
pub struct CPDCompData {
    /// Forward sweep data
    pub forward: CPDSweepData,
    /// Backward sweep data
    pub backward: CPDSweepData,
    /// CPD estimate in volts
    pub cpd_estimate_v: f32,
    /// Fit coefficients
    pub fit_coefficients: CPDFitCoefficients,
}

impl NanonisClient {
    /// Open the CPD compensation module.
    ///
    /// This opens the Contact Potential Difference compensation interface.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.cpd_comp_open()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn cpd_comp_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("CPDComp.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Close the CPD compensation module.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.cpd_comp_close()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn cpd_comp_close(&mut self) -> Result<(), NanonisError> {
        self.quick_send("CPDComp.Close", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set the CPD compensation parameters.
    ///
    /// # Arguments
    /// * `params` - A [`CPDCompParams`] struct with compensation parameters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::cpd_comp::CPDCompParams;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let params = CPDCompParams {
    ///     speed_hz: 10.0,
    ///     range_v: 2.0,
    ///     averaging: 5,
    /// };
    /// client.cpd_comp_params_set(&params)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn cpd_comp_params_set(&mut self, params: &CPDCompParams) -> Result<(), NanonisError> {
        self.quick_send(
            "CPDComp.ParamsSet",
            vec![
                NanonisValue::F32(params.speed_hz),
                NanonisValue::F32(params.range_v),
                NanonisValue::I32(params.averaging),
            ],
            vec!["f", "f", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the CPD compensation parameters.
    ///
    /// # Returns
    /// A [`CPDCompParams`] struct with current parameters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let params = client.cpd_comp_params_get()?;
    /// println!("Speed: {} Hz, Range: {} V", params.speed_hz, params.range_v);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn cpd_comp_params_get(&mut self) -> Result<CPDCompParams, NanonisError> {
        let result = self.quick_send("CPDComp.ParamsGet", vec![], vec![], vec!["f", "f", "i"])?;

        if result.len() >= 3 {
            Ok(CPDCompParams {
                speed_hz: result[0].as_f32()?,
                range_v: result[1].as_f32()?,
                averaging: result[2].as_i32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Get the CPD compensation data.
    ///
    /// Returns the graph data, CPD estimate, and fit coefficients from the
    /// CPD compensation module.
    ///
    /// # Returns
    /// A [`CPDCompData`] struct with sweep data and fit results.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let data = client.cpd_comp_data_get()?;
    /// println!("CPD estimate: {} V", data.cpd_estimate_v);
    /// println!("Fit: a={}, b={}", data.fit_coefficients.a, data.fit_coefficients.b);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn cpd_comp_data_get(&mut self) -> Result<CPDCompData, NanonisError> {
        let result = self.quick_send(
            "CPDComp.DataGet",
            vec![],
            vec![],
            vec![
                "i", "*f", "*f", "*f", // Forward: size, bias, freq_shift, fit
                "i", "*f", "*f", "*f", // Backward: size, bias, freq_shift, fit
                "f", "d", "d", // CPD estimate, a, b coefficients
            ],
        )?;

        if result.len() >= 11 {
            let forward_bias = result[1].as_f32_array()?.to_vec();
            let forward_freq = result[2].as_f32_array()?.to_vec();
            let forward_fit = result[3].as_f32_array()?.to_vec();

            let backward_bias = result[5].as_f32_array()?.to_vec();
            let backward_freq = result[6].as_f32_array()?.to_vec();
            let backward_fit = result[7].as_f32_array()?.to_vec();

            Ok(CPDCompData {
                forward: CPDSweepData {
                    bias_v: forward_bias,
                    freq_shift: forward_freq,
                    freq_shift_fit: forward_fit,
                },
                backward: CPDSweepData {
                    bias_v: backward_bias,
                    freq_shift: backward_freq,
                    freq_shift_fit: backward_fit,
                },
                cpd_estimate_v: result[8].as_f32()?,
                fit_coefficients: CPDFitCoefficients {
                    a: result[9].as_f64()?,
                    b: result[10].as_f64()?,
                },
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
