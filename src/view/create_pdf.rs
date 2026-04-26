use cairo::PdfSurface;
use std::cell::RefCell;

use crate::model::DrawCrExt;
use cairo::Context;

///width and height move_to [x, y]
#[derive(Clone)]
pub struct CreatePDF {
    path: String,
    width: f64,
    height: f64,
    context: RefCell<Option<Context>>,
    surface: RefCell<Option<PdfSurface>>,
}

///Difine PDF
impl CreatePDF {
    ///Create Surface width and height
    pub fn new(_path: String, _width: f64, _height: f64) -> Self {
        CreatePDF {
            path: _path,
            width: _width,
            height: _height,
            context: RefCell::new(None),
            surface: RefCell::new(None),
        }
    }

    ///Get path .pdf
    pub fn get_path(&self) -> String {
        self.path.to_string()
    }

    pub fn create_surface(&mut self) {
        let surface = match PdfSurface::new(self.width, self.height, self.path.to_string()) {
            Ok(surface) => Ok(surface),
            Err(er) => Err(er),
        };

        match surface {
            Ok(face) => {
                *self.surface.borrow_mut() = Some(face.clone());
                match Context::new(face.clone()) {
                    Ok(ctx) => *self.context.borrow_mut() = Some(ctx),
                    Err(_) => *self.surface.borrow_mut() = None,
                }
            }
            Err(_) => *self.surface.borrow_mut() = None,
        }
    }

    pub fn new_page(&self) {
        if let Some(ctx) = self.context.borrow().as_ref() {
            //create new page ref
            let _ = ctx.show_page();
        }
    }
    ///drop close surface.
    pub fn drop(&self) {
        //take() clear obj
        if let Some(surface) = self.surface.borrow_mut().take() {
            surface.finish();
            let _ = drop(surface);
        }
    }

    pub fn add(&self, obj: &dyn DrawCrExt) {
        obj.draw(self.context.clone());
    }
}
