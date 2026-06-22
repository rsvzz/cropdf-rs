use crate::model::{ContentPageExt, DrawCrExt, DrawMaxPG};
use cairo::Context;
use std::{cell::RefCell};
#[derive(Clone)]
///Content object for header.
pub struct HeaderPG {
    ///width max used for header
    width: f64,
    ///height max used for header
    height: f64,
    ///use for draw content
    context: Option<Context>,
    ///add header to all page is true false not
    is_repeat: bool,
    //repeat need items for draw in new page
    items: RefCell<Option<Vec<Box<dyn DrawCrExt>>>>,
}
#[derive(Clone)]
///Content BodyPG Draw ending headerpg
pub struct BodyPG {
    width: f64,
    height: f64,
    context: Option<Context>,
    header_w: f64,
    header_h: f64,
}

#[derive(Clone)]
/// footer content page.
pub struct FooterPG {
    width: f64,
    height: f64,
    context: Option<Context>,
}

impl HeaderPG {
    pub fn new(ctx: Option<Context>, w: f64, h: f64, _is_repeat: bool) -> Self {
        HeaderPG {
            width: w,
            height: h,
            context: ctx.clone(),
            is_repeat: _is_repeat,
            items: None.into()
        }
    }

    /// repeat header is_repeat true
    pub fn draw_repeat(&self){
        if self.is_repeat{
            if let Some(items) = self.items.borrow().as_ref(){
                for item in items {
                    item.draw(self.context.as_ref());
                }
            } 
        }
    }

    pub fn set_repeat(&mut self, _is_repeat: bool){
        self.is_repeat = _is_repeat;
    }

    pub fn get_repeat(&self) -> bool{
        self.is_repeat
    }
}

impl DrawMaxPG for HeaderPG {
    fn get_height(&self) -> f64 {
        self.height
    }

    fn get_width(&self) -> f64 {
        self.width
    }
}

impl BodyPG {

    pub fn new(ctx: Option<&Context>, w: f64, h: f64) -> Self {
        BodyPG {
            width: w,
            height: h,
            context: ctx.cloned(),
            header_h: 0.0,
            header_w: 0.0,
        }
    }

    pub fn set_header_pg(&mut self, header: &dyn DrawMaxPG){
        self.header_w = header.get_width();
        self.header_h = header.get_height();
    }
}

impl ContentPageExt for BodyPG {
    fn add(&self, obj: &dyn DrawCrExt) {
          if let Some(point) = obj.get_point() {
            //point draw in header width and height max
            if self.width >= point.x && self.height >= point.y {
                obj.draw(self.context.as_ref());
            }
        }
    }
}

impl DrawMaxPG for BodyPG {
    fn get_height(&self) -> f64 {
        self.height
    }

    fn get_width(&self) -> f64 {
        self.width
    }
}

impl FooterPG {
    pub fn new(ctx: Option<&Context>, w: f64, h: f64) -> Self {
        FooterPG {
            width: w,
            height: h,
            context: ctx.cloned(),
        }
    }
}

impl DrawMaxPG for FooterPG {
    fn get_height(&self) -> f64 {
        self.height
    }

    fn get_width(&self) -> f64 {
        self.width
    }
}

impl ContentPageExt for HeaderPG {
    fn add(&self, obj: &dyn DrawCrExt) {
        if let Some(point) = obj.get_point() {
            //point draw in header width and height max
            if self.width >= point.x && self.height >= point.y {
                obj.draw(self.context.as_ref());

                if let Some(_items)  = 
                self.items.borrow_mut().as_mut(){
                    _items.push(obj.clone_box());
                }
                else {
                    *self.items.borrow_mut() = 
                    Some(vec![obj.clone_box()]);
                    
                }
            }
            //self >= obj width and height for drawing
        }
    }
}

impl ContentPageExt for FooterPG {
    fn add(&self, obj: &dyn DrawCrExt) {
        //self >= obj width and height for drawing
        if self.width >= obj.get_width() && self.height >= obj.get_height() {
            obj.draw(self.context.as_ref());
        }
    }
}
