// ==================== User Output Types ====================

use crate::types::NanonisValue;

/// Output mode for user output channels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    /// User Output mode
    UserOutput = 0,
    /// Monitor mode
    Monitor = 1,
    /// Calculated Signal mode
    CalcSignal = 2,
    /// Override mode (returned by ModeGet only)
    Override = 3,
}

impl From<OutputMode> for u16 {
    fn from(mode: OutputMode) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for OutputMode {
    type Error = crate::error::NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OutputMode::UserOutput),
            1 => Ok(OutputMode::Monitor),
            2 => Ok(OutputMode::CalcSignal),
            3 => Ok(OutputMode::Override),
            _ => Err(crate::error::NanonisError::Protocol(format!(
                "Invalid output mode: {}",
                value
            ))),
        }
    }
}

impl From<OutputMode> for NanonisValue {
    fn from(mode: OutputMode) -> Self {
        NanonisValue::U16(mode as u16)
    }
}

/// Math operation for calculated signals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcOperation {
    /// No operation
    None = 0,
    /// Add two signals
    Add = 1,
    /// Subtract signal 2 from signal 1
    Subtract = 2,
    /// Multiply two signals
    Multiply = 3,
    /// Divide signal 1 by signal 2
    Divide = 4,
    /// Logarithm of signal 1
    Log = 6,
}

impl From<CalcOperation> for u16 {
    fn from(op: CalcOperation) -> Self {
        op as u16
    }
}

impl TryFrom<u16> for CalcOperation {
    type Error = crate::error::NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CalcOperation::None),
            1 => Ok(CalcOperation::Add),
            2 => Ok(CalcOperation::Subtract),
            3 => Ok(CalcOperation::Multiply),
            4 => Ok(CalcOperation::Divide),
            6 => Ok(CalcOperation::Log),
            _ => Err(crate::error::NanonisError::Protocol(format!(
                "Invalid calculation operation: {}",
                value
            ))),
        }
    }
}

impl From<CalcOperation> for NanonisValue {
    fn from(op: CalcOperation) -> Self {
        NanonisValue::U16(op as u16)
    }
}

/// Configuration for a calculated signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalcSignalConfig {
    /// First signal index (0-127)
    pub signal_1: u16,
    /// Math operation to perform
    pub operation: CalcOperation,
    /// Second signal index (0-127)
    pub signal_2: u16,
}

impl CalcSignalConfig {
    pub fn new(signal_1: u16, operation: CalcOperation, signal_2: u16) -> Self {
        Self {
            signal_1,
            operation,
            signal_2,
        }
    }
}
