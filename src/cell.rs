#[derive(Default)] // TODO: implement Copy?
pub struct Cell {
    text: String,
    cursor_position: usize,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        Self {
            text: String::from(c),
            cursor_position: 1 // cell starts with 1 char
        }
    }
}

impl From<String> for Cell {
    fn from(text: String) -> Self {
        Self {
            cursor_position: text.len(),
            text,
        }
    }
}

impl Cell {
    pub fn to_str(&self) -> &str {
        &self.text
    }

    pub fn to_string(&self) -> &String {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn insert(&mut self, c: char) {
        self.text.insert(self.cursor_position, c);

        self.move_cursor(crossterm::event::KeyCode::Right);
    }

    pub fn move_cursor(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Left => {
                self.cursor_position = self.cursor_position.saturating_sub(1);
            },
            crossterm::event::KeyCode::Right => {
                if self.cursor_position < self.len() {
                    self.cursor_position = self.cursor_position.saturating_add(1);
                }
            },
            _ => {}
        }
    }

    pub fn handle_delete(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Delete => {
                if self.cursor_position < self.len() {
                    self.text.remove(self.cursor_position);
                }
            },
            crossterm::event::KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.text.remove(self.cursor_position - 1);
                    self.cursor_position = self.cursor_position.saturating_sub(1);
                }
            },
            _ => {}
        }
    }
}
