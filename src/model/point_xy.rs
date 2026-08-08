#[derive(Clone, Copy)]
pub struct PointXY {
    pub x: f64,
    pub y: f64,
}

impl PointXY {
    pub fn new(_x: f64, _y: f64) -> Self {
        PointXY { x: _x, y: _y }
    }
}

#[derive(Clone, Copy)]
/// LimitCR width and height max for control CR.
pub struct LimitCR {
    /// width max
    pub width: f64,
    /// height max
    pub height: f64,
}

impl LimitCR {
    pub fn new(w: f64, h: f64) -> Self {
        LimitCR {
            width: w,
            height: h,
        }
    }
}
