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

impl TryFrom<u16> for MotorAxis {
    type Error = NanonisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MotorAxis::All),
            1 => Ok(MotorAxis::X),
            2 => Ok(MotorAxis::Y),
            3 => Ok(MotorAxis::Z),
            _ => Err(NanonisError::Protocol(format!(
                "Invalid motor axis: {}",
                value
            ))),
        }
    }
}

impl TryFrom<i32> for MotorAxis {
    type Error = NanonisError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        MotorAxis::try_from(value as u16)
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

#[cfg(test)]
mod tests {
    use super::*;

    // ---- MotorDirection round-trip ----

    #[test]
    fn motor_direction_roundtrip() {
        for (dir, val) in [
            (MotorDirection::XPlus, 0), (MotorDirection::XMinus, 1),
            (MotorDirection::YPlus, 2), (MotorDirection::YMinus, 3),
            (MotorDirection::ZPlus, 4), (MotorDirection::ZMinus, 5),
        ] {
            assert_eq!(u32::from(dir), val);
            assert_eq!(MotorDirection::try_from(val).unwrap(), dir);
        }
    }

    #[test]
    fn motor_direction_invalid() {
        assert!(MotorDirection::try_from(6u32).is_err());
        assert!(MotorDirection::try_from(u32::MAX).is_err());
    }

    // ---- MotorGroup round-trip ----

    #[test]
    fn motor_group_roundtrip() {
        for val in 0..6u32 {
            let group = MotorGroup::try_from(val).unwrap();
            assert_eq!(u32::from(group), val);
        }
        assert!(MotorGroup::try_from(6u32).is_err());
    }

    // ---- MotorAxis ----

    #[test]
    fn motor_axis_roundtrip() {
        for (axis, val) in [
            (MotorAxis::All, 0u16), (MotorAxis::X, 1), (MotorAxis::Y, 2), (MotorAxis::Z, 3),
        ] {
            assert_eq!(u16::from(axis), val);
            assert_eq!(MotorAxis::try_from(val).unwrap(), axis);
        }
        assert!(MotorAxis::try_from(4u16).is_err());
    }

    #[test]
    fn motor_axis_from_i32() {
        assert_eq!(MotorAxis::try_from(0i32).unwrap(), MotorAxis::All);
        assert_eq!(MotorAxis::try_from(3i32).unwrap(), MotorAxis::Z);
    }

    // ---- MovementMode ----

    #[test]
    fn movement_mode_roundtrip() {
        assert_eq!(u32::from(MovementMode::Relative), 0);
        assert_eq!(u32::from(MovementMode::Absolute), 1);
        assert_eq!(MovementMode::try_from(0u32).unwrap(), MovementMode::Relative);
        assert_eq!(MovementMode::try_from(1u32).unwrap(), MovementMode::Absolute);
        assert!(MovementMode::try_from(2u32).is_err());
    }

    // ---- MotorDisplacement ----

    #[test]
    fn displacement_zero() {
        let d = MotorDisplacement::new(0, 0, 0);
        assert!(d.is_zero());
        assert!(d.to_motor_movements().is_empty());
    }

    #[test]
    fn displacement_single_axis() {
        let d = MotorDisplacement::x_only(5);
        assert!(!d.is_zero());
        let moves = d.to_motor_movements();
        assert_eq!(moves, vec![(MotorDirection::XPlus, 5)]);

        let d = MotorDisplacement::y_only(-3);
        assert_eq!(d.to_motor_movements(), vec![(MotorDirection::YMinus, 3)]);
    }

    #[test]
    fn displacement_z_minus_before_xy_z_plus_after() {
        // Safety-critical: Z-away must come first, Z-toward must come last
        let d = MotorDisplacement::new(2, -3, -5);
        let moves = d.to_motor_movements();
        assert_eq!(moves[0].0, MotorDirection::ZMinus); // first
        assert_eq!(moves[0].1, 5);

        let d = MotorDisplacement::new(2, -3, 5);
        let moves = d.to_motor_movements();
        assert_eq!(moves.last().unwrap().0, MotorDirection::ZPlus); // last
        assert_eq!(moves.last().unwrap().1, 5);
    }

    #[test]
    fn displacement_all_axes() {
        let d = MotorDisplacement::new(-1, 2, -3);
        let moves = d.to_motor_movements();
        // Order: ZMinus, X, Y (no ZPlus since z < 0)
        assert_eq!(moves.len(), 3);
        assert_eq!(moves[0], (MotorDirection::ZMinus, 3));
        assert_eq!(moves[1], (MotorDirection::XMinus, 1));
        assert_eq!(moves[2], (MotorDirection::YPlus, 2));
    }

    // ---- StepCount ----

    #[test]
    fn step_count_conversions() {
        let s = StepCount::from(100u16);
        assert_eq!(u16::from(s), 100);
        // u32 truncation
        let s = StepCount::from(65536u32); // wraps to 0
        assert_eq!(u16::from(s), 0);
    }
}
