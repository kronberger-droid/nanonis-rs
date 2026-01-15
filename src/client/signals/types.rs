use serde::{Deserialize, Serialize};

// ==================== Signal Types ====================

/// Nanonis signal index (0-127) for TCP protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignalIndex(pub u8);

impl SignalIndex {
    pub fn new(index: u8) -> Self {
        Self(index)
    }

    pub fn get(&self) -> u8 {
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

#[derive(Debug, Clone)]
pub struct SignalFrame {
    pub counter: u64,
    pub data: Vec<f32>,
}
