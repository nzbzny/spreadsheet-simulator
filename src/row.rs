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

    pub fn insert_at(&mut self, x: usize, c: char) {
        if self.cells.len() <= x {
            self.cells.resize_with(x.saturating_add(1), Default::default);
        }

        // TODO: this could still be out-of-bounds if we're at the bounds of usize.
        // will need to do something about that eventually
        self.cells[x].insert(c);
    }
}
