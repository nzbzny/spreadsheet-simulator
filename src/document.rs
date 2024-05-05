use crate::editor::Position;
use crate::parser;
use crate::Cell;
use crate::Row;

use std::collections::HashMap;
use std::fs::{self, File};
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
                    filename: Some(filename),
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
            filename: Some(filename),
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No filename",
            ));
        }

        let mut doc_string = String::new();

        for row_idx in 0..self.max_row().saturating_add(1) {
            if let Some(row) = self.get_row(row_idx) {
                for col_idx in 0..row.max_col().saturating_add(1) {
                    if let Some(cell) = self.get_cell(col_idx, row_idx) {
                        doc_string.push_str(cell.to_string());
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
            if let Some(row) = self.rows.remove(&row_idx) {
                self.rows.insert(row_idx.saturating_add(1), row);
            }
        }

        self.rows.insert(at, Row::default());
        self.max_row = self.max_row.saturating_add(1);
    }

    pub fn insert_column(&mut self, at: usize) {
        for row in self.rows.values_mut() {
            row.insert_column(at);
        }
    }

    pub fn clear_cell(&mut self, pos: &Position) {
        if let Some(row) = self.rows.get_mut(&pos.row) {
            row.clear_cell(pos.col);
        }
    }

    pub fn delete_row(&mut self, row: usize) {
        self.rows.remove(&row);

        let mut new_max = self.max_row;

        for row_idx in row..self.max_row.saturating_add(1) {
            if let Some(row) = self.rows.remove(&row_idx) {
                self.rows.insert(row_idx.saturating_sub(1), row);
                new_max = row_idx;
            }
        }

        self.max_row = new_max;
    }

    pub fn delete_column(&mut self, at: usize) {
        for row in self.rows.values_mut() {
            row.delete_column(at);
        }
    }

    pub fn evaluate_current_cell(&mut self, pos: &Position) {
        let mut evaluated = "".to_string();
        if let Some(current_cell) = self.get_cell(pos.col, pos.row) {
            let text = current_cell.to_str();
            evaluated = parser::parse(text.strip_prefix("="), self);
        }
        
        if evaluated.is_empty() {
            return;
        }

        // TODO: we know that cell exists. we can force this here
        if let Some(current_mut_cell) = self.get_mut_cell(pos) {
            current_mut_cell.set_evaluated(evaluated)
        }
    }
}
