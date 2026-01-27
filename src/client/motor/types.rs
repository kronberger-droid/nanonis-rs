use crate::error::NanonisError;

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
            _ => Err(NanonisError::Protocol(format!(
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
            _ => Err(NanonisError::Protocol(format!(
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
            _ => Err(NanonisError::Protocol(format!(
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
