use cairo::{Context, FontSlant, FontWeight};

use crate::model::{
    DrawCrExt, DrawCrParam, LabelCr, LimitCR, LimitExt, ParamArgs, ParamArgsExt, PointExt,
    PointIncXY, PointXY,
};
use std::cell::RefCell;

#[derive(Clone)]
/// type column for show in column and table
pub enum ColumnTypeCR {
    /// show label text
    Text(RefCell<Option<ColumnTextCR>>),
    //show images in the column
    //Image(ColumnImageCR)
}

#[derive(Clone)]
pub enum ColumnParamCR {
    Text(RefCell<Option<ParamArgs>>),
}
/*
/// Valid items column with data table
pub trait ItemColumnCR {
    /// set item column with table
    /// * `self` &
    /// * `table` container ColumnCR
    fn run_item_for_column(
        &self,
        table: &TableCR,
        ctx: Option<&Content>,
        item_ps: &mut ItemPosition,
    );
}*/

pub trait ColumnDrawCloneExt {
    fn clone_box(&self) -> Box<dyn ColumnDrawExt>;
}

impl<T> ColumnDrawCloneExt for T
where
    T: 'static + ColumnDrawExt + Clone,
{
    fn clone_box(&self) -> Box<dyn ColumnDrawExt> {
        Box::new(self.clone())
    }
}

///Get data for column
pub trait ColumnDrawExt: ColumnDrawCloneExt {
    /// Get item column
    fn get_column_item(&self) -> ColumnTypeCR;
}

///impl Clone in ColumnDrawExt
impl Clone for Box<dyn ColumnDrawExt> {
    fn clone(&self) -> Box<dyn ColumnDrawExt> {
        self.clone_box()
    }
}

#[derive(Clone)]
///ColumnCR list LabelCR and others
pub struct ColumnCR {
    /// title show for column
    title: String,
    /// items columns for draw
    items: RefCell<Option<Vec<Box<dyn ColumnDrawExt>>>>,
    /// Draw (x, y)
    point: RefCell<Option<PointXY>>,
    /// width and height max
    limit: RefCell<Option<LimitCR>>,
    /// position column add
    col_id: RefCell<u32>,
    /// param text show.
    param: RefCell<Option<ColumnParamCR>>,
    /// Is column group
    is_grouping: RefCell<bool>,
}

impl ColumnCR {
    /// new Object ColumnCR
    /// * `title` - title column show for header
    /// * `w` - param for use width
    pub fn new(title: String, w: f64, args: Option<&ColumnParamCR>) -> Self {
        ColumnCR {
            title: title,                                    // need title for show title column
            limit: RefCell::new(Some(LimitCR::new(w, 5.0))), //default 5.0 pt
            point: None.into(),                       //dont need point default
            items: None.into(),                              // dont need items for default
            col_id: RefCell::new(0),
            param: RefCell::new(args.cloned()),
            is_grouping: RefCell::new(false),
        }
    }

    /// set group true or false
    pub fn set_grouping(&self, grouping: bool){
        *self.is_grouping.borrow_mut() = grouping;
    }

    pub fn get_grouping(&self) -> bool{
       self.is_grouping.borrow().clone()
    }
    pub fn get_title(&self) -> String {
        self.title.to_string()
    }

    /// add items to column
    pub fn add(&self, obj: &dyn ColumnDrawExt) {
        if let Some(list) = self.items.borrow_mut().as_mut() {
            list.push(obj.clone_box());
        } else {
            *self.items.borrow_mut() = Some(vec![obj.clone_box()]);
        }
    }

    /// items list clone
    pub fn get_list_items(&self) -> Option<Vec<Box<dyn ColumnDrawExt>>> {
        self.items.borrow_mut().as_mut().cloned()
    }

    pub fn set_col_postion_id(&self, index: u32) {
        *self.col_id.borrow_mut() = index;
    }

    pub fn get_column_index(&self) -> u32 {
        self.col_id.borrow().clone()
    }

