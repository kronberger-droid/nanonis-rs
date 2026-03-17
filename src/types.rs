use crate::error::NanonisError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== Helpers ====================

/// Safely convert an f32 seconds value (from the server) to a Duration.
/// Returns a Protocol error if the value is negative, infinite, or NaN.
pub(crate) fn duration_from_secs_f32(secs: f32) -> Result<Duration, NanonisError> {
    if secs.is_nan() || secs.is_infinite() || secs < 0.0 {
        return Err(NanonisError::Protocol(format!(
            "Invalid duration value: {secs}"
        )));
    }
    Ok(Duration::from_secs_f32(secs))
}

// ==================== Core Protocol Value Type ====================

#[derive(Debug, Clone)]
pub enum NanonisValue {
    U8(u8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    F32(f32),
    F64(f64),
    String(String),
    ArrayU8(Vec<u8>),
    ArrayU32(Vec<u32>),
    ArrayI32(Vec<i32>),
    ArrayF32(Vec<f32>),
    ArrayF64(Vec<f64>),
    ArrayString(Vec<String>),
    Array2DF32(Vec<Vec<f32>>),
}

// From implementations for NanonisValue
impl From<f32> for NanonisValue {
    fn from(value: f32) -> Self {
        NanonisValue::F32(value)
    }
}

impl From<f64> for NanonisValue {
    fn from(value: f64) -> Self {
        NanonisValue::F64(value)
    }
}

impl From<u16> for NanonisValue {
    fn from(value: u16) -> Self {
        NanonisValue::U16(value)
    }
}

impl From<u32> for NanonisValue {
    fn from(value: u32) -> Self {
        NanonisValue::U32(value)
    }
}

impl From<i16> for NanonisValue {
    fn from(value: i16) -> Self {
        NanonisValue::I16(value)
    }
}

impl From<i32> for NanonisValue {
    fn from(value: i32) -> Self {
        NanonisValue::I32(value)
    }
}

impl From<String> for NanonisValue {
    fn from(value: String) -> Self {
        NanonisValue::String(value)
    }
}

impl From<Vec<f32>> for NanonisValue {
    fn from(value: Vec<f32>) -> Self {
        NanonisValue::ArrayF32(value)
    }
}

impl From<Vec<String>> for NanonisValue {
    fn from(value: Vec<String>) -> Self {
        NanonisValue::ArrayString(value)
    }
}

impl From<Vec<i32>> for NanonisValue {
    fn from(value: Vec<i32>) -> Self {
        NanonisValue::ArrayI32(value)
    }
}

// TryFrom implementations
impl TryFrom<NanonisValue> for f32 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::F32(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!("Expected f32, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for f64 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::F64(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!("Expected f64, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for u16 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::U16(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!("Expected u16, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for u32 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::U32(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!("Expected u32, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for i16 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::I16(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!("Expected i16, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for i32 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::I32(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!("Expected i32, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for Vec<f32> {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::ArrayF32(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!(
                "Expected Vec<f32>, got {value:?}"
            ))),
        }
    }
}

impl TryFrom<NanonisValue> for Vec<String> {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::ArrayString(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!(
                "Expected Vec<String>, got {value:?}"
            ))),
        }
    }
}

impl TryFrom<NanonisValue> for Vec<i32> {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::ArrayI32(v) => Ok(v),
            _ => Err(NanonisError::Protocol(format!(
                "Expected Vec<i32>, got {value:?}"
            ))),
        }
    }
}

// Convenience methods
impl NanonisValue {
    pub fn as_f32(&self) -> Result<f32, NanonisError> {
        match self {
            NanonisValue::F32(v) => Ok(*v),
            _ => Err(NanonisError::Protocol(format!("Expected f32, got {self:?}"))),
        }
    }

    pub fn as_f64(&self) -> Result<f64, NanonisError> {
        match self {
            NanonisValue::F64(v) => Ok(*v),
            _ => Err(NanonisError::Protocol(format!("Expected f64, got {self:?}"))),
        }
    }

