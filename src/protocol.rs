use crate::error::NanonisError;
use crate::types::NanonisValue;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use log::debug;
use std::io::Read;

// Protocol constants
pub const COMMAND_SIZE: usize = 32;
pub const HEADER_SIZE: usize = 40;
pub const ERROR_INFO_SIZE: usize = 8;
pub const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024; // 100MB
const MAX_ARRAY_ELEMENTS: usize = 10_000_000; // 10M elements — sanity cap for network-derived lengths
pub const RESPONSE_FLAG: u16 = 1;
pub const ZERO_BUFFER: u16 = 0;

#[derive(Debug, Clone)]
struct MessageHeader {
    command: [u8; COMMAND_SIZE],
    body_size: u32,
    send_response: u16,
    _padding: u16,
}

impl MessageHeader {
    fn new(command: &str, body_size: u32) -> Self {
        let mut cmd_bytes = [0u8; COMMAND_SIZE];
        let cmd_str = command.as_bytes();
        let len = cmd_str.len().min(COMMAND_SIZE);
        cmd_bytes[..len].copy_from_slice(&cmd_str[..len]);

        Self {
            command: cmd_bytes,
            body_size,
            send_response: RESPONSE_FLAG, // Always request response for error info
            _padding: ZERO_BUFFER,
        }
    }

    // Safe serialization without unsafe code
    fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut buf = [0u8; HEADER_SIZE];
        buf[0..32].copy_from_slice(&self.command);
        buf[32..36].copy_from_slice(&self.body_size.to_be_bytes());
        buf[36..38].copy_from_slice(&self.send_response.to_be_bytes());
        buf[38..40].copy_from_slice(&self._padding.to_be_bytes());
        buf
    }
}

/// Low-level protocol handling
pub struct Protocol;

