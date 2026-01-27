use super::NanonisClient;
use crate::error::NanonisError;
use crate::types::NanonisValue;

/// Point mark information.
#[derive(Debug, Clone)]
pub struct PointMark {
    /// X coordinate (meters)
    pub x_m: f32,
    /// Y coordinate (meters)
    pub y_m: f32,
    /// Text label
    pub text: String,
    /// RGB color
    pub color: u32,
    /// Visible flag
    pub visible: bool,
}

/// Line mark information.
#[derive(Debug, Clone)]
pub struct LineMark {
    /// Start X coordinate (meters)
    pub start_x_m: f32,
    /// Start Y coordinate (meters)
    pub start_y_m: f32,
    /// End X coordinate (meters)
    pub end_x_m: f32,
    /// End Y coordinate (meters)
    pub end_y_m: f32,
    /// RGB color
    pub color: u32,
    /// Visible flag
    pub visible: bool,
}

impl NanonisClient {
    /// Draw text at a specified point in the scan frame.
    ///
    /// # Arguments
    /// * `x_m` - X coordinate in meters
    /// * `y_m` - Y coordinate in meters
    /// * `text` - Text to draw
    /// * `color` - RGB color value
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use nanonis_rs::NanonisClient;
    ///
    /// let mut client = NanonisClient::new("127.0.0.1", 6501)?;
    /// // Draw red "X" at position (1nm, 2nm)
    /// client.marks_point_draw(1e-9, 2e-9, "X", 0xFF0000)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn marks_point_draw(
        &mut self,
        x_m: f32,
        y_m: f32,
        text: &str,
        color: u32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Marks.PointDraw",
            vec![
                NanonisValue::F32(x_m),
                NanonisValue::F32(y_m),
                NanonisValue::String(text.to_string()),
                NanonisValue::U32(color),
            ],
            vec!["f", "f", "+*c", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Draw text at multiple points in the scan frame.
    ///
    /// # Arguments
    /// * `x_coords_m` - X coordinates in meters
    /// * `y_coords_m` - Y coordinates in meters
    /// * `texts` - Text labels for each point
    /// * `colors` - RGB colors for each point
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_points_draw(
        &mut self,
        x_coords_m: &[f32],
        y_coords_m: &[f32],
        texts: &[String],
        colors: &[u32],
    ) -> Result<(), NanonisError> {
        let num_points = x_coords_m.len() as i32;

        self.quick_send(
            "Marks.PointsDraw",
            vec![
                NanonisValue::I32(num_points),
                NanonisValue::ArrayF32(x_coords_m.to_vec()),
                NanonisValue::ArrayF32(y_coords_m.to_vec()),
                NanonisValue::ArrayString(texts.to_vec()),
                NanonisValue::ArrayU32(colors.to_vec()),
            ],
            vec!["i", "*f", "*f", "+*c", "*I"],
            vec![],
        )?;
        Ok(())
    }

    /// Draw a line in the scan frame.
    ///
    /// # Arguments
    /// * `start_x_m` - Start X coordinate in meters
    /// * `start_y_m` - Start Y coordinate in meters
    /// * `end_x_m` - End X coordinate in meters
    /// * `end_y_m` - End Y coordinate in meters
    /// * `color` - RGB color value
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_line_draw(
        &mut self,
        start_x_m: f32,
        start_y_m: f32,
        end_x_m: f32,
        end_y_m: f32,
        color: u32,
    ) -> Result<(), NanonisError> {
        self.quick_send(
            "Marks.LineDraw",
            vec![
                NanonisValue::F32(start_x_m),
                NanonisValue::F32(start_y_m),
                NanonisValue::F32(end_x_m),
                NanonisValue::F32(end_y_m),
                NanonisValue::U32(color),
            ],
            vec!["f", "f", "f", "f", "I"],
            vec![],
        )?;
        Ok(())
    }

    /// Draw multiple lines in the scan frame.
    ///
    /// # Arguments
    /// * `start_x_m` - Start X coordinates in meters
    /// * `start_y_m` - Start Y coordinates in meters
    /// * `end_x_m` - End X coordinates in meters
    /// * `end_y_m` - End Y coordinates in meters
    /// * `colors` - RGB colors for each line
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_lines_draw(
        &mut self,
        start_x_m: &[f32],
        start_y_m: &[f32],
        end_x_m: &[f32],
        end_y_m: &[f32],
        colors: &[u32],
    ) -> Result<(), NanonisError> {
        let num_lines = start_x_m.len() as i32;

        self.quick_send(
            "Marks.LinesDraw",
            vec![
                NanonisValue::I32(num_lines),
                NanonisValue::ArrayF32(start_x_m.to_vec()),
                NanonisValue::ArrayF32(start_y_m.to_vec()),
                NanonisValue::ArrayF32(end_x_m.to_vec()),
                NanonisValue::ArrayF32(end_y_m.to_vec()),
                NanonisValue::ArrayU32(colors.to_vec()),
            ],
            vec!["i", "*f", "*f", "*f", "*f", "*I"],
            vec![],
        )?;
        Ok(())
    }

    /// Erase a point mark from the scan frame.
    ///
    /// # Arguments
    /// * `point_index` - Index of point to erase (-1 to erase all)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_points_erase(&mut self, point_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "Marks.PointsErase",
            vec![NanonisValue::I32(point_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Erase a line mark from the scan frame.
    ///
    /// # Arguments
    /// * `line_index` - Index of line to erase (-1 to erase all)
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_lines_erase(&mut self, line_index: i32) -> Result<(), NanonisError> {
        self.quick_send(
            "Marks.LinesErase",
            vec![NanonisValue::I32(line_index)],
            vec!["i"],
            vec![],
        )?;
        Ok(())
    }

    /// Show or hide a point mark.
    ///
    /// # Arguments
    /// * `point_index` - Index of point (-1 for all)
    /// * `visible` - True to show, false to hide
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_points_visible_set(
        &mut self,
        point_index: i32,
        visible: bool,
    ) -> Result<(), NanonisError> {
        // Note: 0 = visible, 1 = invisible in the protocol
        let flag = if visible { 0u16 } else { 1u16 };
        self.quick_send(
            "Marks.PointsVisibleSet",
            vec![NanonisValue::I32(point_index), NanonisValue::U16(flag)],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Show or hide a line mark.
    ///
    /// # Arguments
    /// * `line_index` - Index of line (-1 for all)
    /// * `visible` - True to show, false to hide
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_lines_visible_set(
        &mut self,
        line_index: i32,
        visible: bool,
    ) -> Result<(), NanonisError> {
        // Note: 0 = visible, 1 = invisible in the protocol
        let flag = if visible { 0u16 } else { 1u16 };
        self.quick_send(
            "Marks.LinesVisibleSet",
            vec![NanonisValue::I32(line_index), NanonisValue::U16(flag)],
            vec!["i", "H"],
            vec![],
        )?;
        Ok(())
    }

    /// Get information about all drawn point marks.
    ///
    /// # Returns
    /// A vector of [`PointMark`] structs.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_points_get(&mut self) -> Result<Vec<PointMark>, NanonisError> {
        let result = self.quick_send(
            "Marks.PointsGet",
            vec![],
            vec![],
            vec!["i", "*f", "*f", "i", "*+c", "*I", "*I"],
        )?;

        if result.len() >= 7 {
            let num_points = result[0].as_i32()? as usize;
            let x_coords = result[1].as_f32_array()?;
            let y_coords = result[2].as_f32_array()?;
            let texts = result[4].as_string_array()?;
            let colors = result[5].as_u32_array()?;
            let visible = result[6].as_u32_array()?;

            let mut points = Vec::with_capacity(num_points);
            for i in 0..num_points {
                if i < x_coords.len()
                    && i < y_coords.len()
                    && i < texts.len()
                    && i < colors.len()
                    && i < visible.len()
                {
                    points.push(PointMark {
                        x_m: x_coords[i],
                        y_m: y_coords[i],
                        text: texts[i].clone(),
                        color: colors[i],
                        visible: visible[i] != 0,
                    });
                }
            }
            Ok(points)
        } else {
            Ok(vec![])
        }
    }

    /// Get information about all drawn line marks.
    ///
    /// # Returns
    /// A vector of [`LineMark`] structs.
    ///
    /// # Errors
    /// Returns `NanonisError` if communication fails.
    pub fn marks_lines_get(&mut self) -> Result<Vec<LineMark>, NanonisError> {
        let result = self.quick_send(
            "Marks.LinesGet",
            vec![],
            vec![],
            vec!["i", "*f", "*f", "*f", "*f", "*I", "*I"],
        )?;

        if result.len() >= 7 {
            let num_lines = result[0].as_i32()? as usize;
            let start_x = result[1].as_f32_array()?;
            let start_y = result[2].as_f32_array()?;
            let end_x = result[3].as_f32_array()?;
            let end_y = result[4].as_f32_array()?;
            let colors = result[5].as_u32_array()?;
            let visible = result[6].as_u32_array()?;

            let mut lines = Vec::with_capacity(num_lines);
            for i in 0..num_lines {
                if i < start_x.len()
                    && i < start_y.len()
                    && i < end_x.len()
                    && i < end_y.len()
                    && i < colors.len()
                    && i < visible.len()
                {
                    lines.push(LineMark {
                        start_x_m: start_x[i],
                        start_y_m: start_y[i],
                        end_x_m: end_x[i],
                        end_y_m: end_y[i],
                        color: colors[i],
                        visible: visible[i] != 0,
                    });
                }
            }
            Ok(lines)
        } else {
            Ok(vec![])
        }
    }
}
