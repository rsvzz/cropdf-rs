mod label_cr;
mod line_cr;
mod page_ext;
mod param_args;
mod point_xy;
mod type_font;
mod content_page;

pub use cairo::{FontSlant, FontWeight};
pub use label_cr::{LabelCr};
pub use line_cr::{LineCr};
pub use page_ext::{BodyExt, DrawCrExt, FooterExt, HeaderExt, ParamArgsExt, ContentPageExt, DrawMaxPG};
pub use param_args::ParamArgs;
pub use point_xy::PointXY;
pub use type_font::{FontSlantCr, FontWeightCr, AxisCr};

pub use content_page::{HeaderPG, BodyPG, FooterPG,};