    pub fn as_u16(&self) -> Result<u16, NanonisError> {
        match self {
            NanonisValue::U16(v) => Ok(*v),
            _ => Err(NanonisError::Protocol(format!("Expected u16, got {self:?}"))),
        }
    }

    pub fn as_u32(&self) -> Result<u32, NanonisError> {
        match self {
            NanonisValue::U32(v) => Ok(*v),
            _ => Err(NanonisError::Protocol(format!("Expected u32, got {self:?}"))),
        }
    }

    pub fn as_i16(&self) -> Result<i16, NanonisError> {
        match self {
            NanonisValue::I16(v) => Ok(*v),
            _ => Err(NanonisError::Protocol(format!("Expected i16, got {self:?}"))),
        }
    }

    pub fn as_i32(&self) -> Result<i32, NanonisError> {
        match self {
            NanonisValue::I32(v) => Ok(*v),
            _ => Err(NanonisError::Protocol(format!("Expected i32, got {self:?}"))),
        }
    }

    pub fn as_string_array(&self) -> Result<&[String], NanonisError> {
        match self {
            NanonisValue::ArrayString(arr) => Ok(arr),
            _ => Err(NanonisError::Protocol(format!(
                "Expected string array, got {self:?}"
            ))),
        }
    }

    pub fn as_f32_array(&self) -> Result<&[f32], NanonisError> {
        match self {
            NanonisValue::ArrayF32(arr) => Ok(arr),
            _ => Err(NanonisError::Protocol(format!(
                "Expected f32 array, got {self:?}"
            ))),
        }
    }

    pub fn as_f64_array(&self) -> Result<&[f64], NanonisError> {
        match self {
            NanonisValue::ArrayF64(arr) => Ok(arr),
            _ => Err(NanonisError::Protocol(format!(
                "Expected f64 array, got {self:?}"
            ))),
        }
    }

    pub fn as_i32_array(&self) -> Result<&[i32], NanonisError> {
        match self {
            NanonisValue::ArrayI32(arr) => Ok(arr),
            _ => Err(NanonisError::Protocol(format!(
                "Expected i32 array, got {self:?}"
            ))),
        }
    }

    pub fn as_u32_array(&self) -> Result<&[u32], NanonisError> {
        match self {
            NanonisValue::ArrayU32(arr) => Ok(arr),
            _ => Err(NanonisError::Protocol(format!(
                "Expected u32 array, got {self:?}"
            ))),
        }
    }

    pub fn as_string(&self) -> Result<&str, NanonisError> {
        match self {
            NanonisValue::String(s) => Ok(s),
            _ => Err(NanonisError::Protocol(format!("Expected string, got {self:?}"))),
        }
    }

    pub fn as_f32_2d_array(&self) -> Result<&Vec<Vec<f32>>, NanonisError> {
        match self {
            NanonisValue::Array2DF32(arr) => Ok(arr),
            _ => Err(NanonisError::Protocol(format!(
                "Expected 2D f32 array, got {self:?}"
            ))),
        }
    }
}

// ==================== Domain-Specific Types ====================
// Note: Domain-specific types have been moved to their respective modules:
// - Motor types -> client::motor::types
// - Scan types -> client::scan::types
// - Z-controller types -> client::z_ctrl::types
// - Bias types -> client::bias::types
// - Signal types -> client::signals::types
// - Oscilloscope types -> client::oscilloscope::types
// - TCP Logger types -> client::tcplog::types

