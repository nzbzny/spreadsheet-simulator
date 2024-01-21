use crate::Cell;
use crate::Row;

use std::collections::HashMap;

#[derive(Default)]
pub struct Document {
    rows: HashMap<usize, Row>,
}

impl Document {
    pub fn get_row(&self, row_idx: usize) -> Option<&Row> {
        return self.rows.get(&row_idx);
    }

    pub fn insert_at(&mut self, col_idx: usize, row_idx: usize, c: char) {
        if let Some(row) = self.rows.get_mut(&row_idx) {
            row.insert_at(col_idx, c);
        } else {
            let mut row = Row::default();
            row.insert_at(col_idx, c);

            self.rows.insert(row_idx, row);
        }
    }

    pub fn get_cell(&self, col_idx: usize, row_idx: usize) -> Option<&Cell> {
        if let Some(row) = self.get_row(row_idx) {
            return row.at(col_idx);
        }

        None
    }
}
