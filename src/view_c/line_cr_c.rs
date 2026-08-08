use crate::model::{AxisCr, LimitExt, LineCr, PointExt, PointXY};

#[repr(C)]
pub struct LineCROpaque {
    pub inner: LineCr,
}

#[unsafe(no_mangle)]
pub extern "C" fn line_cr_new(
    width: f64,
    height: f64,
    thickness: f64,
    _axis: AxisCr,
) -> *mut LineCROpaque {
    let mut line = LineCr::new(thickness, _axis);
    line.set_limit_wh(width, height);
    let wrapper = LineCROpaque { inner: line };
    Box::into_raw(Box::new(wrapper))
}

#[unsafe(no_mangle)]
pub extern "C" fn line_cr_set_param_args(ptr: *mut LineCROpaque, x: f64, y: f64) {
    if !ptr.is_null() {
        unsafe {
            let line = &mut (*ptr).inner;
            let point = PointXY::new(x, y);
            line.set_point_xy(point.x, point.y);
        };
    }
}
