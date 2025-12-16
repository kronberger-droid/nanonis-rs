use crate::error::NanonisError;
use serde::{Deserialize, Serialize};

// ==================== Core Protocol Value Type ====================

#[derive(Debug, Clone)]
pub enum NanonisValue {
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    F32(f32),
    F64(f64),
    String(String),
    ArrayU16(Vec<u16>),
    ArrayI16(Vec<i16>),
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
            _ => Err(NanonisError::Type(format!("Expected f32, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for f64 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::F64(v) => Ok(v),
            _ => Err(NanonisError::Type(format!("Expected f64, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for u16 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::U16(v) => Ok(v),
            _ => Err(NanonisError::Type(format!("Expected u16, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for u32 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::U32(v) => Ok(v),
            _ => Err(NanonisError::Type(format!("Expected u32, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for i16 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::I16(v) => Ok(v),
            _ => Err(NanonisError::Type(format!("Expected i16, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for i32 {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::I32(v) => Ok(v),
            _ => Err(NanonisError::Type(format!("Expected i32, got {value:?}"))),
        }
    }
}

impl TryFrom<NanonisValue> for Vec<f32> {
    type Error = NanonisError;

    fn try_from(value: NanonisValue) -> Result<Self, Self::Error> {
        match value {
            NanonisValue::ArrayF32(v) => Ok(v),
            _ => Err(NanonisError::Type(format!(
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
            _ => Err(NanonisError::Type(format!(
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
            _ => Err(NanonisError::Type(format!(
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
            _ => Err(NanonisError::Type(format!("Expected f32, got {self:?}"))),
        }
    }

    pub fn as_f64(&self) -> Result<f64, NanonisError> {
        match self {
            NanonisValue::F64(v) => Ok(*v),
            _ => Err(NanonisError::Type(format!("Expected f64, got {self:?}"))),
        }
    }

    pub fn as_u16(&self) -> Result<u16, NanonisError> {
        match self {
            NanonisValue::U16(v) => Ok(*v),
            _ => Err(NanonisError::Type(format!("Expected u16, got {self:?}"))),
        }
    }

    pub fn as_u32(&self) -> Result<u32, NanonisError> {
        match self {
            NanonisValue::U32(v) => Ok(*v),
            _ => Err(NanonisError::Type(format!("Expected u32, got {self:?}"))),
        }
    }

    pub fn as_i16(&self) -> Result<i16, NanonisError> {
        match self {
            NanonisValue::I16(v) => Ok(*v),
            _ => Err(NanonisError::Type(format!("Expected i16, got {self:?}"))),
        }
    }

    pub fn as_i32(&self) -> Result<i32, NanonisError> {
        match self {
            NanonisValue::I32(v) => Ok(*v),
            _ => Err(NanonisError::Type(format!("Expected i32, got {self:?}"))),
        }
    }

    pub fn as_string_array(&self) -> Result<&[String], NanonisError> {
        match self {
            NanonisValue::ArrayString(arr) => Ok(arr),
            _ => Err(NanonisError::Type(format!(
                "Expected string array, got {self:?}"
            ))),
        }
    }

    pub fn as_f32_array(&self) -> Result<&[f32], NanonisError> {
        match self {
            NanonisValue::ArrayF32(arr) => Ok(arr),
            _ => Err(NanonisError::Type(format!(
                "Expected f32 array, got {self:?}"
            ))),
        }
    }

    pub fn as_f64_array(&self) -> Result<&[f64], NanonisError> {
        match self {
            NanonisValue::ArrayF64(arr) => Ok(arr),
            _ => Err(NanonisError::Type(format!(
                "Expected f64 array, got {self:?}"
            ))),
        }
    }

    pub fn as_i32_array(&self) -> Result<&[i32], NanonisError> {
        match self {
            NanonisValue::ArrayI32(arr) => Ok(arr),
            _ => Err(NanonisError::Type(format!(
                "Expected i32 array, got {self:?}"
            ))),
        }
    }

    pub fn as_string(&self) -> Result<&str, NanonisError> {
        match self {
            NanonisValue::String(s) => Ok(s),
            _ => Err(NanonisError::Type(format!("Expected string, got {self:?}"))),
        }
    }

    pub fn as_f32_2d_array(&self) -> Result<&Vec<Vec<f32>>, NanonisError> {
        match self {
            NanonisValue::Array2DF32(arr) => Ok(arr),
            _ => Err(NanonisError::Type(format!(
                "Expected 2D f32 array, got {self:?}"
            ))),
        }
    }
}

// ==================== Index Types ====================

/// TCP channel index (0-23)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelIndex(pub u8);

impl ChannelIndex {
    pub fn new(index: u8) -> Result<Self, String> {
        if index <= 23 {
            Ok(Self(index))
        } else {
            Err(format!("Channel index {} out of range (0-23)", index))
        }
    }

    pub const fn new_unchecked(index: u8) -> Self {
        Self(index)
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

impl std::fmt::Display for ChannelIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for ChannelIndex {
    fn from(index: u8) -> Self {
        Self::new(index).unwrap_or_else(|_| {
            log::warn!(
                "Creating ChannelIndex from out-of-range value {}, clamping to 23",
                index
            );
            Self(23.min(index))
        })
    }
}

/// Signal index (0-127, but stored as usize for convenience)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignalIndex(pub u8);

impl SignalIndex {
    pub const fn new(index: u8) -> Self {
        Self(index)
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

impl From<SignalIndex> for u8 {
    fn from(signal: SignalIndex) -> Self {
        signal.0
    }
}

impl From<SignalIndex> for i32 {
    fn from(signal: SignalIndex) -> Self {
        signal.0 as i32
    }
}

impl From<usize> for SignalIndex {
    fn from(index: usize) -> Self {
        Self(index as u8)
    }
}

impl From<u8> for SignalIndex {
    fn from(index: u8) -> Self {
        Self(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OscilloscopeIndex(pub i32);

impl From<OscilloscopeIndex> for i32 {
    fn from(osci: OscilloscopeIndex) -> Self {
        osci.0
    }
}

impl From<i32> for OscilloscopeIndex {
    fn from(index: i32) -> Self {
        OscilloscopeIndex(index)
    }
}

impl From<usize> for OscilloscopeIndex {
    fn from(index: usize) -> Self {
        OscilloscopeIndex(index as i32)
    }
}

// ==================== Motor Control Types ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorDirection {
    XPlus = 0,
    XMinus = 1,
    YPlus = 2,
    YMinus = 3,
    ZPlus = 4,
    ZMinus = 5,
}

impl From<MotorDirection> for u32 {
    fn from(direction: MotorDirection) -> Self {
        direction as u32
    }
}

impl TryFrom<u32> for MotorDirection {
    type Error = NanonisError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MotorDirection::XPlus),
            1 => Ok(MotorDirection::XMinus),
            2 => Ok(MotorDirection::YPlus),
            3 => Ok(MotorDirection::YMinus),
            4 => Ok(MotorDirection::ZPlus),
            5 => Ok(MotorDirection::ZMinus),
            _ => Err(NanonisError::InvalidInput(format!(
                "Invalid motor direction: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorGroup {
    Group1 = 0,
    Group2 = 1,
    Group3 = 2,
    Group4 = 3,
    Group5 = 4,
    Group6 = 5,
}

impl From<MotorGroup> for u32 {
    fn from(group: MotorGroup) -> Self {
        group as u32
    }
}

impl TryFrom<u32> for MotorGroup {
    type Error = NanonisError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MotorGroup::Group1),
            1 => Ok(MotorGroup::Group2),
            2 => Ok(MotorGroup::Group3),
            3 => Ok(MotorGroup::Group4),
            4 => Ok(MotorGroup::Group5),
            5 => Ok(MotorGroup::Group6),
            _ => Err(NanonisError::InvalidInput(format!(
                "Invalid motor group: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StepCount(pub u16);

impl From<StepCount> for u16 {
    fn from(steps: StepCount) -> Self {
        steps.0
    }
}

impl From<u16> for StepCount {
    fn from(steps: u16) -> Self {
        StepCount(steps)
    }
}

impl From<u32> for StepCount {
    fn from(steps: u32) -> Self {
        StepCount(steps as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Frequency(pub f32);

impl Frequency {
    pub fn hz(value: f32) -> Self {
        Self(value)
    }
}

impl From<Frequency> for f32 {
    fn from(freq: Frequency) -> Self {
        freq.0
    }
}

impl From<f32> for Frequency {
    fn from(freq: f32) -> Self {
        Frequency(freq)
    }
}

impl From<f64> for Frequency {
    fn from(freq: f64) -> Self {
        Frequency(freq as f32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Amplitude(pub f32);

impl Amplitude {
    pub fn volts(value: f32) -> Self {
        Self(value)
    }
}

impl From<Amplitude> for f32 {
    fn from(amp: Amplitude) -> Self {
        amp.0
    }
}

impl From<f32> for Amplitude {
    fn from(amp: f32) -> Self {
        Amplitude(amp)
    }
}

impl From<f64> for Amplitude {
    fn from(amp: f64) -> Self {
        Amplitude(amp as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorAxis {
    All = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl From<MotorAxis> for u16 {
    fn from(axis: MotorAxis) -> Self {
        axis as u16
    }
}

impl From<u16> for MotorAxis {
    fn from(value: u16) -> Self {
        match value {
            0 => MotorAxis::All,
            1 => MotorAxis::X,
            2 => MotorAxis::Y,
            3 => MotorAxis::Z,
            _ => MotorAxis::All,
        }
    }
}

impl From<i32> for MotorAxis {
    fn from(value: i32) -> Self {
        MotorAxis::from(value as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn meters(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementMode {
    Relative = 0,
    Absolute = 1,
}

impl From<MovementMode> for u32 {
    fn from(mode: MovementMode) -> Self {
        mode as u32
    }
}

impl TryFrom<u32> for MovementMode {
    type Error = NanonisError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MovementMode::Relative),
            1 => Ok(MovementMode::Absolute),
            _ => Err(NanonisError::InvalidInput(format!(
                "Invalid movement mode: {}",
                value
            ))),
        }
    }
}

/// Motor movement specification
#[derive(Debug, Clone)]
pub struct MotorMovement {
    pub direction: MotorDirection,
    pub steps: StepCount,
    pub group: MotorGroup,
}

impl MotorMovement {
    pub fn new(direction: MotorDirection, steps: StepCount, group: MotorGroup) -> Self {
        Self {
            direction,
            steps,
            group,
        }
    }
}

/// 3D motor displacement for coordinated movement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotorDisplacement {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl MotorDisplacement {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    pub fn x_only(steps: i16) -> Self {
        Self {
            x: steps,
            y: 0,
            z: 0,
        }
    }

    pub fn y_only(steps: i16) -> Self {
        Self {
            x: 0,
            y: steps,
            z: 0,
        }
    }

    pub fn z_only(steps: i16) -> Self {
        Self {
            x: 0,
            y: 0,
            z: steps,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }

    pub fn to_motor_movements(&self) -> Vec<(MotorDirection, u16)> {
        let mut movements = Vec::new();

        // FIRST: ZMinus movements (away from surface) for safety
        if self.z < 0 {
            movements.push((MotorDirection::ZMinus, (-self.z) as u16));
        }

        // SECOND: X axis movements
        if self.x > 0 {
            movements.push((MotorDirection::XPlus, self.x as u16));
        } else if self.x < 0 {
            movements.push((MotorDirection::XMinus, (-self.x) as u16));
        }

        // THIRD: Y axis movements
        if self.y > 0 {
            movements.push((MotorDirection::YPlus, self.y as u16));
        } else if self.y < 0 {
            movements.push((MotorDirection::YMinus, (-self.y) as u16));
        }

        // LAST: ZPlus movements (toward surface)
        if self.z > 0 {
            movements.push((MotorDirection::ZPlus, self.z as u16));
        }

        movements
    }
}

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
            _ => Err(NanonisError::InvalidInput(format!(
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
            _ => Err(NanonisError::InvalidInput(format!(
                "Invalid scan direction: {}",
                value
            ))),
        }
    }
}

// ==================== Control Types ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZControllerHold {
    NoChange = 0,
    Hold = 1,
    Release = 2,
}

impl From<ZControllerHold> for u16 {
    fn from(hold: ZControllerHold) -> Self {
        hold as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseMode {
    Keep = 0,
    Relative = 1,
    Absolute = 2,
}

impl From<PulseMode> for u16 {
    fn from(mode: PulseMode) -> Self {
        mode as u16
    }
}

// ==================== Trigger and Timing Types ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerMode {
    Immediate = 0,
    Level = 1,
    Digital = 2,
}

impl From<TriggerMode> for u16 {
    fn from(mode: TriggerMode) -> Self {
        mode as u16
    }
}

impl From<u16> for TriggerMode {
    fn from(value: u16) -> Self {
        match value {
            0 => TriggerMode::Immediate,
            1 => TriggerMode::Level,
            2 => TriggerMode::Digital,
            _ => TriggerMode::Immediate,
        }
    }
}

impl From<i32> for TriggerMode {
    fn from(value: i32) -> Self {
        TriggerMode::from(value as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerSlope {
    Falling = 0,
    Rising = 1,
}

impl From<TriggerSlope> for u16 {
    fn from(slope: TriggerSlope) -> Self {
        slope as u16
    }
}

impl TryFrom<u16> for TriggerSlope {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TriggerSlope::Falling),
            1 => Ok(TriggerSlope::Rising),
            _ => Err(NanonisError::InvalidInput(format!(
                "Invalid trigger slope: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TriggerLevel(pub f64);

impl From<TriggerLevel> for f64 {
    fn from(level: TriggerLevel) -> Self {
        level.0
    }
}

impl From<f64> for TriggerLevel {
    fn from(level: f64) -> Self {
        TriggerLevel(level)
    }
}

impl From<f32> for TriggerLevel {
    fn from(level: f32) -> Self {
        TriggerLevel(level as f64)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampleCount(pub i32);

impl SampleCount {
    pub fn new(count: i32) -> Self {
        Self(count)
    }
}

impl From<SampleCount> for i32 {
    fn from(samples: SampleCount) -> Self {
        samples.0
    }
}

impl From<i32> for SampleCount {
    fn from(count: i32) -> Self {
        SampleCount(count)
    }
}

impl From<u32> for SampleCount {
    fn from(count: u32) -> Self {
        SampleCount(count as i32)
    }
}

impl From<usize> for SampleCount {
    fn from(count: usize) -> Self {
        SampleCount(count as i32)
    }
}

// ==================== Oscilloscope Types ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsciTriggerMode {
    Immediate = 0,
    Level = 1,
    Auto = 2,
}

impl From<OsciTriggerMode> for u16 {
    fn from(mode: OsciTriggerMode) -> Self {
        mode as u16
    }
}

impl TryFrom<u16> for OsciTriggerMode {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OsciTriggerMode::Immediate),
            1 => Ok(OsciTriggerMode::Level),
            2 => Ok(OsciTriggerMode::Auto),
            _ => Err(NanonisError::InvalidInput(format!(
                "Invalid oscilloscope trigger mode: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OversamplingIndex {
    Samples50 = 0,
    Samples20 = 1,
    Samples10 = 2,
    Samples5 = 3,
    Samples2 = 4,
    Samples1 = 5,
}

impl From<OversamplingIndex> for u16 {
    fn from(index: OversamplingIndex) -> Self {
        index as u16
    }
}

impl TryFrom<u16> for OversamplingIndex {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OversamplingIndex::Samples50),
            1 => Ok(OversamplingIndex::Samples20),
            2 => Ok(OversamplingIndex::Samples10),
            3 => Ok(OversamplingIndex::Samples5),
            4 => Ok(OversamplingIndex::Samples2),
            5 => Ok(OversamplingIndex::Samples1),
            _ => Err(NanonisError::InvalidInput(format!(
                "Invalid oversampling index: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimebaseIndex(pub i32);

impl From<TimebaseIndex> for i32 {
    fn from(index: TimebaseIndex) -> Self {
        index.0
    }
}

impl From<TimebaseIndex> for u16 {
    fn from(index: TimebaseIndex) -> Self {
        index.0 as u16
    }
}

impl From<i32> for TimebaseIndex {
    fn from(value: i32) -> Self {
        TimebaseIndex(value)
    }
}

impl From<u16> for TimebaseIndex {
    fn from(value: u16) -> Self {
        TimebaseIndex(value as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataToGet {
    Current,
    NextTrigger,
    Wait2Triggers,
}

#[derive(Debug, Clone, Copy)]
pub struct TriggerConfig {
    pub mode: OsciTriggerMode,
    pub slope: TriggerSlope,
    pub level: f64,
    pub hysteresis: f64,
}

impl TriggerConfig {
    pub fn new(mode: OsciTriggerMode, slope: TriggerSlope, level: f64, hysteresis: f64) -> Self {
        Self {
            mode,
            slope,
            level,
            hysteresis,
        }
    }

    pub fn immediate() -> Self {
        Self {
            mode: OsciTriggerMode::Immediate,
            slope: TriggerSlope::Rising,
            level: 0.0,
            hysteresis: 0.0,
        }
    }

    pub fn level_trigger(level: f64, slope: TriggerSlope) -> Self {
        Self {
            mode: OsciTriggerMode::Level,
            slope,
            level,
            hysteresis: 0.1,
        }
    }

    pub fn auto_trigger() -> Self {
        Self {
            mode: OsciTriggerMode::Auto,
            slope: TriggerSlope::Rising,
            level: 0.0,
            hysteresis: 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SignalStats {
    pub mean: f64,
    pub std_dev: f64,
    pub relative_std: f64,
    pub window_size: usize,
    pub stability_method: String,
}

#[derive(Debug, Clone)]
pub struct OsciData {
    pub t0: f64,
    pub dt: f64,
    pub size: i32,
    pub data: Vec<f64>,
    pub signal_stats: Option<SignalStats>,
    pub is_stable: bool,
    pub fallback_value: Option<f64>,
}

impl OsciData {
    pub fn new(t0: f64, dt: f64, size: i32, data: Vec<f64>) -> Self {
        Self {
            t0,
            dt,
            size,
            data,
            signal_stats: None,
            is_stable: true,
            fallback_value: None,
        }
    }

    pub fn new_with_stats(t0: f64, dt: f64, size: i32, data: Vec<f64>, stats: SignalStats) -> Self {
        Self {
            t0,
            dt,
            size,
            data,
            signal_stats: Some(stats),
            is_stable: true,
            fallback_value: None,
        }
    }

    pub fn new_stable(t0: f64, dt: f64, size: i32, data: Vec<f64>) -> Self {
        Self {
            t0,
            dt,
            size,
            data,
            signal_stats: None,
            is_stable: true,
            fallback_value: None,
        }
    }

    pub fn new_unstable_with_fallback(
        t0: f64,
        dt: f64,
        size: i32,
        data: Vec<f64>,
        fallback: f64,
    ) -> Self {
        Self {
            t0,
            dt,
            size,
            data,
            signal_stats: None,
            is_stable: false,
            fallback_value: Some(fallback),
        }
    }

    pub fn values(&self) -> &[f64] {
        &self.data
    }

    pub fn time_series(&self) -> Vec<(f64, f64)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, &value)| (self.t0 + i as f64 * self.dt, value))
            .collect()
    }

    pub fn stats(&self) -> Option<&SignalStats> {
        self.signal_stats.as_ref()
    }

    pub fn is_stable(&self) -> bool {
        self.signal_stats.is_some()
    }

    pub fn duration(&self) -> f64 {
        (self.size - 1) as f64 * self.dt
    }

    pub fn sample_rate(&self) -> f64 {
        if self.dt > 0.0 {
            1.0 / self.dt
        } else {
            0.0
        }
    }

    pub fn time_points(&self) -> Vec<f64> {
        (0..self.size)
            .map(|i| self.t0 + i as f64 * self.dt)
            .collect()
    }
}

// ==================== TCP Logger Types ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TCPLogStatus {
    Disconnected = 0,
    Idle = 1,
    Start = 2,
    Stop = 3,
    Running = 4,
    TCPConnect = 5,
    TCPDisconnect = 6,
    BufferOverflow = 7,
}

impl From<TCPLogStatus> for i32 {
    fn from(status: TCPLogStatus) -> Self {
        status as i32
    }
}

impl TryFrom<i32> for TCPLogStatus {
    type Error = NanonisError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TCPLogStatus::Disconnected),
            1 => Ok(TCPLogStatus::Idle),
            2 => Ok(TCPLogStatus::Start),
            3 => Ok(TCPLogStatus::Stop),
            4 => Ok(TCPLogStatus::Running),
            5 => Ok(TCPLogStatus::TCPConnect),
            6 => Ok(TCPLogStatus::TCPDisconnect),
            7 => Ok(TCPLogStatus::BufferOverflow),
            _ => Err(NanonisError::InvalidInput(format!(
                "Invalid TCP Logger status: {}",
                value
            ))),
        }
    }
}

impl std::fmt::Display for TCPLogStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            TCPLogStatus::Disconnected => "Disconnected",
            TCPLogStatus::Idle => "Idle",
            TCPLogStatus::Start => "Start",
            TCPLogStatus::Stop => "Stop",
            TCPLogStatus::Running => "Running",
            TCPLogStatus::TCPConnect => "TCP Connect",
            TCPLogStatus::TCPDisconnect => "TCP Disconnect",
            TCPLogStatus::BufferOverflow => "Buffer Overflow",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Debug, Clone)]
pub struct TCPLoggerData {
    pub num_channels: u32,
    pub oversampling: f32,
    pub counter: u64,
    pub state: TCPLogStatus,
    pub data: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct SignalFrame {
    pub counter: u64,
    pub data: Vec<f32>,
}
