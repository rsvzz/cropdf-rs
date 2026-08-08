use std::cell::RefCell;

use crate::model::{AxisCr, DrawCrExt, LimitCR, LimitExt, PointExt, PointXY};

use cairo::Context;

#[derive(Clone)]
pub struct LineCr {
    axis: AxisCr,
    thickness: f64,
    point: RefCell<Option<PointXY>>,
    limit: RefCell<Option<LimitCR>>,
}

impl LineCr {
    pub fn new(_thickness: f64, _axis: AxisCr) -> Self {
        LineCr {
            limit: RefCell::new(None),
            axis: _axis,
            thickness: _thickness,
            point: RefCell::new(None),
        }
    }

    pub fn set_axis(&mut self, _axis: AxisCr) {
        self.axis = _axis
    }
}

impl PointExt for LineCr {
    fn set_point_xy(&self, x: f64, y: f64) {
        *self.point.borrow_mut() = Some(PointXY::new(x, y));
    }

    fn get_point(&self) -> Option<PointXY> {
        self.point.borrow().clone()
    }
}

impl LimitExt for LineCr {
    fn set_limit_wh(&self, w: f64, h: f64) {
        *self.limit.borrow_mut() = Some(LimitCR {
            width: w,
            height: h,
        })
    }

    fn get_limit_wh(&self) -> Option<LimitCR> {
        self.limit.borrow().clone()
    }
}

impl DrawCrExt for LineCr {
    fn draw(&self, context: Option<&Context>) {
        if let Some(cr) = context {
            if let Some(point) = self.point.borrow().as_ref() {
                cr.set_source_rgb(0.0, 0.0, 0.0); // negro
                cr.set_line_width(self.thickness);
                if let Some(limit) = self.limit.borrow().as_ref() {
                    match self.axis {
                        AxisCr::Horizontal => {
                            cr.move_to(point.x, point.y);
                            cr.line_to(limit.width, point.y);
                        }
                        AxisCr::Vertical => {
                            cr.move_to(point.x, point.y);
                            cr.line_to(point.x, limit.height);
                        }
                    }
                    let _ = cr.stroke();
                }
            }
        }
    }

    fn get_limits(&self) -> Option<LimitCR> {
        self.limit.borrow().clone()
    }

    fn get_points(&self) -> Option<PointXY> {
        self.point.borrow().clone()
    }
}
