use crate::model::{
    AxisCr, ColumnCR, ColumnTextCR, DrawCrExt, DrawCrParam, LimitWHXY, LineCr, ParamArgs,
    ParamArgsExt, PointIncXY, PointXY, point_xy,
};
use cairo::{Context, FontSlant, FontWeight};
use std::cell::RefCell;

#[derive(Clone)]
///T is Items
pub struct TableCR {
    limit: LimitWHXY,
    items: RefCell<Option<Vec<ColumnCR>>>,
    thinkness: f64,
    down: f64,
}

impl TableCR {
    pub fn new(_limit: &LimitWHXY, _thinkness: f64, _down: f64) -> Self {
        TableCR {
            limit: *_limit,
            items: None.into(),
            thinkness: _thinkness,
            down: _down,
        }
    }

    pub fn get_down_y_table(&self) -> f64 {
        self.down
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

    ///run items column
    fn run_items_column(
        &self,
        ctx: Option<&Context>,
        col_items: &mut Option<Vec<Box<dyn DrawCrExt>>>,
        limit_col: &LimitWHXY,
        pt_inc: &mut PointIncXY,
        item_ps: &mut ItemPosition,
        title: String,
    ) {
        if let Some(items) = col_items.as_mut() {
            let mut item_c: u32 = 0;

            //draw title column

            let param_t = ParamArgs::new(
                "Sans".to_string(),
                15.0,
                FontSlant::Normal,
                FontWeight::Bold,
                Some(PointXY::new(50.0, 50.0)),
            );

            let mut col_title = ColumnTextCR::new(title, &limit_col);

            col_title.set_param(Some(&param_t));

            col_title.set_wight_and_height(limit_col.get_width(), limit_col.get_height());
            // (y) for next position down item
            pt_inc.set_limit_point_y();
            // set (x, y) for item with pt_inc sum (y) down
            // move 2.0 right | --
            col_title.set_xy(
                pt_inc.get_x() + 2.0,
                pt_inc.get_y() + (self.get_down_y_table() - 4.0),
            );

            col_title.draw(ctx);
            // end tile column first draw

            for item in items {
                //items empty width and height set for column width and height
                item.set_wight_and_height(limit_col.get_width(), limit_col.get_height());
                // (y) for next position down item
                pt_inc.set_limit_point_y();
                // set (x, y) for item with pt_inc sum (y) down
                // move 2.0 right | --
                item.set_xy(
                    pt_inc.get_x() + 2.0,
                    pt_inc.get_y() + (self.get_down_y_table() - 4.0), //up
                );
                println!(
                    "item: {} x {} y {}",
                    item_c,
                    pt_inc.get_x(),
                    (pt_inc.get_y() + pt_inc.get_down_y())
                );
                //draw items with down
                item.draw(ctx);
                //increment
                item_c += 1;
            }

            println!("*****************");
            item_ps.set_position(item_c);
        }
    }

    /// process column
    fn column_item(&self, ctx: Option<&Context>, col: &ColumnCR, item_ps: &mut ItemPosition) {
        item_ps.set_column(); // count column is needed
        let limit_col = col.get_limit(); // position column
        //let point = PointXY::new(pt.get_x(), pt.get_y());
        let mut point_inc = PointIncXY::new(&limit_col, self.get_down_y_table());

        item_ps.set_down_y(self.get_down_y_table());

        self.run_items_column(
            ctx,
            &mut col.get_list_items(),
            &limit_col,
            &mut point_inc,
            item_ps,
            col.get_title(),
        );
    }

    ///iter all column
    fn run_column_for_items(&self, ctx: Option<&Context>, items: &Vec<ColumnCR>) {
        let mut item_ps = ItemPosition::new(); // save items for columns max items
        //let mut col_count = 0;
        //point table start position (x, y)
        let point_tb = PointXY::new(self.get_limit().get_x(), self.get_limit().get_y());
        //count width and height sum in all columns
        let mut line_t: LineTable = LineTable::new();
        println!("table x:{} y: {}", point_tb.x, point_tb.y);
        println!("*****************");
        for col in items {
            self.column_item(ctx, col, &mut item_ps);
            let col_limit = col.get_limit();
            line_t.set_width_inc(col_limit.get_width());
            line_t.set_height_inc(col_limit.get_height());
            //col_count += 1;
        }

        let mut first_line = LineCr::new(line_t.get_width(), 2.0, 2.0, AxisCr::Horizontal);
        first_line.set_point(Some(point_tb));
        first_line.draw(ctx);

        let mut col_status = false;

        let mut point_inc: Option<PointIncXY> = None;

        for i in 0..item_ps.get_items() + 1 {
            //+1 is tittle
            if !col_status {
                let mut col_limit = LimitWHXY::new(0.0, 0.0, 0.0, 0.0);

                for c in 0..item_ps.get_columns() {
                    if let Some(column) = items.get(c as usize) {
                        let limit_cols = column.get_limit();
                        // sum width
                        col_limit.set_width(limit_cols.get_width() + col_limit.get_width());
                        col_limit.set_height(limit_cols.get_height() + col_limit.get_height());

                        if c == 0 {
                            point_inc = Some(PointIncXY::new(&limit_cols, self.get_down_y_table()));

                            if let Some(point) = point_inc.as_mut() {
                                point.set_limit_point_y();
                                col_limit.set_x(point.get_x());
                                col_limit.set_y(self.get_down_y_table());
                            }
                        }
                    }
                }
                let limit = LimitWHXY::new(
                    col_limit.get_width(),
                    col_limit.get_height(),
                    col_limit.get_x(),
                    col_limit.get_y(),
                );
                item_ps.set_limit_all(Some(&limit));
                println!("col: {} items: {}", limit.get_width(), i);
            }

            if let Some(limit) = item_ps.get_limit() {
                let mut line =
                    LineCr::new(limit.get_width(), 2.0, self.thinkness, AxisCr::Horizontal);

                //default line for table
                let mut line_cp = line.clone();
                if i == 0 {
                    println!("x {} , y {}", self.limit.get_x(), self.limit.get_y());

                    let point_t = PointXY::new(self.limit.get_x(), self.limit.get_y());
                    line_cp.set_point(Some(point_t));
                    line_cp.draw(ctx);

                    if let Some(point_ic) = point_inc.as_ref() {
                        line_cp.set_axis(AxisCr::Vertical);
                        line_cp.set_wight_and_height(
                            5.0,
                            point_ic.get_y()
                                + (item_ps.get_down_y() * (item_ps.get_items() + 1) as f64),
                        );

                        // start line vertical
                        line_cp.draw(ctx);
                        
                       let point_s = PointXY::new(item_ps.get_limit().unwrap().get_width(), self.limit.get_y()); //max width 
                       line_cp.set_point(Some(point_s));
                        //end line vertical
                        println!("2 line v x:{}, y:{}", line_cp.get_point().unwrap().x, line_cp.get_point().unwrap().y);
                        line_cp.draw(ctx);
                    }
                }

                if let Some(point) = point_inc.as_mut() {
                    point.set_limit_point_y();
                    line.set_point(Some(PointXY::new(point.get_x(), point.get_y())));
                }

                line.draw(ctx);
            }

            col_status = true;
        }
    }

    /// draw all Column
    pub fn show(&self, ctx: Option<&Context>) {
        // get columns added for table
        if let Some(list) = self.items.borrow().as_ref() {
            self.run_column_for_items(ctx, list);
            /*
            //run columns
            let mut list_vec: Vec<LineTable> = vec![];
            let mut item_ps = ItemPosition::new();

            for col in list {
                let pt = col.get_limit();

                let point = PointXY::new(pt.get_x(), pt.get_y());

                let mut line_t: LineTable = LineTable::new(point);
                item_ps.set_column(); //increment columns

                let mut item_c: u32 = 0;

                if let Some(items) = col.get_list_items().as_mut() {

                    if item_c == 0{
                        item_ps.set_down_y(col.get_down_y()); //down after use
                    }
                    let mut point_inc = PointIncXY::new(&pt, col.get_down_y() + 5.0);
                    //let mut position: u32 = 0; //read positon items for column
                    //run item column
                    for item in items {
                        //items empty or full
                        item.set_wight_and_height(pt.get_width(), pt.get_height());
                        point_inc.set_limit_point_y(); // move down
                        //--> | 2.0 padiding
                        item.set_xy(point_inc.get_x() + 2.0, point_inc.get_y()+ point_inc.get_down_y());
                        item.draw(ctx);

                        line_t.set_width_inc(col.get_limit().get_width());
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

                            if c == 0 {
                                point_inc = Some(PointIncXY::new(&limit_cols, column.get_down_y() + 5.0));

                                col_limit.set_x(col_limit.get_x());
                                col_limit.set_y(col_limit.get_y());
                            }
                        }
                    }
                    let limit = LimitWHXY::new(
                        col_limit.get_width(),
                        col_limit.get_height(),
                        col_limit.get_x(),
                        col_limit.get_y(),
                    );
                    item_ps.set_limit_all(Some(&limit));
                    println!("col: {} items: {}", limit.get_width(), i);
                }

                if let Some(limit) = item_ps.get_limit() {
                    let mut line =
                        LineCr::new(limit.get_width(), 5.0, self.thinkness, AxisCr::Horizontal);

                    //default line for table
                    let mut line_cp = line.clone();
                    if i == 0 {
                        println!("x {} , y {}", self.limit.get_x(), self.limit.get_y());

                        let point_t = PointXY::new(self.limit.get_x(), self.limit.get_y());
                        line_cp.set_point(Some(point_t));
                        line_cp.draw(ctx);


                        if let Some(point_ic) =  point_inc.as_ref(){
                            line_cp.set_axis(AxisCr::Vertical);
                            line_cp.set_wight_and_height(5.0, ((point_ic.get_y() + (item_ps.get_down_y() *  (item_ps.get_items()) as f64))));
                            line_cp.draw(ctx);
                        }


                    }

                    if let Some(point) = point_inc.as_mut() {
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

            */
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
    down_y: f64,
}

impl ItemPosition {
    fn new() -> Self {
        ItemPosition {
            position: 0,
            col: 0,
            limit: None,
            down_y: 0.0,
        }
    }

    fn get_down_y(&self) -> f64 {
        self.down_y
    }

    fn set_down_y(&mut self, down: f64) {
        self.down_y = down
    }

    fn set_limit_all(&mut self, _limit: Option<&LimitWHXY>) {
        self.limit = _limit.cloned();
    }

    fn get_limit(&self) -> Option<LimitWHXY> {
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
    width: f64,
    height: f64,
}

impl LineTable {
    pub fn new() -> Self {
        LineTable {
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
