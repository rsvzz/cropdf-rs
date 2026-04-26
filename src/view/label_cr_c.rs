use cairo::{FontSlant, FontWeight};

use crate::model::{LabelCr, ParamArgs, PointXY};
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
    slant: i32,
    weight: i32,
    x: f64,
    y: f64,
) {
    if !ptr.is_null() {
        unsafe {
            let label = &mut (*ptr).inner; //let mut label in rust.
            let point = PointXY::new(x, y);

            let family = CStr::from_ptr(font); //convert const *char de C a rust.
            let font_family = family.to_str().unwrap().to_string(); //String rust

            let font_slant = match slant {
                0 => FontSlant::Italic,
                1 => FontSlant::Normal,
                2 => FontSlant::Oblique,
                _ => FontSlant::Normal,
            };

            let font_weight = match weight {
                0 => FontWeight::Bold,
                1 => FontWeight::Normal,
                _ => FontWeight::Normal,
            };

            let args = ParamArgs::new(font_family, size, font_slant, font_weight, point);
            label.set_point(Some(args));
        }
    }
}
