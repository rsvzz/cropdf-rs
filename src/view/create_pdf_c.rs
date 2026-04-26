use crate::{view::CreatePDF, view::LabelCROpaque};
use std::{ffi::CString, os::raw::c_char};

/// Tipo opaco para C
#[repr(C)]
pub struct CreatePDFOpaque {
    pub inner: CreatePDF,
}

#[unsafe(no_mangle)]
pub extern "C" fn create_pdf_new(
    path: *const c_char,
    width: f64,
    height: f64,
) -> *mut CreatePDFOpaque {
    let c_str = unsafe { std::ffi::CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap().to_string();

    let pdf = CreatePDF::new(path_str, width, height);
    let wrapper = CreatePDFOpaque { inner: pdf };
    Box::into_raw(Box::new(wrapper))
}

#[unsafe(no_mangle)]
pub extern "C" fn create_pdf_free(ptr: *mut CreatePDFOpaque) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn create_pdf_get_path(ptr: *mut CreatePDFOpaque) -> *const c_char {
    if ptr.is_null() {
        return std::ptr::null();
    }

    let pdf = unsafe { &(*ptr).inner };
    let path_str = pdf.get_path(); // esto devuelve un String

    // Convertir a CString
    let c_string = CString::new(path_str).unwrap();

    // Importante: usamos `into_raw` para que C pueda usarlo
    c_string.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn create_pdf_surface(ptr: *mut CreatePDFOpaque) {
    if ptr.is_null() {
        return
    }

   let pdf = unsafe { &mut (*ptr).inner };
   pdf.create_surface();

}

#[unsafe(no_mangle)]
pub extern "C" fn create_pdf_new_page(ptr: *mut CreatePDFOpaque) {
    if ptr.is_null() {
        return
    }

   let pdf = unsafe { & (*ptr).inner };
   pdf.new_page();
   
}

#[unsafe(no_mangle)]
pub extern "C" fn create_pdf_drop(ptr: *mut CreatePDFOpaque) {
    if ptr.is_null() {
        return
    }

   let pdf = unsafe { & (*ptr).inner };
   pdf.drop();
   
}

#[unsafe(no_mangle)]
pub extern "C" fn create_pdf_add(ptr: *mut CreatePDFOpaque, label_ptr: *mut LabelCROpaque) {
    if ptr.is_null() || label_ptr.is_null() {
        println!("ptr o lbl_ptr is null");
        return
    }

   let pdf = unsafe { & (*ptr).inner };
   let label = unsafe {&(*label_ptr).inner};

   pdf.add(&*label);
   
}
