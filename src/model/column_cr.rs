use crate::model::{DrawCrExt, DrawCrParam, LabelCr, LimitWHXY, ParamArgs, ParamArgsExt, PointExt, PointXY};
use std::cell::RefCell;

#[derive(Clone)]
///ColumnCR list LabelCR and others
pub struct ColumnCR {
    title: String,
    ///use width and height
    limit: LimitWHXY,
    items: RefCell<Option<Vec<Box<dyn DrawCrExt>>>>,
    //down: f64,
}

/*
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
        }
    }
}
*/

impl ColumnCR {
    /// new Object ColumnCR
    /// * `title` - title column show for header 
    /// * `limit` - param for use width and height
    pub fn new(title: String, limit: &LimitWHXY) -> Self {
        ColumnCR {
            title: title,
            limit: *limit,
            items: None.into(),
            //down: _down,
        }
    }

    pub fn get_limit(&self) -> LimitWHXY {
        self.limit
    }

    pub fn get_title(&self) -> String {
        self.title.to_string()
    }

    ///move (x, y) only Y
    //pub fn get_down_y(&self) -> f64{
    //    self.down
    //}

    /// add items to column
    pub fn add(&self, obj: &dyn DrawCrExt) {
        if let Some(list) = self.items.borrow_mut().as_mut() {
            list.push(obj.clone_box());
        } else {
            *self.items.borrow_mut() = Some(vec![obj.clone_box()]);
        }
    }

    /// items list clone
    pub fn get_list_items(&self) -> Option<Vec<Box<dyn DrawCrExt>>> {
        self.items.borrow_mut().as_mut().cloned()
        /*
        let list = self.items.borrow();
        list.as_ref().map(|vec| {
            vec.iter()
                .map(|item| item.clone_box()) // Invoca nuestro método manual de clonación de cajas
                .collect::<Vec<Box<dyn DrawCrParam<T>>>>() // Reconstruye el nuevo Vector
        })
        */
    }
}

#[derive(Clone)]
///LabelCr for Column show
pub struct ColumnTextCR {
    lbl: LabelCr,
}

impl ColumnTextCR {
    /// new object text name column and limit width and height needed
    ///text
    pub fn new(text: String, limit: &LimitWHXY) -> Self {
        let mut _lbl = LabelCr::new(text, 0.0, 0.0);
        _lbl.set_width(limit.get_width());
        _lbl.set_height(limit.get_height());
        ColumnTextCR { lbl: _lbl }
    }

    pub fn get_text(&self) -> String {
        self.lbl.get_text()
    }
}

impl DrawCrParam<Option<&ParamArgs>> for ColumnTextCR {
    fn set_param(&mut self, param: Option<&ParamArgs>) {
        self.lbl.set_point(param.cloned());
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
    
    fn set_wight_and_height(&mut self, width: f64, height: f64) {
        self.lbl.set_width(width);
        self.lbl.set_height(height);
    }
    
    fn set_xy(&mut self, x: f64, y: f64) {
        self.lbl.set_point_xy(x, y);
    }
}
