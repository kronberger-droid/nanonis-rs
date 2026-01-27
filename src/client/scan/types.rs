use crate::error::NanonisError;
use crate::types::Position;

// ==================== Scan Types ====================

#[derive(Debug, Clone, Copy)]
pub struct ScanFrame {
    pub center: Position,
    pub width_m: f32,
    pub height_m: f32,
    pub angle_deg: f32,
}

impl ScanFrame {
    pub fn new(center: Position, width_m: f32, height_m: f32, angle_deg: f32) -> Self {
        Self {
            center,
            width_m,
            height_m,
            angle_deg,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanAction {
    Start = 0,
    Stop = 1,
    Pause = 2,
    Resume = 3,
    Freeze = 4,
    Unfreeze = 5,
    GoToCenter = 6,
}

impl From<ScanAction> for u16 {
    fn from(action: ScanAction) -> Self {
        action as u16
    }
}

impl TryFrom<u16> for ScanAction {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScanAction::Start),
            1 => Ok(ScanAction::Stop),
            2 => Ok(ScanAction::Pause),
            3 => Ok(ScanAction::Resume),
            4 => Ok(ScanAction::Freeze),
            5 => Ok(ScanAction::Unfreeze),
            6 => Ok(ScanAction::GoToCenter),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid scan action: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanDirection {
    Down = 0,
    Up = 1,
}

impl From<ScanDirection> for u32 {
    fn from(direction: ScanDirection) -> Self {
        direction as u32
    }
}

impl TryFrom<u32> for ScanDirection {
    type Error = NanonisError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScanDirection::Down),
            1 => Ok(ScanDirection::Up),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid scan direction: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScanConfig {
    pub forward_linear_speed_m_s: f32,
    pub backward_linear_speed_m_s: f32,
    pub forward_time_per_line_s: f32,
    pub backward_time_per_line_s: f32,
    pub keep_parameter_constant: u16,
    pub speed_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct ScanProps {
    /// Continuous scan: whether scan continues after frame completion
    pub continuous_scan: bool,
    /// Bouncy scan: whether scan direction changes after frame completion
    pub bouncy_scan: bool,
    /// Autosave mode: All, Next, or Off
    pub autosave: AutosaveMode,
    /// Base name for saved images
    pub series_name: String,
    /// Comment saved in file
    pub comment: String,
    /// Module names whose parameters are saved in image header
    pub modules_names: Vec<String>,
    /// Number of parameters per module (read-only, returned by GET)
    pub num_params_per_module: Vec<i32>,
    /// Parameters for each module - 2D array: rows = modules, columns = parameters (read-only, returned by GET)
    pub parameters: Vec<Vec<String>>,
    /// Autopaste mode: All, Next, or Off
    pub autopaste: AutopasteMode,
}

impl ScanProps {
    /// Create a builder for modifying scan properties.
    /// Use this to set only the properties you want to change.
    pub fn to_builder(&self) -> ScanPropsBuilder {
        ScanPropsBuilder {
            continuous_scan: None,
            bouncy_scan: None,
            autosave: None,
            series_name: None,
            comment: None,
            modules_names: None,
            autopaste: None,
        }
    }
}

/// Builder for setting scan properties.
/// Use `None` for fields that should not be changed.
#[derive(Debug, Clone, Default)]
pub struct ScanPropsBuilder {
    /// Continuous scan: None = no change, Some(true) = On, Some(false) = Off
    pub continuous_scan: Option<bool>,
    /// Bouncy scan: None = no change, Some(true) = On, Some(false) = Off
    pub bouncy_scan: Option<bool>,
    /// Autosave mode: None = no change
    pub autosave: Option<AutosaveMode>,
    /// Base name for saved images: None = no change
    pub series_name: Option<String>,
    /// Comment saved in file: None = no change
    pub comment: Option<String>,
    /// Module names whose parameters are saved in image header: None = no change
    pub modules_names: Option<Vec<String>>,
    /// Autopaste mode: None = no change
    pub autopaste: Option<AutopasteMode>,
}

impl ScanPropsBuilder {
    /// Create a new builder with all fields set to None (no changes)
    pub fn new() -> Self {
        Self::default()
    }

    /// Set continuous scan mode
    pub fn continuous_scan(mut self, value: bool) -> Self {
        self.continuous_scan = Some(value);
        self
    }

    /// Set bouncy scan mode
    pub fn bouncy_scan(mut self, value: bool) -> Self {
        self.bouncy_scan = Some(value);
        self
    }

    /// Set autosave mode
    pub fn autosave(mut self, mode: AutosaveMode) -> Self {
        self.autosave = Some(mode);
        self
    }

    /// Set series name
    pub fn series_name(mut self, name: impl Into<String>) -> Self {
        self.series_name = Some(name.into());
        self
    }

    /// Set comment
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Set modules names
    pub fn modules_names(mut self, names: Vec<String>) -> Self {
        self.modules_names = Some(names);
        self
    }

    /// Set autopaste mode
    pub fn autopaste(mut self, mode: AutopasteMode) -> Self {
        self.autopaste = Some(mode);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutosaveMode {
    /// Save all future images automatically
    All = 0,
    /// Save only the next frame
    Next = 1,
    /// Autosave is disabled
    Off = 2,
}

impl From<AutosaveMode> for u32 {
    fn from(mode: AutosaveMode) -> Self {
        // For SET: 0=no change, 1=All, 2=Next, 3=Off
        match mode {
            AutosaveMode::All => 1,
            AutosaveMode::Next => 2,
            AutosaveMode::Off => 3,
        }
    }
}

impl TryFrom<u32> for AutosaveMode {
    type Error = NanonisError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        // For GET: 0=All, 1=Next, 2=Off
        match value {
            0 => Ok(AutosaveMode::All),
            1 => Ok(AutosaveMode::Next),
            2 => Ok(AutosaveMode::Off),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid autosave mode: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutopasteMode {
    /// Paste all future images automatically
    All = 0,
    /// Paste only the next frame
    Next = 1,
    /// Autopaste is disabled
    Off = 2,
}

impl From<AutopasteMode> for u32 {
    fn from(mode: AutopasteMode) -> Self {
        // For SET: 0=no change, 1=All, 2=Next, 3=Off
        match mode {
            AutopasteMode::All => 1,
            AutopasteMode::Next => 2,
            AutopasteMode::Off => 3,
        }
    }
}

impl TryFrom<u32> for AutopasteMode {
    type Error = NanonisError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        // For GET: 0=All, 1=Next, 2=Off
        match value {
            0 => Ok(AutopasteMode::All),
            1 => Ok(AutopasteMode::Next),
            2 => Ok(AutopasteMode::Off),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid autopaste mode: {}",
                value
            ))),
        }
    }
}
