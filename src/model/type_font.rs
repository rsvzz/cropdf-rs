#[repr(C)]
pub enum FontSlantCr{
    Normal,
    Static,
    Oblique,
}

#[repr(C)]
pub enum FontWeightCr{
    Normal,
    Bold,
}

#[repr(C)]
#[derive(Clone)]
///H or V orientation
pub enum AxisCr {
    Horizontal,
    Vertical
}
