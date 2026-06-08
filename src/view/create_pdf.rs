use cairo::PdfSurface;
use crate::model::{ContentPageExt, DrawCrExt};
use cairo::Context;

///Create page for draw file pdf width max and height max of page
#[derive(Clone)]
pub struct CreatePDF {
    //path file
    path: String,
    //page width max of page for draw
    width: f64,
    // height max of page
    height: f64,
    // context cairo
    context: Option<Context>,
    // Pfg Surface
    surface: Option<PdfSurface>,
}

///Difine PDF
impl CreatePDF {
    ///Create Surface width and height
    pub fn new(_path: String, _width: f64, _height: f64) -> Self {
        CreatePDF {
            path: _path,
            width: _width,
            height: _height,
            context: None,
            surface: None,
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
                self.surface = Some(face.clone());
                match Context::new(face) {
                    Ok(ctx) => self.context = Some(ctx),
                    Err(_) => self.surface = None,
                }
            }
            Err(_) => self.surface = None,
        }
    }

    pub fn new_page(&self) {
        if let Some(ctx) = &self.context {
            //create new page ref
            let _ = ctx.show_page();
        }
    }
    ///drop close surface.
    pub fn drop(&mut self) {
        //take() clear obj
        if let Some(surface) = self.surface.take() {
            surface.finish();
            let _ = drop(surface);
        }
    }

    pub fn get_context(&self) -> Option<&Context>{
        self.context.as_ref()
    }
}

impl ContentPageExt for CreatePDF {
    fn add(&self, obj: &dyn DrawCrExt) {
        obj.draw(self.context.as_ref());
    }
}
