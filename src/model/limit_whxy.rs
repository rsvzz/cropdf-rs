///position obj and width and height max
#[derive(Clone, Copy)]
pub struct LimitWHXY {
    width: f64,
    height: f64,
    x: f64,
    y: f64,
}

impl LimitWHXY {
    pub fn new(_width: f64, _height: f64, _x: f64, _y: f64) -> Self {
        LimitWHXY {
            width: _width,
            height: _height,
            x: _x,
            y: _y,
        }
    }

    pub fn set_width(&mut self, w: f64){
        self.width = w
    }

    pub fn set_height(&mut self, h: f64){
        self.height = h
    }

    pub fn set_x(&mut self, x: f64){
        self.x = x
    }

    pub fn set_y(&mut self, y: f64){
        self.y = y
    }

    pub fn get_width(&self)-> f64{
        self.width
    }

    pub fn get_height(&self)-> f64{
        self.height
    }

    pub fn get_x(&self) -> f64{
        self.x
    }

    pub fn get_y(&self) -> f64{
        self.y
    }


}
