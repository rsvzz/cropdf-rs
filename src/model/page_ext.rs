use crate::model::{ParamArgs, PointXY};
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
    fn get_width(&self) -> f64;
    fn get_height(&self) -> f64;
    fn get_point(&self) -> Option<PointXY>;
    fn set_wight_and_height(&mut self, width: f64, height: f64);
    fn set_xy(&mut self, x: f64, y: f64);
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
    fn set_param(&mut self, param: T);
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
pub trait ParamArgsExt<T> {
    fn set_point(&mut self, _setting: T);
    fn point(&self) -> T;
}

pub trait PointExt {
    fn set_point_xy(&mut self, x: f64, y: f64);
}

