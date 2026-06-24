use crate::model::{AxisCr, DrawCrExt, ParamArgsExt, PointXY};

use cairo::Context;

#[derive(Clone)]
pub struct LineCr {
    width: f64,
    height: f64,
    axis: AxisCr,
    thickness: f64,
    setting: Option<PointXY>,
}

impl LineCr {
    pub fn new(_width: f64, _height: f64, _thickness: f64, _axis: AxisCr) -> Self {
        LineCr {
            width: _width,
            height: _height,
            axis: _axis,
            thickness: _thickness,
            setting: None,
        }
    }
}

impl ParamArgsExt for LineCr {
    type Output = Option<PointXY>;
    type InSetting = Option<PointXY>;

    fn set_point(&mut self, _setting: Self::InSetting) {
        self.setting = _setting;
    }

    fn point(&self) -> Self::Output {
        self.setting.clone()
    }
}

impl DrawCrExt for LineCr {
    fn draw(&self, context: Option<&Context>) {
        if let Some(cr) = context {
            if let Some(point) = &self.setting {
                cr.set_source_rgb(0.0, 0.0, 0.0); // negro
                cr.set_line_width(self.thickness);

                match self.axis {
                    AxisCr::Horizontal => {
                        cr.move_to(point.x, point.y);
                        cr.line_to(self.width, point.y);
                    }
                    AxisCr::Vertical => {
                        cr.move_to(point.x, point.y);
                        cr.line_to(point.x, self.height);
                    }
                }

                let _ = cr.stroke();
            }
        }
    }

    fn get_width(&self) -> f64 {
        self.width
    }

    fn get_height(&self) -> f64 {
        self.height
    }

    fn get_point(&self) -> Option<PointXY> {
        self.setting.clone()
    }

    fn set_wight_and_height(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    fn set_xy(&mut self, x: f64, y: f64) {
        if let Some(param) = self.setting.as_mut() {
            *param = PointXY::new(x, y);
            //self.set_point(Some(param));
        }
    }
}
