use crate::model::{LimitCR, ParamArgs, PointXY};
use cairo::Context;

pub trait HeaderExt {
    fn create_title_header(&self, title: String, param: &ParamArgs);
}

pub trait BodyExt {
    fn create_title_header(&self, title: String, param: &ParamArgs);
}

pub trait FooterExt {
    fn create_title_header(&self, title: String, param: &ParamArgs);
}

pub trait DrawCrExtClone {
    fn clone_box(&self) -> Box<dyn DrawCrExt>;
}

impl<T> DrawCrExtClone for T
where
    T: 'static + DrawCrExt + Clone,
{
    fn clone_box(&self) -> Box<dyn DrawCrExt> {
        Box::new(self.clone())
    }
}

pub trait DrawCrExt : DrawCrExtClone {
    fn draw(&self, context: Option<&Context>);
    /// get object width and height
    fn get_limits(&self) -> Option<LimitCR>;
    /// get point (x, y)
    fn get_points(&self) -> Option<PointXY>;
}


pub trait DrawCrParamClone<T> {
    fn clone_box(&self) -> Box<dyn DrawCrParam<T>>;
}

impl<T, U> DrawCrParamClone<T> for U
where
    U: 'static + DrawCrParam<T> + Clone,
{
    
    fn clone_box(&self) -> Box<dyn DrawCrParam<T>>{
        Box::new(self.clone())
    }
}

pub trait DrawCrParam<T> : DrawCrParamClone<T>{
    fn set_param(&self, param: T);
}

/// join Clone
impl<T> Clone for Box<dyn DrawCrParamClone<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn DrawCrExt> {
    fn clone(&self) -> Box<dyn DrawCrExt>  {
        self.clone_box()
    }
}
 
pub trait ContentPageExt {
    //type Output;
    fn add(&self, obj: &dyn DrawCrExt);
}

///Content max for headerpg and bodypg and footerpg
pub trait DrawMaxPG {
    fn get_height(&self) -> f64;
    fn get_width(&self) -> f64;
}

///Point (x, y) for Draw Control
/// * `T` type Data for input and out
pub trait ParamArgsExt {
    fn set_param_arg(&self, _setting: Option<&ParamArgs>);
    fn get_param_arg(&self) -> Option<ParamArgs>;
}

/// point (x, y) for draw controls 
pub trait PointExt {
    /// dont need mut for this
    fn set_point_xy(&self, x: f64, y: f64);
    fn get_point(&self) -> Option<PointXY>;
}

/// Limit width and height for controls
pub trait LimitExt {

    /// set value width and height
    fn set_limit_wh(&self, w: f64, h: f64);
    /// get limit
    fn get_limit_wh(&self) -> Option<LimitCR>;
}