impl Protocol {
    /// Parse error information from the end of a response body using safe slice operations
    pub fn parse_error_info(body: &[u8], data_end_cursor: usize) -> Result<(), NanonisError> {
        // Get error section safely
        let error_section = match body.get(data_end_cursor..) {
            Some(section) if section.len() >= ERROR_INFO_SIZE => section,
            _ => return Ok(()), // No error info available
        };

        // Use safe slice splitting instead of manual indexing
        let (status_bytes, rest) = error_section.split_at(4);
        let (size_bytes, message_bytes) = rest.split_at(4);

        let error_status = i32::from_be_bytes(
            status_bytes
                .try_into()
                .map_err(|_| NanonisError::Protocol("Invalid error status format".into()))?,
        );

        let error_desc_size = i32::from_be_bytes(
            size_bytes
                .try_into()
                .map_err(|_| NanonisError::Protocol("Invalid error size format".into()))?,
        ) as usize;

        if error_desc_size > 0 {
            // Safe message extraction with bounds checking
            let message_slice = message_bytes
                .get(..error_desc_size)
                .ok_or_else(|| NanonisError::Protocol("Error message truncated".into()))?;

            // Use from_utf8 for better error handling
            let error_msg = std::str::from_utf8(message_slice)
                .map_err(|_| NanonisError::Protocol("Invalid UTF-8 in error message".into()))?;

            let trimmed_msg = error_msg.trim();
            if !trimmed_msg.is_empty() {
                return Err(NanonisError::Server {
                    code: error_status,
                    message: trimmed_msg.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Helper for reading exact byte counts with better error messages
    pub fn read_exact_bytes<const N: usize>(
        reader: &mut dyn Read,
    ) -> Result<[u8; N], NanonisError> {
        debug!("Attempting to read exactly {} bytes", N);
        let mut buf = [0u8; N];

        match reader.read_exact(&mut buf) {
            Ok(()) => {
                debug!(
                    "Successfully read {} bytes: {:02x?}",
                    N,
                    if N <= 20 { &buf[..] } else { &buf[..20] }
                );
                Ok(buf)
            }
            Err(e) => {
                debug!("Failed to read {} bytes: {} (kind: {:?})", N, e, e.kind());
                Err(NanonisError::from_io(
                    e,
                    format!("Failed to read {N} bytes from Nanonis"),
                ))
            }
        }
    }

    /// Helper for reading variable-length data with size validation
    pub fn read_variable_bytes(
        reader: &mut dyn Read,
        size: usize,
    ) -> Result<Vec<u8>, NanonisError> {
        debug!("Attempting to read {} variable bytes", size);

        // Reasonable size limit to prevent memory attacks
        if size > MAX_RESPONSE_SIZE {
            debug!("Size {} exceeds maximum {}", size, MAX_RESPONSE_SIZE);
            return Err(NanonisError::Protocol(format!(
                "Response size {} exceeds maximum {}",
                size, MAX_RESPONSE_SIZE
            )));
        }

        let mut body = vec![0u8; size];
        match reader.read_exact(&mut body) {
            Ok(()) => {
                debug!(
                    "Successfully read {} variable bytes: {:02x?}",
                    size,
                    if size <= 50 { &body[..] } else { &body[..50] }
                );
                Ok(body)
            }
            Err(e) => {
                debug!(
                    "Failed to read {} variable bytes: {} (kind: {:?})",
                    size,
                    e,
                    e.kind()
                );
                // Note: do NOT call read_to_end() here for diagnostics --
                // on a live TCP connection it blocks until the peer closes
                // the socket, which may be never.
                Err(NanonisError::from_io(
                    e,
                    format!("Failed to read {size} byte response body"),
                ))
            }
        }
    }

    /// Parse response with error checking
    pub fn parse_response_with_error_check(
        response: &[u8],
        response_types: &[&str],
    ) -> Result<Vec<NanonisValue>, NanonisError> {
        let (values, cursor_position) = Self::parse_response(response, response_types)?;

        // Check for errors in the section following the parsed data
        Self::parse_error_info(response, cursor_position)?;

        Ok(values)
    }

    /// Serialize a value according to its type specification
    pub fn serialize_value(
        value: &NanonisValue,
        body_type: &str,
        buffer: &mut Vec<u8>,
    ) -> Result<(), NanonisError> {
        match (value, body_type) {
            (NanonisValue::U8(v), "b") => buffer.push(*v),
            (NanonisValue::U16(v), "H") => buffer.write_u16::<BigEndian>(*v)?,
            (NanonisValue::I16(v), "h") => buffer.write_i16::<BigEndian>(*v)?,
            (NanonisValue::U32(v), "I") => buffer.write_u32::<BigEndian>(*v)?,
            (NanonisValue::I32(v), "i") => buffer.write_i32::<BigEndian>(*v)?,
            (NanonisValue::F32(v), "f") => buffer.write_f32::<BigEndian>(*v)?,
            (NanonisValue::F64(v), "d") => buffer.write_f64::<BigEndian>(*v)?,

            (NanonisValue::String(s), t) if t.contains("*c") => {
                let bytes = s.as_bytes();
                if t.starts_with("+") {
                    buffer.write_u32::<BigEndian>(bytes.len() as u32)?;
                }
                buffer.extend_from_slice(bytes);
            }

            (NanonisValue::ArrayString(arr), "+*c") => {
                // Write total byte size (sum of all string lengths + size prefixes)
                let total_size: usize = arr.iter().map(|s| 4 + s.len()).sum();
                buffer.write_u32::<BigEndian>(total_size as u32)?;
                // Write number of strings
                buffer.write_u32::<BigEndian>(arr.len() as u32)?;
                // Write each string with its length prepended
                for s in arr {
                    let bytes = s.as_bytes();
                    buffer.write_u32::<BigEndian>(bytes.len() as u32)?;
                    buffer.extend_from_slice(bytes);
                }
            }

            (NanonisValue::ArrayString(arr), "*+c") => {
                // Don't write count - it comes from previous variable
                for s in arr {
                    let bytes = s.as_bytes();
                    buffer.write_u32::<BigEndian>(bytes.len() as u32)?;
                    buffer.extend_from_slice(bytes);
                }
            }

            (NanonisValue::ArrayI32(arr), t) if t.contains("*i") => {
                if t.starts_with("+") {
                    buffer.write_u32::<BigEndian>(arr.len() as u32)?;
                }
                for &val in arr {
                    buffer.write_i32::<BigEndian>(val)?;
                }
            }

            (NanonisValue::ArrayU32(arr), t) if t.contains("*I") => {
                if t.starts_with("+") {
                    buffer.write_u32::<BigEndian>(arr.len() as u32)?;
                }
                for &val in arr {
                    buffer.write_u32::<BigEndian>(val)?;
                }
            }

            (NanonisValue::ArrayF32(arr), t) if t.contains("*f") => {
                if t.starts_with("+") {
                    buffer.write_u32::<BigEndian>(arr.len() as u32)?;
                }
                for &val in arr {
                    buffer.write_f32::<BigEndian>(val)?;
                }
            }

            (NanonisValue::ArrayF64(arr), t) if t.contains("*d") => {
                if t.starts_with("+") {
                    buffer.write_u32::<BigEndian>(arr.len() as u32)?;
                }
                for &val in arr {
                    buffer.write_f64::<BigEndian>(val)?;
                }
            }

            (NanonisValue::ArrayU8(arr), t) if t.contains("*b") => {
                if t.starts_with("+") {
                    buffer.write_i32::<BigEndian>(arr.len() as i32)?;
                }
                buffer.extend_from_slice(arr);
            }

            _ => {
                return Err(NanonisError::Protocol(format!(
                    "Unsupported type combination: {value:?} with {body_type}"
                )));
            }
        }
        Ok(())
    }

    /// Validate that a network-derived array length is within sane bounds.
    fn validate_array_len(len: usize, type_desc: &str) -> Result<(), NanonisError> {
        if len > MAX_ARRAY_ELEMENTS {
            return Err(NanonisError::Protocol(format!(
                "Array length {len} exceeds maximum ({MAX_ARRAY_ELEMENTS}) for type '{type_desc}'"
            )));
        }
        Ok(())
    }

    /// Parse response data according to type specifications.
    /// Returns the parsed values and the cursor position after parsing (for error section location).
    pub fn parse_response(
        response: &[u8],
        response_types: &[&str],
    ) -> Result<(Vec<NanonisValue>, usize), NanonisError> {
        let mut cursor = std::io::Cursor::new(response);
        let mut result = Vec::with_capacity(response_types.len());

        for &response_type in response_types {
            let value = match response_type {
                "H" => NanonisValue::U16(cursor.read_u16::<BigEndian>()?),
                "h" => NanonisValue::I16(cursor.read_i16::<BigEndian>()?),
                "I" => NanonisValue::U32(cursor.read_u32::<BigEndian>()?),
                "i" => NanonisValue::I32(cursor.read_i32::<BigEndian>()?),
                "f" => NanonisValue::F32(cursor.read_f32::<BigEndian>()?),
                "d" => NanonisValue::F64(cursor.read_f64::<BigEndian>()?),

                t if t.contains("*f") => {
                    let len = if t.starts_with("+") {
                        cursor.read_u32::<BigEndian>()? as usize
                    } else if let Some(prev_val) = result.last() {
                        match prev_val {
                            NanonisValue::U32(len) => *len as usize,
                            NanonisValue::I32(len) => *len as usize,
                            _ => {
                                return Err(NanonisError::Protocol(
                                    "Array length not found".to_string(),
                                ));
                            }
                        }
                    } else {
                        return Err(NanonisError::Protocol(
                            "Array length not specified".to_string(),
                        ));
                    };

                    Self::validate_array_len(len, t)?;
                    let mut arr = Vec::with_capacity(len);
                    for _ in 0..len {
                        arr.push(cursor.read_f32::<BigEndian>()?);
                    }
                    NanonisValue::ArrayF32(arr)
                }

                t if t.contains("*d") => {
                    let len = if t.starts_with("+") {
                        cursor.read_u32::<BigEndian>()? as usize
                    } else if let Some(prev_val) = result.last() {
                        match prev_val {
                            NanonisValue::U32(len) => *len as usize,
                            NanonisValue::I32(len) => *len as usize,
                            _ => {
                                return Err(NanonisError::Protocol(
                                    "Array length not found".to_string(),
                                ));
                            }
                        }
                    } else {
                        return Err(NanonisError::Protocol(
                            "Array length not specified".to_string(),
                        ));
                    };

                    Self::validate_array_len(len, t)?;
                    let mut arr = Vec::with_capacity(len);
                    for _ in 0..len {
                        arr.push(cursor.read_f64::<BigEndian>()?);
                    }
                    NanonisValue::ArrayF64(arr)
                }

                t if t.contains("*i") => {
                    let len = if t.starts_with("+") {
                        cursor.read_u32::<BigEndian>()? as usize
                    } else if let Some(prev_val) = result.last() {
                        match prev_val {
                            NanonisValue::U32(len) => *len as usize,
                            NanonisValue::I32(len) => *len as usize,
                            _ => {
                                return Err(NanonisError::Protocol(
                                    "Array length not found".to_string(),
                                ));
                            }
                        }
                    } else {
                        return Err(NanonisError::Protocol(
                            "Array length not specified".to_string(),
                        ));
                    };

                    Self::validate_array_len(len, t)?;
                    let mut arr = Vec::with_capacity(len);
                    for _ in 0..len {
                        arr.push(cursor.read_i32::<BigEndian>()?);
                    }
                    NanonisValue::ArrayI32(arr)
                }

                t if t.contains("*I") => {
                    let len = if t.starts_with("+") {
                        cursor.read_u32::<BigEndian>()? as usize
                    } else if let Some(prev_val) = result.last() {
                        match prev_val {
                            NanonisValue::U32(len) => *len as usize,
                            NanonisValue::I32(len) => *len as usize,
                            _ => {
                                return Err(NanonisError::Protocol(
                                    "Array length not found".to_string(),
                                ));
                            }
                        }
                    } else {
                        return Err(NanonisError::Protocol(
                            "Array length not specified".to_string(),
                        ));
                    };

                    Self::validate_array_len(len, t)?;
                    let mut arr = Vec::with_capacity(len);
                    for _ in 0..len {
                        arr.push(cursor.read_u32::<BigEndian>()?);
                    }
                    NanonisValue::ArrayU32(arr)
                }

                // Handle string arrays with prepended length
                "+*c" => {
                    // First read total byte size (we don't use this, but it's in the protocol)
                    let _total_size = cursor.read_u32::<BigEndian>()?;
                    // Then read number of strings
                    let num_strings = cursor.read_u32::<BigEndian>()? as usize;
                    Self::validate_array_len(num_strings, "+*c")?;
                    let mut strings = Vec::with_capacity(num_strings);

                    for _ in 0..num_strings {
                        let string_len = cursor.read_u32::<BigEndian>()? as usize;
                        let mut string_bytes = vec![0u8; string_len];
                        cursor.read_exact(&mut string_bytes)?;
                        let string = String::from_utf8_lossy(&string_bytes).to_string();
                        strings.push(string);
                    }

                    NanonisValue::ArrayString(strings)
                }

                // Handle string arrays with count from previous variable
                "*+c" => {
                    // Get string count from previous variable (should be an integer)
                    let num_strings = match result.last() {
                        Some(NanonisValue::I32(count)) => *count as usize,
                        Some(NanonisValue::U32(count)) => *count as usize,
                        _ => {
                            return Err(NanonisError::Protocol(
                                "String count not found for *+c type".to_string(),
                            ));
                        }
                    };

                    Self::validate_array_len(num_strings, "*+c")?;
                    let mut strings = Vec::with_capacity(num_strings);
                    for _ in 0..num_strings {
                        let string_len = cursor.read_u32::<BigEndian>()? as usize;
                        let mut string_bytes = vec![0u8; string_len];
                        cursor.read_exact(&mut string_bytes)?;
                        let string = String::from_utf8_lossy(&string_bytes).to_string();
                        strings.push(string);
                    }

                    NanonisValue::ArrayString(strings)
                }

                // Handle dynamic strings (*-c) where length comes from previous variable
                "*-c" => {
                    // Get string length from previous variable (should be an integer)
                    let string_length = match result.last() {
                        Some(NanonisValue::I32(len)) => *len as usize,
                        Some(NanonisValue::U32(len)) => *len as usize,
                        _ => {
                            return Err(NanonisError::Protocol(
                                "String length not found for *-c type".to_string(),
                            ));
                        }
                    };

                    // Read string bytes
                    let mut string_bytes = vec![0u8; string_length];
                    cursor.read_exact(&mut string_bytes)?;
                    let string = String::from_utf8_lossy(&string_bytes).to_string();

                    NanonisValue::String(string)
                }

                // Handle integer arrays with count from previous variable (*+i)
                "*+i" => {
                    // Get array count from previous variable (should be an integer)
                    let array_count = match result.last() {
                        Some(NanonisValue::I32(count)) => *count as usize,
                        Some(NanonisValue::U32(count)) => *count as usize,
                        _ => {
                            return Err(NanonisError::Protocol(
                                "Array count not found for *+i type".to_string(),
                            ));
                        }
                    };

                    // Read the integer array
                    Self::validate_array_len(array_count, "*+i")?;
                    let mut arr = Vec::with_capacity(array_count);
                    for _ in 0..array_count {
                        arr.push(cursor.read_i32::<BigEndian>()?);
                    }

                    NanonisValue::ArrayI32(arr)
                }

                "2f" => {
                    // 2D float array - dimensions should be in the two preceding i32 values
                    if result.len() < 2 {
                        return Err(NanonisError::Protocol(
                            "2D array dimensions not found".to_string(),
                        ));
                    }

                    let rows = match result[result.len() - 2] {
                        NanonisValue::I32(r) => r as usize,
                        _ => {
                            return Err(NanonisError::Protocol(
                                "Invalid row count for 2D array".to_string(),
                            ));
                        }
                    };

                    let cols = match result[result.len() - 1] {
                        NanonisValue::I32(c) => c as usize,
                        _ => {
                            return Err(NanonisError::Protocol(
                                "Invalid column count for 2D array".to_string(),
                            ));
                        }
                    };

                    // Read the flat array data
                    let mut data_2d = Vec::with_capacity(rows);

                    for _ in 0..rows {
                        let mut row_data = Vec::with_capacity(cols);
                        for _ in 0..cols {
                            row_data.push(cursor.read_f32::<BigEndian>()?);
                        }
                        data_2d.push(row_data);
                    }

                    NanonisValue::Array2DF32(data_2d)
                }

                _ => {
                    return Err(NanonisError::Protocol(format!(
                        "Unsupported response type: {response_type}"
                    )));
                }
            };

            result.push(value);
        }

        Ok((result, cursor.position() as usize))
    }

    /// Create command header with proper padding using safe serialization
    pub fn create_command_header(command: &str, body_size: u32) -> Vec<u8> {
        let header = MessageHeader::new(command, body_size);
        header.to_bytes().to_vec()
    }

    /// Validate command response header
    pub fn validate_response_header(
        header: &[u8; HEADER_SIZE],
        expected_command: &str,
    ) -> Result<u32, NanonisError> {
        // Extract body size from header (bytes 32-36)
        let response_body_size =
            u32::from_be_bytes([header[32], header[33], header[34], header[35]]);

        // Verify command matches (bytes 0-32 of header)
        let received_command = String::from_utf8_lossy(&header[0..COMMAND_SIZE])
            .trim_end_matches('\0')
            .to_string();

        if received_command == expected_command {
            Ok(response_body_size)
        } else {
            Err(NanonisError::Protocol(format!(
                "Command mismatch: expected {expected_command}, got {received_command}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{BigEndian, WriteBytesExt};

    // ---- Header tests ----

    #[test]
    fn create_command_header_basic() {
        let header = Protocol::create_command_header("Bias.Set", 4);
        assert_eq!(header.len(), HEADER_SIZE);
        assert_eq!(&header[..8], b"Bias.Set");
        assert!(header[8..32].iter().all(|&b| b == 0));
        assert_eq!(
            u32::from_be_bytes([header[32], header[33], header[34], header[35]]),
            4
        );
        assert_eq!(u16::from_be_bytes([header[36], header[37]]), RESPONSE_FLAG);
    }

    #[test]
    fn create_command_header_truncates_long_command() {
        let long_cmd = "A".repeat(40);
        let header = Protocol::create_command_header(&long_cmd, 0);
        assert!(header[..32].iter().all(|&b| b == b'A'));
    }

    #[test]
    fn validate_response_header_matching() {
        let header = Protocol::create_command_header("Bias.Get", 42);
        let header_arr: [u8; HEADER_SIZE] = header.try_into().unwrap();
        assert_eq!(
            Protocol::validate_response_header(&header_arr, "Bias.Get").unwrap(),
            42
        );
    }

    #[test]
    fn validate_response_header_mismatch() {
        let header = Protocol::create_command_header("Bias.Get", 42);
        let header_arr: [u8; HEADER_SIZE] = header.try_into().unwrap();
        assert!(
            Protocol::validate_response_header(&header_arr, "Scan.Start")
                .unwrap_err()
                .is_protocol()
        );
    }

    #[test]
    fn validate_response_header_command_ending_in_zero_digit() {
        // Regression: trim_end_matches('0') previously stripped digit '0'
        let header = Protocol::create_command_header("Foo.Get0", 10);
        let header_arr: [u8; HEADER_SIZE] = header.try_into().unwrap();
        assert_eq!(
            Protocol::validate_response_header(&header_arr, "Foo.Get0").unwrap(),
            10
        );
    }

    // ---- Serialization ----

    #[test]
    fn serialize_primitives() {
        let cases: Vec<(NanonisValue, &str, Vec<u8>)> = vec![
            (NanonisValue::U16(0x1234), "H", vec![0x12, 0x34]),
            (NanonisValue::I32(-1), "i", vec![0xFF, 0xFF, 0xFF, 0xFF]),
            (NanonisValue::F32(1.0), "f", 1.0f32.to_be_bytes().to_vec()),
            (NanonisValue::F64(2.5), "d", 2.5f64.to_be_bytes().to_vec()),
        ];
        for (val, td, expected) in cases {
            let mut buf = Vec::new();
            Protocol::serialize_value(&val, td, &mut buf).unwrap();
            assert_eq!(buf, expected, "Failed for type {td}");
        }
    }

    #[test]
    fn serialize_string_with_prepended_length() {
        let mut buf = Vec::new();
        Protocol::serialize_value(&NanonisValue::String("hi".into()), "+*c", &mut buf).unwrap();
        assert_eq!(buf, [0, 0, 0, 2, b'h', b'i']);
    }

    #[test]
    fn serialize_type_mismatch() {
        let mut buf = Vec::new();
        assert!(
            Protocol::serialize_value(&NanonisValue::F32(1.0), "i", &mut buf)
                .unwrap_err()
                .is_protocol()
        );
    }

    // ---- Parse response ----

    #[test]
    fn parse_primitives() {
        let mut data = Vec::new();
        data.write_u16::<BigEndian>(42).unwrap();
        data.write_i32::<BigEndian>(-7).unwrap();
        data.write_f32::<BigEndian>(1.25).unwrap();
        data.write_f64::<BigEndian>(2.25).unwrap();

        let (vals, _) = Protocol::parse_response(&data, &["H", "i", "f", "d"]).unwrap();
        assert_eq!(vals[0].as_u16().unwrap(), 42);
        assert_eq!(vals[1].as_i32().unwrap(), -7);
        assert!((vals[2].as_f32().unwrap() - 1.25).abs() < 1e-5);
        assert!((vals[3].as_f64().unwrap() - 2.25).abs() < 1e-10);
    }

    #[test]
    fn parse_f32_array_prepended() {
        let mut data = Vec::new();
        data.write_u32::<BigEndian>(3).unwrap();
        for v in &[1.0f32, 2.0, 3.0] {
            data.write_f32::<BigEndian>(*v).unwrap();
        }
        let (vals, _) = Protocol::parse_response(&data, &["+*f"]).unwrap();
        assert_eq!(vals[0].as_f32_array().unwrap(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn parse_f32_array_implicit_length() {
        let mut data = Vec::new();
        data.write_u32::<BigEndian>(2).unwrap();
        data.write_f32::<BigEndian>(10.0).unwrap();
        data.write_f32::<BigEndian>(20.0).unwrap();
        let (vals, _) = Protocol::parse_response(&data, &["I", "*f"]).unwrap();
        assert_eq!(vals[1].as_f32_array().unwrap(), &[10.0, 20.0]);
    }

    #[test]
    fn parse_string_array_prepended() {
        let mut data = Vec::new();
        let total = (4 + 5 + 4 + 5) as u32; // two 5-byte strings
        data.write_u32::<BigEndian>(total).unwrap();
        data.write_u32::<BigEndian>(2).unwrap();
        data.write_u32::<BigEndian>(5).unwrap();
        data.extend_from_slice(b"hello");
        data.write_u32::<BigEndian>(5).unwrap();
        data.extend_from_slice(b"world");
        let (vals, _) = Protocol::parse_response(&data, &["+*c"]).unwrap();
        assert_eq!(vals[0].as_string_array().unwrap(), &["hello", "world"]);
    }

    #[test]
    fn parse_dynamic_string() {
        let mut data = Vec::new();
        data.write_i32::<BigEndian>(4).unwrap();
        data.extend_from_slice(b"test");
        let (vals, _) = Protocol::parse_response(&data, &["i", "*-c"]).unwrap();
        assert_eq!(vals[1].as_string().unwrap(), "test");
    }

    #[test]
    fn parse_2d_f32_array() {
        let mut data = Vec::new();
        data.write_i32::<BigEndian>(2).unwrap();
        data.write_i32::<BigEndian>(3).unwrap();
        for v in &[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0] {
            data.write_f32::<BigEndian>(*v).unwrap();
        }
        let (vals, _) = Protocol::parse_response(&data, &["i", "i", "2f"]).unwrap();
        assert_eq!(
            vals[2].as_f32_2d_array().unwrap(),
            &[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]
        );
    }

    #[test]
    fn parse_empty_response() {
        let (vals, pos) = Protocol::parse_response(&[], &[]).unwrap();
        assert!(vals.is_empty());
        assert_eq!(pos, 0);
    }

    #[test]
    fn parse_truncated_data() {
        assert!(
            Protocol::parse_response(&[0; 2], &["f"])
                .unwrap_err()
                .is_io()
        );
    }

    #[test]
    fn parse_unsupported_type() {
        assert!(
            Protocol::parse_response(&[0; 4], &["UNKNOWN"])
                .unwrap_err()
                .is_protocol()
        );
    }

    // ---- Round-trip ----

    #[test]
    fn roundtrip_all_primitives() {
        let cases: Vec<(NanonisValue, &str)> = vec![
            (NanonisValue::U16(1000), "H"),
            (NanonisValue::I16(-500), "h"),
            (NanonisValue::U32(123456), "I"),
            (NanonisValue::I32(-42), "i"),
            (NanonisValue::F32(1.25), "f"),
            (NanonisValue::F64(2.25281828), "d"),
        ];
        for (val, td) in &cases {
            let mut buf = Vec::new();
            Protocol::serialize_value(val, td, &mut buf).unwrap();
            let (parsed, _) = Protocol::parse_response(&buf, &[td]).unwrap();
            match (val, &parsed[0]) {
                (NanonisValue::U16(a), NanonisValue::U16(b)) => assert_eq!(a, b),
                (NanonisValue::I16(a), NanonisValue::I16(b)) => assert_eq!(a, b),
                (NanonisValue::U32(a), NanonisValue::U32(b)) => assert_eq!(a, b),
                (NanonisValue::I32(a), NanonisValue::I32(b)) => assert_eq!(a, b),
                (NanonisValue::F32(a), NanonisValue::F32(b)) => assert!((a - b).abs() < 1e-7),
                (NanonisValue::F64(a), NanonisValue::F64(b)) => assert!((a - b).abs() < 1e-15),
                _ => panic!("Type mismatch"),
            }
        }
    }

    #[test]
    fn roundtrip_arrays() {
        // f32 array
        let arr = vec![1.0f32, -2.5, 0.0, f32::MAX, f32::MIN_POSITIVE];
        let mut buf = Vec::new();
        Protocol::serialize_value(&NanonisValue::ArrayF32(arr.clone()), "+*f", &mut buf).unwrap();
        let (p, _) = Protocol::parse_response(&buf, &["+*f"]).unwrap();
        assert_eq!(p[0].as_f32_array().unwrap(), &arr);

        // string array
        let strings = vec!["hello".into(), "".into(), "world!".into()];
        buf.clear();
        Protocol::serialize_value(&NanonisValue::ArrayString(strings.clone()), "+*c", &mut buf)
            .unwrap();
        let (p, _) = Protocol::parse_response(&buf, &["+*c"]).unwrap();
        assert_eq!(p[0].as_string_array().unwrap(), &strings);

        // empty array
        buf.clear();
        Protocol::serialize_value(&NanonisValue::ArrayF32(vec![]), "+*f", &mut buf).unwrap();
        let (p, _) = Protocol::parse_response(&buf, &["+*f"]).unwrap();
        assert!(p[0].as_f32_array().unwrap().is_empty());
    }

    // ---- Error info ----

    #[test]
    fn error_info_no_error() {
        let mut body = Vec::new();
        body.write_i32::<BigEndian>(0).unwrap();
        body.write_i32::<BigEndian>(0).unwrap();
        Protocol::parse_error_info(&body, 0).unwrap();
    }

    #[test]
    fn error_info_with_message() {
        let mut body = Vec::new();
        body.write_i32::<BigEndian>(-1).unwrap();
        let msg = "Something failed";
        body.write_i32::<BigEndian>(msg.len() as i32).unwrap();
        body.extend_from_slice(msg.as_bytes());
        match Protocol::parse_error_info(&body, 0).unwrap_err() {
            NanonisError::Server { code, message } => {
                assert_eq!(code, -1);
                assert_eq!(message, "Something failed");
            }
            e => panic!("Expected Server error, got {e:?}"),
        }
    }

    #[test]
    fn error_info_too_short_is_ok() {
        Protocol::parse_error_info(&[0; 4], 0).unwrap();
    }

    #[test]
    fn error_info_with_data_offset() {
        let mut body = vec![0xAA; 10];
        body.write_i32::<BigEndian>(-5).unwrap();
        let msg = "error";
        body.write_i32::<BigEndian>(msg.len() as i32).unwrap();
        body.extend_from_slice(msg.as_bytes());
        assert_eq!(
            Protocol::parse_error_info(&body, 10)
                .unwrap_err()
                .error_code(),
            Some(-5)
        );
    }

    // ---- Array length cap ----

    #[test]
    fn array_len_validation() {
        Protocol::validate_array_len(MAX_ARRAY_ELEMENTS, "*f").unwrap();
        assert!(
            Protocol::validate_array_len(MAX_ARRAY_ELEMENTS + 1, "*f")
                .unwrap_err()
                .is_protocol()
        );
    }

    // ---- Read helpers ----

    #[test]
    fn read_variable_bytes_rejects_oversized() {
        let mut r = std::io::Cursor::new(vec![0u8; 10]);
        assert!(
            Protocol::read_variable_bytes(&mut r, MAX_RESPONSE_SIZE + 1)
                .unwrap_err()
                .is_protocol()
        );
    }

    #[test]
    fn read_exact_bytes_success_and_failure() {
        let mut r = std::io::Cursor::new([1u8, 2, 3, 4]);
        assert_eq!(
            Protocol::read_exact_bytes::<4>(&mut r).unwrap(),
            [1, 2, 3, 4]
        );

        let mut r = std::io::Cursor::new([1u8, 2]);
        assert!(Protocol::read_exact_bytes::<4>(&mut r).unwrap_err().is_io());
    }

    // ---- parse_response_with_error_check ----

    #[test]
    fn with_error_check_clean() {
        let mut data = Vec::new();
        data.write_f32::<BigEndian>(1.5).unwrap();
        data.write_i32::<BigEndian>(0).unwrap();
        data.write_i32::<BigEndian>(0).unwrap();
        let vals = Protocol::parse_response_with_error_check(&data, &["f"]).unwrap();
        assert!((vals[0].as_f32().unwrap() - 1.5).abs() < 1e-7);
    }

    #[test]
    fn with_error_check_server_error() {
        let mut data = Vec::new();
        data.write_f32::<BigEndian>(1.5).unwrap();
        data.write_i32::<BigEndian>(42).unwrap();
        let msg = "bad param";
        data.write_i32::<BigEndian>(msg.len() as i32).unwrap();
        data.extend_from_slice(msg.as_bytes());
        let err = Protocol::parse_response_with_error_check(&data, &["f"]).unwrap_err();
        assert!(err.is_server_error());
        assert_eq!(err.error_code(), Some(42));
    }
}
