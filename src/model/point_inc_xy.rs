use crate::model::{LimitWHXY, PointXY};

pub struct PointIncXY {
    limit: LimitWHXY,
    down: f64,
    changed: bool,
}

impl PointIncXY {
    pub fn new(_limit: &LimitWHXY, p_y: f64) -> Self {
        PointIncXY {
            limit: _limit.clone(),
            down: p_y,
            changed: false,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.limit.get_x()
    }

    pub fn get_y(&self) -> f64 {
        self.limit.get_y()
    }

    pub fn get_height_max(&self) -> f64{
        self.limit.get_height()
    }

    pub fn get_point_conv(&self) -> PointXY{
        PointXY::new(self.limit.get_x(), self.limit.get_y())
    }
    ////move (x, y) new position for next line
    pub fn set_limit_point_y(&mut self) {
        if self.changed {
            let mut y = self.limit.get_y();
            y += self.down; //down y
            self.limit.set_y(y); //set 
        }
        else{
            self.changed = true;
        }
    }

    pub fn get_down_y(&self) -> f64{
        self.down
    }
}
