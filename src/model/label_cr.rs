use crate::model::{DrawCrExt, ParamArgs, ParamArgsExt, PointXY, page_ext::PointExt};
use cairo::Context;

#[derive(Clone)]
pub struct LabelCr {
    text: String,
    width: f64,
    height: f64,
    setting: Option<ParamArgs>,
}

impl LabelCr {
    pub fn new(text: String, _width: f64, _height: f64) -> Self {
        LabelCr {
            text: text,
            width: _width,
            height: _height,
            setting: None,
        }
    }

    pub fn set_text(&mut self, ntext: String) {
        self.text = ntext.to_string();
    }

    pub fn set_width(&mut self, w: f64) {
        self.width = w;
    }

    pub fn set_height(&mut self, h: f64) {
        self.height = h;
    }

    pub fn get_text(&self) -> String {
        self.text.to_string()
    }
}

impl ParamArgsExt for LabelCr {
    type Output = Option<ParamArgs>;
    type InSetting = Option<ParamArgs>;

    fn set_point(&mut self, _setting: Self::InSetting) {
        self.setting = _setting;
    }

    fn point(&self) -> Self::Output {
        self.setting.clone()
    }
}

impl PointExt for LabelCr {
    fn set_point_xy(&mut self, x: f64, y: f64) {
        let sett = self.setting.as_mut();
        if let Some(param) = sett {
            *param.point.borrow_mut() = Some(PointXY::new(x, y));
            //self.set_point(Some(param));
        }
    }
}

impl DrawCrExt for LabelCr {
    fn draw(&self, context: Option<&Context>) {
        if let Some(cr) = context {
            if let Some(param) = &self.setting {
                cr.select_font_face(
                    param.font_family.as_str(),
                    param.font_type,
                    param.font_weight,
                );
                cr.set_font_size(param.font_size);

                if let Ok(extents) = cr.text_extents(&self.text) {
                    //width max change font_size
                    if extents.width() > self.width {
                        cr.set_font_size(param.font_size - 3.0); //change 3pt font_size
                    }
                }
                //[x, y]
                if let Some(point) = param.point.borrow().as_ref() {
                    cr.move_to(point.x, point.y);
                }

                //show text
                let _ = cr.show_text(&self.text);
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
        if let Some(param) = self.setting.clone() {
            param.point.borrow().as_ref().cloned()
        } else {
            None
        }
    }

    fn set_wight_and_height(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    fn set_xy(&mut self, x: f64, y: f64) {
        let sett = self.setting.as_mut();
        if let Some(param) = sett {
            *param.point.borrow_mut() = Some(PointXY::new(x, y));
            //self.set_point(Some(param));
        }
    }
}
