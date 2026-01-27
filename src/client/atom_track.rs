use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Atom Tracking control type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ATControl {
    /// Modulation control
    #[default]
    Modulation = 0,
    /// Controller
    Controller = 1,
    /// Drift measurement
    DriftMeasurement = 2,
}

impl From<ATControl> for u16 {
    fn from(ctrl: ATControl) -> Self {
        ctrl as u16
    }
}

/// Quick compensation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuickCompType {
    /// Tilt compensation
    #[default]
    Tilt = 0,
    /// Drift compensation
    Drift = 1,
}

impl From<QuickCompType> for u16 {
    fn from(comp: QuickCompType) -> Self {
        comp as u16
    }
}

/// Atom Tracking properties.
#[derive(Debug, Clone, Copy, Default)]
pub struct AtomTrackProps {
    /// Integral gain of the controller
    pub integral_gain: f32,
    /// Modulation frequency in Hz
    pub frequency_hz: f32,
    /// Modulation amplitude in meters
    pub amplitude_m: f32,
    /// Modulation phase in degrees
    pub phase_deg: f32,
    /// Switch off delay in seconds
    pub switch_off_delay_s: f32,
}

impl NanonisClient {
    /// Turn the selected Atom Tracking control on or off.
    ///
    /// # Arguments
    /// * `control` - Which control to switch (modulation, controller, or drift measurement)
    /// * `enabled` - True to enable, false to disable
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::atom_track::ATControl;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// client.atom_track_ctrl_set(ATControl::Controller, true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn atom_track_ctrl_set(
        &mut self,
        control: ATControl,
        enabled: bool,
    ) -> Result<(), NanonisError> {
        let status = if enabled { 1u16 } else { 0u16 };
        self.quick_send(
            "AtomTrack.CtrlSet",
            vec![
                NanonisValue::U16(control.into()),
                NanonisValue::U16(status),
            ],
            vec!["H", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the status of the selected Atom Tracking control.
    ///
    /// # Arguments
    /// * `control` - Which control to query
    ///
    /// # Returns
    /// True if the control is enabled.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn atom_track_status_get(&mut self, control: ATControl) -> Result<bool, NanonisError> {
        let result = self.quick_send(
            "AtomTrack.StatusGet",
            vec![NanonisValue::U16(control.into())],
            vec!["H"],
            vec!["H"],
        )?;

        if !result.is_empty() {
            Ok(result[0].as_u16()? != 0)
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Set the Atom Tracking parameters.
    ///
    /// # Arguments
    /// * `props` - An [`AtomTrackProps`] struct with tracking parameters
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    /// use nanonis_rs::atom_track::AtomTrackProps;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// let props = AtomTrackProps {
    ///     integral_gain: 10.0,
    ///     frequency_hz: 200.0,
    ///     amplitude_m: 1e-10,
    ///     phase_deg: 0.0,
    ///     switch_off_delay_s: 0.1,
    /// };
    /// client.atom_track_props_set(&props)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn atom_track_props_set(&mut self, props: &AtomTrackProps) -> Result<(), NanonisError> {
        self.quick_send(
            "AtomTrack.PropsSet",
            vec![
                NanonisValue::F32(props.integral_gain),
                NanonisValue::F32(props.frequency_hz),
                NanonisValue::F32(props.amplitude_m),
                NanonisValue::F32(props.phase_deg),
                NanonisValue::F32(props.switch_off_delay_s),
            ],
            vec!["f", "f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the Atom Tracking parameters.
    ///
    /// # Returns
    /// An [`AtomTrackProps`] struct with current tracking parameters.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn atom_track_props_get(&mut self) -> Result<AtomTrackProps, NanonisError> {
        let result = self.quick_send(
            "AtomTrack.PropsGet",
            vec![],
            vec![],
            vec!["f", "f", "f", "f", "f"],
        )?;

        if result.len() >= 5 {
            Ok(AtomTrackProps {
                integral_gain: result[0].as_f32()?,
                frequency_hz: result[1].as_f32()?,
                amplitude_m: result[2].as_f32()?,
                phase_deg: result[3].as_f32()?,
                switch_off_delay_s: result[4].as_f32()?,
            })
        } else {
            Err(NanonisError::Protocol("Invalid response".to_string()))
        }
    }

    /// Start the Tilt or Drift compensation.
    ///
    /// # Arguments
    /// * `comp_type` - Which compensation to start (tilt or drift)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn atom_track_quick_comp_start(
        &mut self,
        comp_type: QuickCompType,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "AtomTrack.QuickCompStart",
            vec![NanonisValue::U16(comp_type.into())],
            vec!["H"],
            vec![],
        )?;
        Ok(())
    }

    /// Apply the Drift measurement to the Drift compensation and turn on compensation.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn atom_track_drift_comp(&mut self) -> Result<(), NanonisError> {
        self.quick_send("AtomTrack.DriftComp", vec![], vec![], vec![])?;
        Ok(())
    }
}
