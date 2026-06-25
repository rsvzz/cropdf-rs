use crate::model::LimitWHXY;

pub struct PointIncXY {
    limit: LimitWHXY,
    down: f64,
}

impl PointIncXY {
    pub fn new(_limit: &LimitWHXY, p_y: f64) -> Self {
        PointIncXY {
            limit: _limit.clone(),
            down: p_y,
        }
    }

    pub fn get_x(&self) -> f64{
         self.limit.get_x()
    }

    pub fn get_y(&self) -> f64{
        self.limit.get_y()
    }
    ////move (x, y) new position for next line
    pub fn set_limit_point_y(&mut self) {
        let mut y = self.limit.get_y();
        y += self.down; //down y
        self.limit.set_y(y); //set 
    }
}
