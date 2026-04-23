#[derive(Clone)]
pub struct PointXY {
   pub x: f64,
   pub y: f64,
}

impl PointXY {
    pub fn new(_x: f64, _y: f64) -> Self {
        PointXY { x: _x, y: _y }
    }
}