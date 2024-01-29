use crate::Cell;

use std::collections::HashMap;

#[derive(Default, Clone)]
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
}
