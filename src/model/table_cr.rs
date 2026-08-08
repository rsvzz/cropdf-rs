use crate::model::{
    AxisCr, ColumnCR, ColumnTextCR, ColumnTypeCR, DrawCrExt, DrawCrParam, ItemPosition, LimitCR, LimitWHXY, LineCr, ParamArgs, PointExt, PointIncXY, PointXY, column_cr::ColumnParamCR, item_position::ColumnPosition, page_ext::LimitExt
};
use cairo::{Context, FontSlant, FontWeight};
use std::cell::RefCell;

#[derive(Clone)]
/// TableCR add columns for draw
pub struct TableCR {
    /// items columns
    items: RefCell<Option<Vec<ColumnCR>>>,
    /// line table
    thinkness: f64,
    /// down in y position (x, y)
    down: f64,
    /// font_size
    size: f64,
    /// point (x,  y)
    point: RefCell<Option<PointXY>>,
    /// width and height
    limit: RefCell<Option<LimitCR>>,
}

impl PointExt for TableCR {
    fn set_point_xy(&self, x: f64, y: f64) {
        *self.point.borrow_mut() = Some(PointXY::new(x, y));
    }

    fn get_point(&self) -> Option<PointXY> {
        self.point.borrow().clone()
    }
}

impl LimitExt for TableCR {
    fn set_limit_wh(&self, w: f64, h: f64) {
        *self.limit.borrow_mut() = Some(LimitCR::new(w, h))
    }

    fn get_limit_wh(&self) -> Option<LimitCR> {
        self.limit.borrow().clone()
    }
}

impl TableCR {
    /// Create new object for table
    /// * `_thinlness` heght for line of table
    /// * `down` positon y for (x, y)
    /// * `size_font` size for title column for show
    pub fn new(_thinkness: f64, _down: f64, size_font: f64) -> Self {
        TableCR {
            items: None.into(),
            thinkness: _thinkness,
            down: _down,
            size: size_font,
            point: RefCell::new(None),
            limit: RefCell::new(None),
        }
    }

    pub fn get_size(&self) -> f64 {
        self.size
    }

