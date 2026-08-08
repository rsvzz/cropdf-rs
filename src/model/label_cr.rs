use std::cell::RefCell;

use crate::model::{
    DrawCrExt, LimitCR, ParamArgs, ParamArgsExt, PointXY,
    page_ext::{LimitExt, PointExt},
};
use cairo::Context;

#[derive(Clone)]
pub struct LabelCr {
    text: String,
    limit: RefCell<Option<LimitCR>>,
    setting: RefCell<Option<ParamArgs>>,
    point: RefCell<Option<PointXY>>,
}

impl LabelCr {
    pub fn new(text: String) -> Self {
        LabelCr {
            text: text,
            setting: RefCell::new(None),
            limit: RefCell::new(None),
            point: RefCell::new(None),
        }
    }

    pub fn set_text(&mut self, ntext: String) {
        self.text = ntext.to_string();
    }

    pub fn get_text(&self) -> String {
        self.text.to_string()
    }
}

impl ParamArgsExt for LabelCr {
    fn set_param_arg(&self, _setting: Option<&ParamArgs>) {
        if let Some(param) = _setting.cloned() {
             *self.setting.borrow_mut() = Some(param);
        }
       
    }

    fn get_param_arg(&self) -> Option<ParamArgs> {
        self.setting.borrow().clone()
    }
}

impl PointExt for LabelCr {
    fn set_point_xy(&self, x: f64, y: f64) {
        *self.point.borrow_mut() = Some(PointXY::new(x, y));
    }

    fn get_point(&self) -> Option<PointXY> {
        self.point.borrow().clone()
    }
}

impl LimitExt for LabelCr {
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

impl DrawCrExt for LabelCr {
    fn draw(&self, context: Option<&Context>) {
        if let Some(cr) = context {
            if let Some(param) = self.setting.borrow().as_ref() {
                cr.select_font_face(
                    param.font_family.as_str(),
                    param.font_type,
                    param.font_weight,
                );
                cr.set_font_size(param.font_size);

                if let Ok(extents) = cr.text_extents(&self.text) {
                    if let Some(limit) = self.limit.borrow().as_ref(){
                        //width max change font_size
                        if extents.width() > limit.width {
                            cr.set_font_size(param.font_size - 3.0); //change 3pt font_size
                        }
                    }
                }
                //[x, y]
                if let Some(point) = self.point.borrow().as_ref() {
                    cr.move_to(point.x, point.y);
                }

                //show text
                let _ = cr.show_text(&self.text);
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
