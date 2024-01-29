use crate::Cell;
use crate::editor::Position;
use crate::Row;

use std::collections::HashMap;
use std::fs::{File, self};
use std::io::Write;

#[derive(Default)]
pub struct Document {
    rows: HashMap<usize, Row>,
    max_row: usize,
    pub filename: Option<String>,
}

impl Document {
    pub fn from(filename: String) -> std::io::Result<Self> {
        let contents_r = fs::read_to_string(filename.clone());

        if let Err(err) = contents_r {
            if err.kind() == std::io::ErrorKind::NotFound {
                return Ok(Self {
                    rows: HashMap::new(),
                    max_row: 0,
                    filename: Some(filename)
                });
            }

            return Err(err);
        }

        let contents = contents_r.unwrap();
        let mut rows: HashMap<usize, Row> = HashMap::new();
        let mut max_row: usize = 0;

        for (row_idx, line) in contents.split('\n').enumerate() {
            if line.is_empty() {
                continue;
            }

            let mut current_row = Row::default();

            for (col_idx, text) in line.split(',').enumerate() {
                if !text.is_empty() {
                    current_row.init_cell_at(col_idx, text.to_string());
                }
            }

            rows.insert(row_idx, current_row);
            max_row = row_idx;
        }

        Ok(Self {
            rows,
            max_row,
            filename: Some(filename)
        })
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
        if self.filename.is_none() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "No filename"));
        }

        let mut doc_string = String::new();

        for row_idx in 0..self.max_row().saturating_add(1) {
            if let Some(row) = self.get_row(row_idx) {
                for col_idx in 0..row.max_col().saturating_add(1) {
                    if let Some(cell) = self.get_cell(col_idx, row_idx) {
                        doc_string.push_str(&cell.text());
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

    pub fn insert_row(&mut self, at: usize) {
        for row_idx in (at..self.max_row.saturating_add(1)).rev() {
            if let Some(row) = self.rows.get(&row_idx) {
                self.rows.insert(row_idx + 1, row.clone());
            }
        }

        self.rows.insert(at, Row::default());
        self.max_row = self.max_row.saturating_add(1);
    }
}
