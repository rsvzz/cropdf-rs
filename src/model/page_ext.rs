use crate::model::{ParamArgs};
use std::cell::RefCell;
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


pub trait DrawCrExt {
    fn draw(&self, context: RefCell<Option<Context>>);
}