    pub fn get_down_y_table(&self) -> f64 {
        self.down
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
    /*
    fn set_item_column_values(
        &self,
        item: &mut dyn DrawCrExt,
        limit_col: &LimitWHXY,
        pt_inc: &mut PointIncXY,
    ) {
        item.set_width_and_height(limit_col.get_width(), limit_col.get_height());

        // (y) for next position down item
        pt_inc.set_limit_point_y();
    }

    ///run items for column
    /// * `ctx` Context for cairo
    /// * `col_items` items for columns
    /// * `item_ps` postion items sum
    /// * `title` column name
    fn run_items_column(
        &self,
        ctx: Option<&Context>,
        column: &ColumnCR,
        pt_inc: &mut PointIncXY,
        item_ps: &mut ItemPosition,
        title: String,
    ) {
        if let Some(items) = column.get_list_items().as_ref() {
            let mut item_c: u32 = 0;
            //jump
            let space = 1.5;
            //draw title column

            let param_t = ParamArgs::new(
                "Sans".to_string(),
                self.size,
                FontSlant::Normal,
                FontWeight::Bold,
            );

            let limit_col = column.get_limit_wh().unwrap();
            //not position first items title
            //let mut col_title = ColumnTextCR::new(title);
            // @ pendiente crear posicion del titulo de la column
            //set position first item for column witdout (x, y).
            let mut col_title = ColumnTextCR::new(title);

            col_title.set_param(Some(&param_t));
            col_title.set_limit_wh(limit_col.width, self.get_down_y_table());

            // //set width and height col_title.set_width_and_height(limit_col.get_width(), limit_col.get_height());
            // (y) for next position down item
            pt_inc.set_limit_point_y();
            // set (x, y) for item with pt_inc sum (y) down
            // move 2.0 right | --
            col_title.set_point_xy(
                pt_inc.get_x() + 2.0,
                (pt_inc.get_y() + (self.get_down_y_table() / space).round()) as f64, //@ y set value
            );
            //valid @ si se administra
            item_ps.set_down_y(self.get_down_y_table());

            col_title.draw(ctx);
            // end tile column first draw

            for col_item_draw in items {
                let col_type = col_item_draw.get_column_item();

                let mut _item = match col_type {
                    ColumnTypeCR::Text(ref col_text) => {
                        // @ no vive fixed here // el obj dentro no vive pero si el objeto principal
                        col_text.borrow_mut()
                    }
                };

                if let Some(item) = _item.as_mut() {
                    //no se puede llamar mientras ext exista mas abajo.
                    let point_item: &mut dyn PointExt = item;
                    point_item.set_point_xy(
                        pt_inc.get_x() + 2.0,
                        (pt_inc.get_y() + (self.get_down_y_table() / space).round()) as f64, //up
                    );

                    let item_ext: &mut dyn DrawCrExt = item;
                    //set item properties
                    //self.set_item_column_values(item_ext, column.get_limit_wh().unwrap(), pt_inc);
                    pt_inc.set_limit_point_y();
                    println!(
                        "item: {} x {} y {}",
                        item_c,
                        pt_inc.get_x(),
                        (pt_inc.get_y() + pt_inc.get_down_y())
                    );

                    item_ext.draw(ctx);

                    //increment
                    item_c += 1;

                    //valid @
                    item_ps.set_down_y(self.get_down_y_table());
                }
            }
        }
    }

    /// process column
    /// * `ctx` cairo draw
    /// * `col` column
    /// * `items_ps` manager for column position
    fn column_item(&self, ctx: Option<&Context>, col: &ColumnCR, item_ps: &mut ItemPosition) {
        item_ps.set_column(); // count column is needed
        // position table
        //let point = PointXY::new(pt.get_x(), pt.get_y());
        let point_tb = self.point.as_ref().unwrap();
        let limit_col = col.get_limit_wh().unwrap();

        let mut point_inc = PointIncXY::new(limit_col.width, point_tb.x, self.get_down_y_table());

        item_ps.set_down_y(self.get_down_y_table());

        self.run_items_column(ctx, &col, &mut point_inc, item_ps, col.get_title());
    }
    /// draw line for table
    /// * `ctx` Object cairo for draw
    /// * `line_t` get max width and height of columns
    /// * `item_ps` get columns max added
    /// * `items` columns
    fn set_draw_line_for_table(
        &self,
        ctx: Option<&Context>,
        line_t: &mut LineTable,
        item_ps: &mut ItemPosition,
        items: &Vec<ColumnCR>,
    ) {
        //let point_tb = PointXY::new(self.limit.get_x(), self.limit.get_y());

        if let Some(point_l) = self.get_point() {
            let mut first_line = LineCr::new(self.thinkness, AxisCr::Horizontal);
            first_line.set_point_xy(point_l.x, point_l.y);
            first_line.draw(ctx);

            let mut col_status = false;
            let mut point_inc: Option<PointIncXY> = None;

            //sum items max for height line_t
            for i in 0..item_ps.get_items() + 1 {
                line_t.set_height_inc(self.get_down_y_table());
            }

            for i in 0..item_ps.get_items() + 1 {
                // add new column title
                if !col_status {
                    //new limit all 0.0
                    let mut col_limit = LimitWHXY::new(0.0, 0.0, 0.0, 0.0);

                    for c in 0..item_ps.get_columns() {
                        if let Some(column) = items.get(c as usize) {
                            let col_width = column.get_limit_wh().unwrap().width;
                            // sum width
                            col_limit.set_width(col_width + col_limit.get_width());
                            col_limit.set_height(self.get_down_y_table() + col_limit.get_height());

                            if c == 0 {
                                point_inc = Some(PointIncXY::new(
                                    col_width,
                                    point_l.x,
                                    self.get_down_y_table(),
                                ));

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
                    //println!("col: {} items: {}", limit.get_width(), i);
                }

                let mut line = LineCr::new(self.thinkness, AxisCr::Horizontal);
                //default line for table
                let mut line_cp = line.clone();
                if i == 0 {
                    //println!("x {} , y {}", self.limit.get_x(), self.limit.get_y());

                    //let point_t = PointXY::new(self.limit.get_x(), self.limit.get_y());
                    //line_cp.set_point(Some(point_t));
                    //line_cp.draw(ctx);

                    if let Some(point_ic) = point_inc.as_ref() {
                        line_cp.set_axis(AxisCr::Vertical);
                        line_cp.set_limit_wh(5.0, point_ic.get_y() + line_t.get_height());

                        // start line vertical
                        //line_cp.draw(ctx);

                        //let point_s = PointXY::new(line_t.get_width(), self.limit.get_y()); //max width
                        line_cp.set_point_xy(line_t.get_width(), point_l.y);
                        //end line vertical
                        line_cp.draw(ctx);

                        for col in items {
                            println!("col limit: x:{}", col.get_limit_wh().unwrap().width);
                            //let point_col = PointXY::new(col.get_limit().get_x(), self.get_limit().get_y()); //max width
                            //line_cp.set_point(Some(point_col));
                            //end line vertical
                            //line_cp.draw(ctx);
                        }
                    }
                }

                if let Some(point) = point_inc.as_mut() {
                    point.set_limit_point_y();
                    line.set_point_xy(point.get_x(), point.get_y());
                }

                line.draw(ctx);
                col_status = true;
            }
        }
    }

    ///iter all column
    /// * `ctx` draw by cairo
    /// * `items`items for columns
    fn run_column_for_items(&self, ctx: Option<&Context>, items: &Vec<ColumnCR>) {
        let mut item_ps = ItemPosition::new(); // save items for columns max items
        //let mut col_count = 0;
        //point table start position (x, y)
        //let point_tb = PointXY::new(self.get_limit().get_x(), self.get_limit().get_y());
        //count width and height sum in all columns
        let mut line_t: LineTable = LineTable::new();
        if let Some(list) = self.items.borrow().as_ref() {
            //println!("table x:{} y: {}", point_tb.x, point_tb.y);
            //println!("*****************");
            for col in list {
                // get limitCR of column
                if let Some(col_item) = col.get_limit_wh() {
                    self.column_item(ctx, col, &mut item_ps);
                    //let col_limit = col.get_limit();
                    // sum width for all items column
                    line_t.set_width_inc(col_item.width);
                }
                //col_count += 1;
            }

            //draw line for table and columns
            self.set_draw_line_for_table(ctx, &mut line_t, &mut item_ps, items);
        }
    }
    */
    /// draw all Column
    pub fn show(&self, ctx: Option<&Context>) {
        // get columns added for table
        if let Some(list) = self.items.borrow().as_ref() {
            // incremet width and height all columns
            let mut wh_inc = LineTable::new();
            // admin position items for draw line in table
            let mut col_position = ColumnPosition::new();
            // font items
            let param_col = ParamArgs::new(
                "Sans".to_string(),
                10.0,
                FontSlant::Normal,
                FontWeight::Normal,
            );
            // add index column for sum width
            let mut col_idex: u32 = 0;
            for col in list {
                col.set_col_postion_id(col_idex);
                //run column add position sum
                col_position.set_columns();
                col_position.set_down_y(self.get_down_y_table());

                if let Some(point_t) = self.get_point().as_ref() {
                    // add point column from table
                    col.set_point_xy(point_t.x, point_t.y);
                }

                col.run_items_draw(ctx, self.get_down_y_table());
                
                // get limitCR and PointXY of column
                //let point_c = col.get_point(); //dont use first set late
                let limit_c = col.get_limit_wh();

                if let Some(col_limit) = limit_c.as_ref() {
                    wh_inc.set_width_inc(col_limit.width);
                    wh_inc.set_height_inc(col_limit.height);
                }

                //positon column in table
                col_idex += 1;
            }
        }

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

#[derive(Clone)]
/// Sum width and height for all columns
struct LineTable {
    width: f64,
    height: f64,
}

impl LineTable {
    /// Create new object
    pub fn new() -> Self {
        LineTable {
            width: 0.0,
            height: 0.0,
        }
    }

    /// get height max for all column
    pub fn get_height(&self) -> f64 {
        self.height
    }

    /// get height max for all column
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
