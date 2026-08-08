use crate::model::PointXY;
/// Get point (x, y) for table first
pub struct PointIncXY {
    /// width column
    width: f64,
    /// point (x, y) table
    point: Option<PointXY>,
    /// down y next line
    down: f64,
}

impl PointIncXY {
    /// Point increment for items columns
    /// * `width` columns
    /// * `point_t` (x, y) is point table
    /// * `down_y` (x, y) is y down for next line
    pub fn new(_width: f64, point_t: Option<&PointXY>, down_y: f64) -> Self {
        PointIncXY {
            width: _width,
            point: point_t.cloned(),
            down: down_y,
        }
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn get_x(&self) -> f64 {
        if let Some(point) = self.point.as_ref() {
            point.x
        } else {
            0.0
        }
    }

    pub fn get_y(&self) -> f64 {
        if let Some(point) = self.point.as_ref() {
            point.y
        } else {
            0.0
        }
    }

    ///move (x, y) new position for next line
    /// * | -
    /// * | -
    pub fn set_point_next_line(&mut self) {
        if let Some(point) = self.point.as_mut() {
            point.y += self.down; // next line
        }
    }

    pub fn get_down(&self) -> f64 {
        self.down
    }
}
