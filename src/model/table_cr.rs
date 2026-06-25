use crate::model::{ColumnCR, LimitWHXY, PointIncXY};
use cairo::Context;
use std::cell::RefCell;

#[derive(Clone)]
///T is Items
pub struct TableCR {
    limit: LimitWHXY,
    items: RefCell<Option<Vec<ColumnCR>>>,
}

impl TableCR {
    pub fn new(_limit: &LimitWHXY) -> Self {
        TableCR {
            limit: *_limit,
            items: None.into(),
        }
    }

    pub fn get_limit(&self) -> LimitWHXY {
        self.limit
    }

    pub fn add(&self, col: Option<&ColumnCR>) {
        if let Some(list) = self.items.borrow_mut().as_mut() {
            if let Some(column) = col.cloned() {
                list.push(column);
            }
        }
    }

    /// draw all Column
    pub fn show(&self, ctx: Option<&Context>) {
        if let Some(list) = self.items.borrow().as_ref() {

            for col in list {
                let pt = col.get_limit();


                if let Some(items) = col.get_list_items().as_mut() {
                    let mut point = PointIncXY::new(&pt, col.get_down_y());
                    
                    for item in items {
                        
                        item.set_wight_and_height(pt.get_width(), pt.get_height());
                        point.set_limit_point_y(); // move down
                        item.set_xy(point.get_x(), point.get_y());
                        item.draw(ctx);
                    }
                }
            }
        }
    }
}