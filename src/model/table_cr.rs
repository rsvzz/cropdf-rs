use crate::model::{AxisCr, ColumnCR, DrawCrExt, LimitWHXY, LineCr, ParamArgsExt, PointIncXY, PointXY};
use cairo::{Context};
use std::{cell::RefCell};

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

    pub fn add(&self, col: Option<ColumnCR>) {
        if let Some(column) = col {
            if let Some(list) = self.items.borrow_mut().as_mut() {
                list.push(column);
            } else {
                *self.items.borrow_mut() = Some(vec![column]);
            }
        }
    }

    /// draw all Column
    pub fn show(&self, ctx: Option<&Context>) {
        // get columns added for table
        if let Some(list) = self.items.borrow().as_ref() {
            //run columns

            //  | 1  | 2  | 3  |
            //  ----------------  ^ | ^
            //  | 4  | 5  | 6  |  (x, y, p)
            //  | 0  | 1  | 0  |

            let mut list_vec: Vec<LineTable> = vec![];
            let mut item_ps = ItemPosition::new();
            for col in list {
                let pt = col.get_limit();

                let point = PointXY::new(pt.get_x(), pt.get_y());

                let mut line_t: LineTable = LineTable::new(point);
                item_ps.set_column(); //increment columns

                let mut item_c: u32 = 0;

                if let Some(items) = col.get_list_items().as_mut() {
                    let mut point_inc = PointIncXY::new(&pt, col.get_down_y());
                    //let mut position: u32 = 0; //read positon items for column
                    //run item column
                    for item in items {
                        //items empty or full
                        item.set_wight_and_height(pt.get_width(), pt.get_height());
                        point_inc.set_limit_point_y(); // move down
                        item.set_xy(point_inc.get_x(), point_inc.get_y());
                        item.draw(ctx);

                        line_t.set_width_inc(col.get_limit().get_width());
                        //line_t.add_column(&ItemPosition::new(p_col, position, point_inc.get_point_conv()));
                        item_c += 1;
                    }
                    item_ps.set_position(item_c);
                    list_vec.push(line_t);
                }

                //p_col += 1; //position column
            }

            let mut col_status = false;
            
            let mut point_inc: Option<PointIncXY> = None;

            for i in 0..item_ps.get_items() {
                
                if !col_status {
                     let mut col_limit = LimitWHXY::new(0.0, 0.0, 0.0, 0.0);
                     
                    for c in 0..item_ps.get_columns() {
                        if let Some(column) = list.get(c as usize) {
                            let limit_cols = column.get_limit();
                            // sum width 
                            col_limit.set_width(limit_cols.get_width() + col_limit.get_width());
                            col_limit.set_height(limit_cols.get_height() + col_limit.get_height());
                            
                            if c == 0{
                                point_inc = Some(PointIncXY::new(&limit_cols, column.get_down_y()));

                                col_limit.set_x(col_limit.get_x());
                                col_limit.set_y(col_limit.get_y());
                            }
                            
                           
                        }
                    }
                    let limit = LimitWHXY::new(col_limit.get_width(), col_limit.get_height(), col_limit.get_x(), col_limit.get_y());
                    item_ps.set_limit_all(Some(&limit));
                     println!("col: {} items: {}", limit.get_width(), i);
                }
                
                if let Some(limit) = item_ps.get_limit(){
                    let mut line = LineCr::new( limit.get_width(), 5.0, 1.0, AxisCr::Horizontal);

                    if let Some(point) = point_inc.as_mut(){
                        point.set_limit_point_y();

                          line.set_point(Some(PointXY::new(point.get_x(), point.get_y() + 5.0)));
                    } 
                  
                    line.draw(ctx);
                }
                
                col_status = true;
            }
            println!(
                "items {}, cols {}",
                item_ps.get_items(),
                item_ps.get_columns()
            );
        }
    }
}

// item in col
// col positon
// 0   0
// 0   1
// 1   0
// 1   1

//  | O | 1 |
//

#[derive(Clone, Copy)]
struct ItemPosition {
    position: u32,
    col: u32,
    limit: Option<LimitWHXY>,
}

impl ItemPosition {
    fn new() -> Self {
        ItemPosition {
            position: 0,
            col: 0,
            limit: None
        }
    }

    fn set_limit_all(&mut self, _limit: Option<&LimitWHXY>){
        self.limit = _limit.cloned();
    }

    fn get_limit(&self) -> Option<LimitWHXY>{
        self.limit
    }

    fn set_position(&mut self, num: u32) {
        if num > self.position {
            self.position = num;
        }
    }

    fn set_column(&mut self) {
        self.col += 1;
    }

    fn get_items(&self) -> u32 {
        self.position
    }

    fn get_columns(&self) -> u32 {
        self.col
    }
}

#[derive(Clone)]
struct LineTable {
    start_point: PointXY,
    width: f64,
    height: f64,
}

impl LineTable {
    pub fn new(point: PointXY) -> Self {
        LineTable {
            start_point: point,
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    ///increment height for column vertical
    pub fn set_height_inc(&mut self, h: f64) {
        self.height += h
    }
    ///increment width for columns horizontal
    pub fn set_width_inc(&mut self, w: f64) {
        self.width += w
    }
}