// ==================== Position Types ====================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- duration_from_secs_f32 ----

    #[test]
    fn duration_valid_values() {
        assert_eq!(duration_from_secs_f32(0.0).unwrap(), Duration::from_secs_f32(0.0));
        assert_eq!(duration_from_secs_f32(1.5).unwrap(), Duration::from_secs_f32(1.5));
        assert_eq!(duration_from_secs_f32(0.001).unwrap(), Duration::from_secs_f32(0.001));
    }

    #[test]
    fn duration_rejects_negative() {
        assert!(duration_from_secs_f32(-1.0).is_err());
        assert!(duration_from_secs_f32(-0.001).is_err());
    }

    #[test]
    fn duration_rejects_nan() {
        assert!(duration_from_secs_f32(f32::NAN).is_err());
    }

    #[test]
    fn duration_rejects_infinity() {
        assert!(duration_from_secs_f32(f32::INFINITY).is_err());
        assert!(duration_from_secs_f32(f32::NEG_INFINITY).is_err());
    }

    // ---- NanonisValue From impls ----

    #[test]
    fn from_primitives() {
        assert!(matches!(NanonisValue::from(1.0f32), NanonisValue::F32(v) if v == 1.0));
        assert!(matches!(NanonisValue::from(2.0f64), NanonisValue::F64(v) if v == 2.0));
        assert!(matches!(NanonisValue::from(42u16), NanonisValue::U16(42)));
        assert!(matches!(NanonisValue::from(100u32), NanonisValue::U32(100)));
        assert!(matches!(NanonisValue::from(-5i16), NanonisValue::I16(-5)));
        assert!(matches!(NanonisValue::from(-10i32), NanonisValue::I32(-10)));
    }

    #[test]
    fn from_collections() {
        assert!(matches!(NanonisValue::from("hello".to_string()), NanonisValue::String(s) if s == "hello"));
        assert!(matches!(NanonisValue::from(vec![1.0f32, 2.0]), NanonisValue::ArrayF32(_)));
        assert!(matches!(NanonisValue::from(vec![1i32, 2]), NanonisValue::ArrayI32(_)));
    }

    // ---- NanonisValue TryFrom / as_* methods ----

    #[test]
    fn tryfrom_correct_types() {
        assert_eq!(f32::try_from(NanonisValue::F32(3.14)).unwrap(), 3.14);
        assert_eq!(f64::try_from(NanonisValue::F64(2.718)).unwrap(), 2.718);
        assert_eq!(u16::try_from(NanonisValue::U16(100)).unwrap(), 100);
        assert_eq!(u32::try_from(NanonisValue::U32(200)).unwrap(), 200);
        assert_eq!(i16::try_from(NanonisValue::I16(-5)).unwrap(), -5);
        assert_eq!(i32::try_from(NanonisValue::I32(-10)).unwrap(), -10);
    }

    #[test]
    fn tryfrom_wrong_types() {
        assert!(f32::try_from(NanonisValue::I32(1)).is_err());
        assert!(u16::try_from(NanonisValue::F32(1.0)).is_err());
        assert!(Vec::<f32>::try_from(NanonisValue::I32(1)).is_err());
        assert!(Vec::<String>::try_from(NanonisValue::F32(1.0)).is_err());
    }

    #[test]
    fn as_methods_correct() {
        assert_eq!(NanonisValue::F32(1.0).as_f32().unwrap(), 1.0);
        assert_eq!(NanonisValue::F64(2.0).as_f64().unwrap(), 2.0);
        assert_eq!(NanonisValue::U16(10).as_u16().unwrap(), 10);
        assert_eq!(NanonisValue::U32(20).as_u32().unwrap(), 20);
        assert_eq!(NanonisValue::I16(-3).as_i16().unwrap(), -3);
        assert_eq!(NanonisValue::I32(-7).as_i32().unwrap(), -7);
        assert_eq!(NanonisValue::String("hi".into()).as_string().unwrap(), "hi");
    }

    #[test]
    fn as_methods_wrong_type() {
        assert!(NanonisValue::I32(1).as_f32().is_err());
        assert!(NanonisValue::F32(1.0).as_u32().is_err());
        assert!(NanonisValue::U16(1).as_string().is_err());
    }

    #[test]
    fn as_array_methods() {
        let f32_arr = NanonisValue::ArrayF32(vec![1.0, 2.0]);
        assert_eq!(f32_arr.as_f32_array().unwrap(), &[1.0, 2.0]);
        assert!(f32_arr.as_f64_array().is_err());

        let str_arr = NanonisValue::ArrayString(vec!["a".into(), "b".into()]);
        assert_eq!(str_arr.as_string_array().unwrap(), &["a", "b"]);
        assert!(str_arr.as_f32_array().is_err());
    }
}
