use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Link angles mode for OC Sync module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkAnglesMode {
    #[default]
    NoChange = 0,
    Link = 1,
    Unlink = 2,
}

impl From<LinkAnglesMode> for u32 {
    fn from(mode: LinkAnglesMode) -> Self {
        mode as u32
    }
}

/// OC Sync angle configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct OCSyncAngles {
    /// Channel 1 on angle (degrees)
    pub ch1_on_deg: f32,
    /// Channel 1 off angle (degrees)
    pub ch1_off_deg: f32,
    /// Channel 2 on angle (degrees)
    pub ch2_on_deg: f32,
    /// Channel 2 off angle (degrees)
    pub ch2_off_deg: f32,
}

/// OC Sync link status.
#[derive(Debug, Clone, Copy, Default)]
pub struct OCSyncLinkStatus {
    /// Channel 1 link status (true = linked)
    pub ch1_linked: bool,
    /// Channel 2 link status (true = linked)
    pub ch2_linked: bool,
}

impl NanonisClient {
    // ==================== OC Sync ====================

    /// Set the angle values for digital channels 1 and 2.
    ///
    /// The On angle is the excitation angle at which the digital channel goes high.
    /// The Off angle is the excitation angle at which the digital channel goes low.
    ///
    /// # Arguments
    /// * `angles` - Angle configuration
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn oc_sync_angles_set(&mut self, angles: &OCSyncAngles) -> Result<(), NanonisError> {
        self.quick_send(
            "OCSync.AnglesSet",
            vec![
                NanonisValue::F32(angles.ch1_on_deg),
                NanonisValue::F32(angles.ch1_off_deg),
                NanonisValue::F32(angles.ch2_on_deg),
                NanonisValue::F32(angles.ch2_off_deg),
            ],
            vec!["f", "f", "f", "f"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the angle values for digital channels 1 and 2.
    ///
    /// # Returns
    /// Angle configuration.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn oc_sync_angles_get(&mut self) -> Result<OCSyncAngles, NanonisError> {
        let result =
            self.quick_send("OCSync.AnglesGet", vec![], vec![], vec!["f", "f", "f", "f"])?;

        Ok(OCSyncAngles {
            ch1_on_deg: result[0].as_f32()?,
            ch1_off_deg: result[1].as_f32()?,
            ch2_on_deg: result[2].as_f32()?,
            ch2_off_deg: result[3].as_f32()?,
        })
    }

    /// Set the link angles status for channels 1 and 2.
    ///
    /// When linked, the difference between Off and On angles is kept constant.
    ///
    /// # Arguments
    /// * `ch1_mode` - Channel 1 link mode
    /// * `ch2_mode` - Channel 2 link mode
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn oc_sync_link_angles_set(
        &mut self,
        ch1_mode: LinkAnglesMode,
        ch2_mode: LinkAnglesMode,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "OCSync.LinkAnglesSet",
            vec![
                NanonisValue::U32(ch1_mode.into()),
                NanonisValue::U32(ch2_mode.into()),
            ],
            vec!["I", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Get the link angles status for channels 1 and 2.
    ///
    /// # Returns
    /// Link status for both channels.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn oc_sync_link_angles_get(&mut self) -> Result<OCSyncLinkStatus, NanonisError> {
        let result = self.quick_send("OCSync.LinkAnglesGet", vec![], vec![], vec!["I", "I"])?;

        Ok(OCSyncLinkStatus {
            ch1_linked: result[0].as_u32()? != 0,
            ch2_linked: result[1].as_u32()? != 0,
        })
    }
}
