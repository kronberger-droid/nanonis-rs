pub mod client;
pub mod error;
pub mod protocol;
pub mod tcplogger_stream;
pub mod types;

// Re-export error types
pub use error::NanonisError;

// Re-export the main types from client
pub use client::{
    ConnectionConfig, NanonisClient, NanonisClientBuilder, TipShaperConfig, TipShaperProps,
    ZSpectroscopyResult,
};
pub use protocol::Protocol;
pub use tcplogger_stream::TCPLoggerStream;

// Re-export commonly used types
pub use types::{
    Amplitude,
    // Indices
    ChannelIndex,
    DataToGet,
    // Signal/Data
    Frequency,
    MotorAxis,
    // Motor/Position
    MotorDirection,
    MotorDisplacement,

    MotorGroup,
    MotorMovement,
    MovementMode,
    // Core protocol
    NanonisValue,

    OsciData,
    OsciTriggerMode,
    OscilloscopeIndex,

    OversamplingIndex,
    Position,
    Position3D,
    PulseMode,

    SampleCount,
    // Control/Scan
    ScanAction,
    ScanDirection,
    ScanFrame,
    SignalFrame,
    SignalIndex,
    SignalStats,
    StepCount,
    TCPLogStatus,
    TCPLoggerData,
    TimebaseIndex,
    TriggerConfig,

    TriggerLevel,
    // Oscilloscope
    TriggerMode,
    TriggerSlope,
    ZControllerHold,
};
