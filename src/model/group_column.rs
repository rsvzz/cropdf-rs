use crate::model::ColumnCR;
use std::cell::{RefCell};

#[derive(Clone)]
///Group Column
pub struct GroupColumn {
    /// columns
    items: RefCell<Option<Vec<ColumnCR>>>,
}

pub struct GroupItem{
    group: Option<ColumnCR>,
    items: RefCell<Option<Vec<ColumnCR>>>,
}

impl GroupColumn {
    pub fn new() -> Self {
        GroupColumn { items: None.into() }
    }

    /// add all column for group
    pub fn add(&self, col: Option<&ColumnCR>) {
        if let Some(item) = col.cloned() {
            if let Some(list) = self.items.borrow_mut().as_mut() {
                list.push(item);
            } else {
                *self.items.borrow_mut() = Some(vec![item]);
            }
        }
    }

    /// exist group
    pub fn apply_group(&self) {
        if let Some(list) = self.items.borrow().as_ref() {
            //let list_g: Vec<&mut ColumnCR> = list.iter_mut().filter(| col: &mut ColumnCR| col.get_grouping()).collect(); //mut

            let list_g: Vec<&ColumnCR> = list.iter().filter(| &col| col.get_grouping()).collect();

            let list_n: Vec<&ColumnCR> = list.iter().filter(| &col| !col.get_grouping()).collect();

        }
    }
}
