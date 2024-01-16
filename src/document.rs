use crate::Cell;
use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    max_cols: usize,
}

impl Document {
    pub fn row(&self, row_idx: usize) -> Option<&Row> {
        if row_idx < self.rows.len() {
            return Some(&self.rows[row_idx]);
        }

        None
    }

    pub fn insert_at(&mut self, col_idx: usize, row_idx: usize, c: char) {
        if self.rows.len() <= row_idx {
            self.rows.resize_with(row_idx.saturating_add(1), Default::default);
        }
        
        // TODO: this could still be out-of-bounds if row_idx is at the bounds of usize.
        // will need to do something about that eventually
        self.rows[row_idx].insert_at(col_idx, c);
    }

    pub fn at(&self, col_idx: usize, row_idx: usize) -> Option<&Cell> {
        if let Some(row) = self.row(row_idx) {
            return row.at(col_idx);
        }

        None
    }

    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
}
