use cairo::{FontSlant, FontWeight};

///Position and type font shows text
#[derive(Clone)]
pub struct ParamArgs {
    pub font_family: String,
    pub font_size: f64,
    pub font_weight: FontWeight,
    pub font_type: FontSlant,
}

impl ParamArgs {
    pub fn new(
        font_family: String,
        size: f64,
        font_slap: FontSlant,
        weight: FontWeight,
    ) -> Self {
        ParamArgs {
            font_family: font_family,
            font_size: size,
            font_weight: weight,
            font_type: font_slap,
        }
    }
}

