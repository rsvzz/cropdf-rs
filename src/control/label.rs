use crate::control::DrawingExt;
use cairo::{Context, FontSlant, FontWeight};

use crate::model::{ILimitExt, IPointExt, LimitMax, Point};

#[derive(Clone)]
pub struct LabelArgs {
    font_family: String,
    font_size: f64,
    font_weight: FontWeight,
    font_type: FontSlant,
}

#[derive(Clone)]
///text show
pub struct Label {
    name: String,
    args: Option<LabelArgs>,
    point: Option<Point>,
    limited: Option<LimitMax>,
}

impl Label {
    /// New Label for showed text
    /// * `name` text show
    /// * `width` max width
    /// * `height` max height
    /// * `x` add (x, y)
    /// * `y` add (x, y)
    /// * `args` font args
    pub fn new(_name: &String, w: f64, h: f64, x: f64, y: f64, arg: &LabelArgs) -> Self {
        Label {
            name: _name.to_string(),
            args: Some(arg.clone()),
            point: Some(Point::new(x, y)),
            limited: Some(LimitMax::new(w, h)),
        }
    }
    /// get name show
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl LabelArgs {
    pub fn new(family: &String, size: f64, font_sl: FontSlant, font_wei: FontWeight) -> Self {
        LabelArgs {
            font_family: family.to_string(),
            font_size: size,
            font_weight: font_wei,
            font_type: font_sl,
        }
    }

    pub fn get_font_family(&self) -> String {
        self.font_family.to_string()
    }

    pub fn get_font_size(&self) -> f64 {
        self.font_size
    }

    pub fn get_font_slant(&self) -> FontSlant {
        self.font_type
    }

    pub fn get_font_weight(&self) -> FontWeight {
        self.font_weight
    }
}

impl IPointExt for Label {
    fn get_pt_x(&self) -> f64 {
        if let Some(point) = &self.point {
            return point.get_point_x();
        }

        return 0.0;
    }

    fn get_pt_y(&self) -> f64 {
        if let Some(point) = &self.point {
            return point.get_point_y();
        }

        return 0.0;
    }
}

impl ILimitExt for Label {
    fn get_width(&self) -> f64 {
        if let Some(limit) = &self.limited {
            return limit.get_limit_width();
        }

        return 0.0;
    }

    fn get_height(&self) -> f64 {
        if let Some(limit) = &self.limited {
            return limit.get_limit_height();
        }

        return 0.0;
    }
}

impl DrawingExt for Label {
    fn draw(&self, ctx: Option<&Context>) {
        if let Some(cr) = ctx {
            if let Some(arg) = &self.args {
                cr.select_font_face(arg.font_family.as_str(), arg.font_type, arg.font_weight);
                cr.set_font_size(arg.font_size);
                if let Ok(extents) = cr.text_extents(&self.name) {
                    if let Some(limit) = &self.limited {
                        //width max change font_size
                        if extents.width() > limit.get_limit_width() {
                            cr.set_font_size(arg.get_font_size() - 3.0); //change 3pt font_size
                        }
                    }
                }

                //[x, y]
                if let Some(pt) = &self.point {
                    cr.move_to(pt.get_point_x(), pt.get_point_y());
                    //show text
                    let _ = cr.show_text(&self.name);
                }
            }
        }
    }
}
