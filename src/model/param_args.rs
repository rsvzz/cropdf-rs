use cairo::{FontSlant, FontWeight};
use crate::model::{PointXY};

///Position and type font shows text
#[derive(Clone)]
pub struct ParamArgs {
    pub font_family: String,
    pub font_size: f64,
    pub font_weight: FontWeight,
    pub font_type: FontSlant,
    pub point: Option<PointXY>,
}

impl ParamArgs {
    pub fn new(
        font_family: String,
        size: f64,
        font_slap: FontSlant,
        weight: FontWeight,
        point: Option<PointXY>,
    ) -> Self {
        ParamArgs {
            font_family: font_family,
            font_size: size,
            font_weight: weight,
            font_type: font_slap,
            point: point,
        }
    }

    pub fn new_point(
        point: PointXY,
    ) -> Self {
        ParamArgs {
            font_family: "".to_string(),
            font_size: 0.0,
            font_weight: FontWeight::Normal,
            font_type: FontSlant::Normal,
            point: Some(point),
        }
    }
}

