use cairo::{FontSlant, FontWeight};

use crate::model::{FontSlantCr, FontWeightCr, LabelCr, ParamArgs, ParamArgsExt, PointXY};
use std::{ffi::CStr, os::raw::c_char};

#[repr(C)]
pub struct LabelCROpaque {
    pub inner: LabelCr,
}

#[unsafe(no_mangle)]
pub extern "C" fn label_cr_new(path: *const c_char, width: f64, height: f64) -> *mut LabelCROpaque {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap().to_string();

    let label = LabelCr::new(path_str, width, height);
    let wrapper = LabelCROpaque { inner: label };
    Box::into_raw(Box::new(wrapper))
}

#[unsafe(no_mangle)]
pub extern "C" fn label_cr_set_param_args(
    ptr: *mut LabelCROpaque,
    font: *const c_char,
    size: f64,
    slant: FontSlantCr,
    weight: FontWeightCr,
    x: f64,
    y: f64,
) {
    if !ptr.is_null() {
        let label: &mut LabelCr;
        let family: &CStr;
        unsafe {
            label = &mut (*ptr).inner; //let mut label in rust unsafe for ref.
            family = CStr::from_ptr(font); //convert const *char de C a rust unsafe for ref.
        }

        let point = PointXY::new(x, y);

        let font_family = family.to_str().unwrap().to_string(); //String rust

        let font_slant = match slant {
            FontSlantCr::Normal => FontSlant::Normal,
            FontSlantCr::Static => FontSlant::Italic,
            FontSlantCr::Oblique => FontSlant::Oblique,
        };

        let font_weight = match weight {
            FontWeightCr::Normal => FontWeight::Normal,
            FontWeightCr::Bold => FontWeight::Bold,
        };

        let args = ParamArgs::new(font_family, size, font_slant, font_weight, Some(point));
        label.set_point(Some(args));
    }
}
