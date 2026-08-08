pub struct ColumnPosition {
    idex: u32,
    col: u32,
    down_y: f64,
}

impl ColumnPosition {
    pub fn new() -> Self {
        ColumnPosition {
            idex: 0,
            col: 0,
            down_y: 0.0,
        }
    }

    pub fn set_down_y(&mut self, y: f64){
        self.down_y = y;
    }

    /// add column count
    pub fn set_columns(&mut self){
        self.col += 1;
    }

    /// get columns count
    pub fn get_columns(&self) -> u32{
        self.col
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

use crate::model::LimitWHXY;

#[derive(Clone, Copy)]
/// Save posion column for iter
pub struct ItemPosition {
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
