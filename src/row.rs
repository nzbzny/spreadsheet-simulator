use crate::Cell;

use std::collections::HashMap;

#[derive(Default)]
pub struct Row {
    cells: HashMap<usize, Cell>,
    max_col: usize,
}

impl Row {
    pub fn at(&self, x: usize) -> Option<&Cell> {
        return self.cells.get(&x);
    }

    pub fn insert_at(&mut self, col_idx: usize, c: char) {
        if let Some(cell) = self.cells.get_mut(&col_idx) {
            cell.insert(c);
        } else {
            self.cells.insert(col_idx, Cell::from(c));

            if col_idx > self.max_col {
                self.max_col = col_idx;
            }
        }
    }

    pub fn get_mut(&mut self, x: usize) -> Option<&mut Cell> {
        return self.cells.get_mut(&x);
    }

    pub fn max_col(&self) -> usize {
        self.max_col
    }

    pub fn init_cell_at(&mut self, col_idx: usize, str: String) {
        self.cells.insert(col_idx, Cell::from(str));
        self.max_col = col_idx;
    }

    pub fn clear_cell(&mut self, col_idx: usize) {
        self.cells.remove(&col_idx);
    }

    pub fn insert_column(&mut self, at: usize) {
        for idx in (at..self.max_col.saturating_add(1)).rev() {
            if let Some(cell) = self.cells.remove(&idx) {
                self.cells.insert(idx + 1, cell);
            }
        }

        self.max_col = self.max_col.saturating_add(1);
    }

    pub fn delete_column(&mut self, at: usize) {
        self.cells.remove(&at);
        let mut new_max_col = self.max_col;

        for idx in at.saturating_add(1)..self.max_col.saturating_add(1) {
            if let Some(cell) = self.cells.remove(&idx) {
                self.cells.insert(idx - 1, cell);
                new_max_col = idx - 1;
            }
        }

        self.max_col = new_max_col;
    }
}
