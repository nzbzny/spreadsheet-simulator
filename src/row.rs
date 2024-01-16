use crate::Cell;

#[derive(Default, Clone)]
pub struct Row {
    cells: Vec<Cell>
}

impl Row {
    pub fn at(&self, x: usize) -> Option<&Cell> {
        if x < self.len() {
            return Some(&self.cells[x]);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn insert_at(&mut self, col_idx: usize, c: char) {
        if self.cells.len() <= col_idx {
            self.cells.resize_with(col_idx.saturating_add(1), Default::default);
        }

        // TODO: this could still be out-of-bounds if col_idx is at the bounds of usize.
        // will need to do something about that eventually
        self.cells[col_idx].insert(c);
    }
}
