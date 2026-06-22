use crate::model::{DrawCrExt, DrawCrParam, LabelCr, LimitWHXY, ParamArgs, ParamArgsExt, PointXY};
use std::cell::RefCell;

///ColumnCR list LabelCR
pub struct ColumnCR<T> {
    title: String,
    limit: LimitWHXY,
    items: RefCell<Option<Vec<Box<dyn DrawCrParam<T>>>>>,
    param: Option<ParamArgs>,
}

impl<T> Clone for ColumnCR<T> {
    fn clone(&self) -> Self {
        let items_guard = self.items.borrow();

        // Clonamos el Option<Vec<...>> usando clone_box para cada elemento
        let cloned_items = items_guard.as_ref().map(|vec| {
            vec.iter()
                .map(|item| item.clone_box()) // Llama al método del rasgo auxiliar
                .collect::<Vec<_>>()
        });
        Self {
            title: self.title.clone(),
            limit: self.limit.clone(),
            items: RefCell::new(cloned_items),
            param: self.param.clone(),
        }
    }
}

impl<T> ColumnCR<T> {
    pub fn new(_title: String, _limit: &LimitWHXY, _param: Option<&ParamArgs>) -> Self {
        ColumnCR {
            title: _title,
            limit: *_limit,
            items: None.into(),
            param: _param.cloned(),
        }
    }

    pub fn get_limit(&self) -> LimitWHXY {
        self.limit
    }

    pub fn get_title(&self) -> String {
        self.title.to_string()
    }

    /// add items to column
    pub fn add(&self, obj: &dyn DrawCrParam<T>) {
        if let Some(list) = self.items.borrow_mut().as_mut() {
            list.push(obj.clone_box());
        } else {
            *self.items.borrow_mut() = Some(vec![obj.clone_box()]);
        }
    }

    pub fn get_param_arg(&self) -> Option<&ParamArgs> {
        self.param.as_ref()
    }

    /// items list clone
    pub fn get_list_items(&self) -> Option<Vec<Box<dyn DrawCrParam<T>>>> {
        let list = self.items.borrow();
        list.as_ref().map(|vec| {
            vec.iter()
                .map(|item| item.clone_box()) // Invoca nuestro método manual de clonación de cajas
                .collect::<Vec<Box<dyn DrawCrParam<T>>>>() // Reconstruye el nuevo Vector
        })
    }
}

#[derive(Clone)]
///LabelCr for Column show
pub struct ColumnTextCR {
    lbl: LabelCr,
}

impl ColumnTextCR {
    pub fn new(text: String) -> Self {
        let _lbl = LabelCr::new(text, 0.0, 0.0);
        ColumnTextCR { lbl: _lbl }
    }

    pub fn set_limit(&mut self, limit: &LimitWHXY) {
        self.lbl.set_width(limit.get_width());
        self.lbl.set_height(limit.get_height());
    }

    pub fn get_text(&self) -> String {
        self.lbl.get_text()
    }
}

impl DrawCrExt for ColumnTextCR {
    fn draw(&self, context: Option<&cairo::Context>) {
        self.lbl.draw(context);
    }

    fn get_width(&self) -> f64 {
        self.lbl.get_width()
    }

    fn get_height(&self) -> f64 {
        self.lbl.get_height()
    }

    fn get_point(&self) -> Option<PointXY> {
        self.lbl.get_point()
    }
}

impl DrawCrParam<Option<ParamArgs>> for ColumnTextCR {
    fn set_param(&mut self, param: Option<ParamArgs>) {
        self.lbl.set_point(param);
    }
}
