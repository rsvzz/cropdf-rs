mod column_cr;
mod content_page;
mod label_cr;
mod limit_whxy;
mod line_cr;
mod page_ext;
mod param_args;
mod point_xy;
mod table_cr;
mod type_font;

pub use cairo::{FontSlant, FontWeight};
pub use label_cr::LabelCr;
pub use line_cr::LineCr;
pub use page_ext::{
    BodyExt, ContentPageExt, DrawCrExt, DrawMaxPG, FooterExt, HeaderExt, ParamArgsExt, DrawCrParam, DrawCrParamClone,
};
pub use param_args::ParamArgs;
pub use point_xy::PointXY;
pub use type_font::{AxisCr, FontSlantCr, FontWeightCr};

pub use content_page::{BodyPG, FooterPG, HeaderPG};

pub use column_cr::{ColumnCR, ColumnTextCR};
pub use limit_whxy::LimitWHXY;
pub use table_cr::TableCR;
