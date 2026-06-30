use crate::model::{AxisCr, LineCr, ParamArgsExt, PointXY};

#[repr(C)]
pub struct LineCROpaque {
    pub inner: LineCr,
}

#[unsafe(no_mangle)]
pub extern "C" fn line_cr_new(width: f64, height: f64, thickness: f64, _axis: AxisCr) -> *mut LineCROpaque {
    let line = LineCr::new(width, height, thickness, _axis);
    let wrapper = LineCROpaque { inner: line };
    Box::into_raw(Box::new(wrapper))
}

#[unsafe(no_mangle)]
pub extern "C" fn line_cr_set_param_args(ptr: *mut LineCROpaque, x: f64, y: f64) {
    if !ptr.is_null() {
        let line: &mut LineCr;

        unsafe {
            line = &mut (*ptr).inner;
        };

        let point = PointXY::new(x, y);

        line.set_point(Some(point));
    }
}
