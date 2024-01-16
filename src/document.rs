use crate::Cell;
use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    max_cols: usize,
}

impl Document {
    pub fn row(&self, y: usize) -> Option<&Row> {
        if y < self.rows.len() {
            return Some(&self.rows[y]);
        }

        None
    }

    pub fn insert_at(&mut self, x: usize, y: usize, c: char) {
        if self.rows.len() <= y {
            self.rows.resize_with(y.saturating_add(1), Default::default);
        }
        
        self.rows[y].insert_at(x, c)
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&Cell> {
        if let Some(row) = self.row(y) {
            return row.at(x);
        }

        None
    }

    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
}
