use crate::Cell;
use crate::editor::Position;
use crate::Row;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Default)]
pub struct Document {
    rows: HashMap<usize, Row>,
    max_row: usize,
    pub filename: Option<String>,
}

impl Document {
    pub fn from(filename: String) -> Self {
        Self {
            rows: HashMap::new(),
            max_row: 0,
            filename: Some(filename)
        }
    }

    pub fn get_row(&self, row_idx: usize) -> Option<&Row> {
        return self.rows.get(&row_idx);
    }

    pub fn insert_at(&mut self, position: &Position, c: char) {
        if let Some(row) = self.rows.get_mut(&position.row) {
            row.insert_at(position.col, c);
        } else {
            let mut row = Row::default();
            row.insert_at(position.col, c);

            self.rows.insert(position.row, row);

            if position.row > self.max_row {
                self.max_row = position.row;
            }
        }
    }

    pub fn get_cell(&self, col_idx: usize, row_idx: usize) -> Option<&Cell> {
        if let Some(row) = self.get_row(row_idx) {
            return row.at(col_idx);
        }

        None
    }

    pub fn get_mut_cell(&mut self, position: &Position) -> Option<&mut Cell> {
        if let Some(row) = self.rows.get_mut(&position.row) {
            return row.get_mut(position.col);
        }
        
        None
    }

    pub fn max_row(&self) -> usize {
        self.max_row
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        let mut doc_string = String::from("");

        for row_idx in 0..self.max_row().saturating_add(1) {
            if let Some(row) = self.get_row(row_idx) {
                for col_idx in 0..row.max_col().saturating_add(1) {
                    if let Some(cell) = self.get_cell(col_idx, row_idx) {
                        doc_string.push_str(&cell.text())
                    }
                    doc_string.push(',');
                }
            }

            doc_string.push('\n');
        }

        let mut file = File::create(self.filename.as_ref().unwrap())?;

        file.write_all(doc_string.as_bytes())?;

        Ok(())
    }
}