    pub fn run_items_draw(&self, ctx: Option<&Context>, next_y: f64) {
        if let Some(list_item_col) = self.get_list_items().as_ref() {
            if let Some(col_wh) = self.limit.borrow().as_ref() {
                if let Some(tb_point) = self.get_point().as_ref() {
                    let mut point_inc = PointIncXY::new(col_wh.width, Some(tb_point), next_y);

                    println!("tb = {}, {}", tb_point.x, tb_point.y);
                    let mut item_count: u32 = 0;
                    if let Some(args) = self.param.borrow().as_ref() {
                        point_inc.set_point_next_line(); //space title
                        let col_title = ColumnTextCR::new(self.get_title());
                        if self.get_column_index() == 0 {
                            col_title.set_point_xy(point_inc.get_x(), point_inc.get_y());
                        } else {
                            // next column width + positon
                            col_title
                                .set_point_xy(col_wh.width + point_inc.get_x(), point_inc.get_y());
                            println!("col_w {}", col_wh.width);
                        }
                        col_title.set_limit_wh(col_wh.width, next_y);
                        let param_t = ParamArgs::new(
                            "Sans".to_string(),
                            12.0,
                            FontSlant::Normal,
                            FontWeight::Bold,
                        );

                        col_title.set_param(Some(&param_t));
                        col_title.draw(ctx);
                        for item in list_item_col {
                            println!("item {}", item_count);
                            let item_t = item.get_column_item();
                            /*
                               1) Sum width and height all column :)
                               2) Get point table for draw item for column
                               3)
                            */

                            match args {
                                ColumnParamCR::Text(text_arg) => {
                                    let param_arg = text_arg.borrow().clone();
                                    match item_t {
                                        ColumnTypeCR::Text(ref col_t) => {
                                            if let Some(col_item) = col_t.borrow().as_ref() {
                                                //add point for items next line
                                                point_inc.set_point_next_line();
                                                println!(
                                                    "item: {} x {} y {}",
                                                    item_count,
                                                    point_inc.get_x(),
                                                    (point_inc.get_y())
                                                );
                                                //sum width add column @
                                                col_item.set_param(param_arg.as_ref()); //dont live
                                                col_item.set_limit_wh(col_wh.width, next_y);

                                                if self.get_column_index() == 0 {
                                                    col_item.set_point_xy(
                                                        point_inc.get_x(),
                                                        point_inc.get_y(),
                                                    );
                                                } else {
                                                    // next column width + positon
                                                    col_item.set_point_xy(
                                                        col_wh.width + point_inc.get_x(),
                                                        point_inc.get_y(),
                                                    );
                                                    println!("col_w {}", col_wh.width);
                                                }

                                                col_item.draw(ctx);

                                                if let Some(l_col) =
                                                    col_item.get_limit_wh().as_ref()
                                                {
                                                    println!("{}-{}", l_col.width, l_col.height);
                                                } else {
                                                    println!("col limit nothing");
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                            // increment item count 0..+1
                            item_count += 1;
                        }
                    }
                }
            }
            //let point_t = self.get_point();
            //next line draw for item
        }
    }
}

impl PointExt for ColumnCR {
    fn set_point_xy(&self, x: f64, y: f64) {
        *self.point.borrow_mut() = Some(PointXY::new(x, y));
    }

    fn get_point(&self) -> Option<PointXY> {
        self.point.borrow().clone()
    }
}

impl LimitExt for ColumnCR {
    fn set_limit_wh(&self, w: f64, h: f64) {
        *self.limit.borrow_mut() = Some(LimitCR {
            width: w,
            height: h,
        })
    }

    fn get_limit_wh(&self) -> Option<LimitCR> {
        self.limit.borrow().clone()
    }
}

#[derive(Clone)]
///LabelCr for Column show
pub struct ColumnTextCR {
    lbl: RefCell<Option<LabelCr>>,
}

impl ColumnTextCR {
    /// new object text name column and limit width and height needed
    /// * `text` name show title column
    /// * `limit` point set position for showing
    pub fn new(text: String) -> Self {
        let _lbl = LabelCr::new(text);
        ColumnTextCR {
            lbl: RefCell::new(Some(_lbl)),
        }
    }

    pub fn get_text(&self) -> Option<String> {
        match self.lbl.borrow().as_ref() {
            Some(label) => Some(label.get_text()),
            _ => None,
        }
    }
}

impl LimitExt for ColumnTextCR {
    fn set_limit_wh(&self, w: f64, h: f64) {
        if let Some(label) = self.lbl.borrow().as_ref() {
            label.set_limit_wh(w, h);
        }
    }

    fn get_limit_wh(&self) -> Option<LimitCR> {
        if let Some(label) = self.lbl.borrow().as_ref() {
            label.get_limits()
        } else {
            None
        }
    }
}

impl PointExt for ColumnTextCR {
    fn set_point_xy(&self, x: f64, y: f64) {
        if let Some(label) = self.lbl.borrow().as_ref() {
            label.set_point_xy(x, y);
        }
    }

    fn get_point(&self) -> Option<PointXY> {
        if let Some(label) = self.lbl.borrow().as_ref() {
            label.get_point()
        } else {
            None
        }
    }
}

impl DrawCrParam<Option<&ParamArgs>> for ColumnTextCR {
    fn set_param(&self, param: Option<&ParamArgs>) {
        if let Some(label) = self.lbl.borrow().as_ref() {
            if let Some(arg) = param {
                label.set_param_arg(Some(arg));
            }
        }
    }
}

impl ColumnDrawExt for ColumnTextCR {
    fn get_column_item(&self) -> ColumnTypeCR {
        ColumnTypeCR::Text(RefCell::new(Some(self.clone())))
    }
}

impl DrawCrExt for ColumnTextCR {
    fn draw(&self, context: Option<&cairo::Context>) {
        if let Some(label) = self.lbl.borrow().as_ref() {
            label.draw(context);
        }
    }

    fn get_limits(&self) -> Option<LimitCR> {
        if let Some(label) = self.lbl.borrow().as_ref() {
            label.get_limits().clone()
        } else {
            None
        }
    }

    fn get_points(&self) -> Option<PointXY> {
        if let Some(label) = self.lbl.borrow().as_ref() {
            label.get_point().clone()
        } else {
            None
        }
    }
}
