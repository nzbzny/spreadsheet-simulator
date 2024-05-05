use crate::constants;
use crate::document::Document;
use crate::parser;

#[derive(Default)] // TODO: implement Copy?
pub struct Cell {
    text: String,
    cursor_position: usize,
    view_start: usize,
    evaluated: String,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        Self {
            text: String::from(c),
            cursor_position: 1,
            view_start: 0,
            evaluated: String::default(),
        }
    }
}

impl From<String> for Cell {
    fn from(text: String) -> Self {
        Self {
            cursor_position: text.len(),
            text,
            view_start: 0,
            evaluated: String::default(),
        }
    }
}

impl Cell {
    pub fn to_str(&self) -> &str {
        if self.evaluated.len() > 0 {
            &self.evaluated
        } else {
            &self.text
        }
    }

    pub fn to_string(&self) -> &String {
        if self.evaluated.len() > 0 {
            &self.evaluated
        } else {
            &self.text
        }
    }

    pub fn view(&self) -> String {
        let text = if self.evaluated.len() > 0 {
            &self.evaluated
        } else {
            &self.text
        };

        let mut end = self.view_start.saturating_add(constants::CELL_VIEW_LEN);
        if end > text.len() {
            end = text.len();
        }

        text.get(self.view_start..end).unwrap().to_string()
    }

    pub fn len(&self) -> usize {
        if self.evaluated.len() > 0 {
            self.evaluated.len()
        } else {
            self.text.len()
        }
    }

    pub fn insert(&mut self, c: char) {
        self.text.insert(self.cursor_position, c);

        self.move_cursor(crossterm::event::KeyCode::Right);
    }

    pub fn move_cursor(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                self.cursor_position = self.cursor_position.saturating_sub(1);
                if self.cursor_position < self.view_start {
                    self.view_start = self.view_start.saturating_sub(1);
                }
            }
            crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                if self.cursor_position < self.len() {
                    self.cursor_position = self.cursor_position.saturating_add(1);

                    if self.cursor_position > self.view_start + constants::CELL_VIEW_LEN {
                        self.view_start = self.view_start.saturating_add(1);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn handle_delete(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Delete => {
                if self.cursor_position < self.len() {
                    self.text.remove(self.cursor_position);
                }
            }
            crossterm::event::KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.text.remove(self.cursor_position - 1);
                    self.cursor_position = self.cursor_position.saturating_sub(1);
                }
            }
            _ => {}
        }
    }

    pub fn set_evaluated(&mut self, evaluated: String) {
        self.evaluated = evaluated;
        self.cursor_position = 0;
        self.view_start = 0;
    }

    pub fn clear_evaluated(&mut self, place_at_end: bool) {
        let was_evaluated = self.evaluated.len() > 0;
        self.evaluated = "".to_string();

        if place_at_end {
            self.cursor_position = self.text.len();
            self.view_start = self.text.len().saturating_sub(constants::CELL_VIEW_LEN);
        } else if was_evaluated {
            self.cursor_position = 1;
            self.view_start = 0;
        }
    }
}
