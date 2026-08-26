#[derive(Clone)]
pub struct LimitMax {
    width: f64,
    height: f64,
}

impl LimitMax {
    pub fn new(_width: f64, _height: f64) -> Self {
        LimitMax {
            width: _width,
            height: _height,
        }
    }

    pub fn get_limit_width(&self) -> f64{
        self.width
    }

     pub fn get_limit_height(&self) -> f64{
        self.height
    }
}

pub trait ILimitExt {
    fn get_width(&self) -> f64;
    fn get_height(&self) -> f64;
}
