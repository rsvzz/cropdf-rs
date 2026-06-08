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

pub trait ParamArgsExt {
    type Output;
    type InSetting;

    fn set_point(&mut self, _setting: Self::InSetting);
    fn point(&self) -> Self::Output;
}

