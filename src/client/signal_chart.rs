use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Signal chart channel configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct SignalChartChannels {
    /// Channel A signal index (-1 for none)
    pub channel_a_index: i32,
    /// Channel B signal index (-1 for none)
    pub channel_b_index: i32,
}

impl NanonisClient {
    /// Open the signal chart module.
    ///
    /// This opens the signal chart interface in the Nanonis software.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.signal_chart_open()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn signal_chart_open(&mut self) -> Result<(), NanonisError> {
        self.quick_send("SignalChart.Open", vec![], vec![], vec![])?;
        Ok(())
    }

    /// Set the signal chart channels.
    ///
    /// Sets the signal indices for channels A and B in the signal chart.
    /// Use -1 to disable a channel.
    ///
    /// # Arguments
    /// * `channels` - A [`SignalChartChannels`] struct with channel indices
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::signal_chart::SignalChartChannels;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let channels = SignalChartChannels {
    ///     channel_a_index: 0,  // First signal
    ///     channel_b_index: 1,  // Second signal
    /// };
    /// client.signal_chart_chs_set(&channels)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn signal_chart_chs_set(
        &mut self,
        channels: &SignalChartChannels,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "SignalChart.ChsSet",
            vec![
                NanonisValue::I32(channels.channel_a_index),
                NanonisValue::I32(channels.channel_b_index),
            ],
            vec!["i", "i"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the signal chart channels.
    ///
    /// Returns the current signal indices for channels A and B.
    ///
    /// # Returns
    /// A [`SignalChartChannels`] struct with current channel indices.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let channels = client.signal_chart_chs_get()?;
    /// println!("Channel A: {}, Channel B: {}", channels.channel_a_index, channels.channel_b_index);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn signal_chart_chs_get(&mut self) -> Result<SignalChartChannels, NanonisError> {
        let result = self.quick_send("SignalChart.ChsGet", vec![], vec![], vec!["i", "i"])?;

        if result.len() >= 2 {
            Ok(SignalChartChannels {
                channel_a_index: result[0].as_i32()?,
                channel_b_index: result[1].as_i32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }
}
