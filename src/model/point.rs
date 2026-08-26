#[derive(Clone)]
/// Point draw control in (x, y)
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(_x: f64, _y: f64) -> Self {
        Point { x: _x, y: _y }
    }

    pub fn get_point_x(&self) -> f64{
        self.x
    }

    pub fn get_point_y(&self) -> f64{
        self.y
    }
}

pub trait IPointExt{
    fn get_pt_x(&self) -> f64;

    fn get_pt_y(&self) -> f64;
}
