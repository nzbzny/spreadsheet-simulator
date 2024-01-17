use crate::Cell;

use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Row {
    cells: HashMap<usize, Cell>,
}

impl Row {
    pub fn at(&self, x: usize) -> Option<&Cell> {
        if x < self.len() {
            return self.cells.get(&x);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn insert_at(&mut self, col_idx: usize, c: char) {
        if let Some(cell) = self.cells.get_mut(&col_idx) {
            cell.insert(c);

            return;
        }

        self.cells.insert(col_idx, Cell::from(c));
    }
}
