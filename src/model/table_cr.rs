use crate::model::{ColumnCR, LimitWHXY};
use cairo::Context;
use std::cell::RefCell;

#[derive(Clone)]
pub struct TableCR<T> {
    limit: LimitWHXY,
    items: RefCell<Option<Vec<ColumnCR<T>>>>,
}

impl<T> TableCR<T> {
    pub fn new(_limit: &LimitWHXY) -> Self {
        TableCR {
            limit: *_limit,
            items: None.into(),
        }
    }

    pub fn get_limit(&self) -> LimitWHXY {
        self.limit
    }

    pub fn add(&self, col: Option<&ColumnCR<T>>) {
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
                if let Some(param) = col.get_param_arg() {
                    if let Some(items) = col.get_list_items() {
                        for item in items {
                            //if let Some(lbl) = 
                            //item.draw(ctx);
                        }
                    }
                }
            }
        }
    }
}
