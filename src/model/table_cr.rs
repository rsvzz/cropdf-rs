use crate::model::{ColumnCR, LimitWHXY};
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
            let mut x: f64;
            let mut y: f64;
            for col in list {
                let pt = col.get_limit();
                x = pt.get_x();
                y = pt.get_y();

                if let Some(items) = col.get_list_items().as_mut() {
                    for item in items {
                        let limit = col.get_limit();
                        item.set_wight_and_height(limit.get_width(), limit.get_height());
                        item.set_xy(x, y);
                        item.draw(ctx);
                    }
                }
            }
        }
    }
}
