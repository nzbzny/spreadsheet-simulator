use crate::Cell;
use crate::document::Document;
use crate::ui;

use std::env;
use std::time::{Duration, Instant};
use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[derive(PartialEq, Clone)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    SaveAs,
    Delete,
}

#[derive(Default, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct StatusMessage {
    pub text: String,
    time: Instant,
}

impl From<&str> for StatusMessage {
    fn from(message: &str) -> Self {
        Self::from(String::from(message))
    }
}

impl From<String> for StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

impl StatusMessage {
    pub fn empty() -> Self {
        StatusMessage::from("")
    }
}

pub struct Editor {
    mode: Mode,
    should_quit: bool,
    pub cursor_position: Position,
    document: Document,
    pub command: Cell,
    pub status_message: StatusMessage,
    pub viewbox_anchor: Position,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = StatusMessage::empty();
        
        let document = if let Some(filename) = args.get(1) {
            let doc = Document::from(filename.to_string());

            match doc {
                Ok(doc) => {
                    doc
                },
                Err(err) => {
                    initial_status = StatusMessage::from(err.to_string());
                    Document::default()
                }
            }
        } else {
            Document::default()
        };

        Self {
            mode: Mode::Normal,
            should_quit: false,
            cursor_position: Position::default(),
            document,
            command: Cell::default(),
            status_message: initial_status,
            viewbox_anchor: Position::default(),
        }
    }

    pub fn run(
        &mut self,
    ) -> Result<(), std::io::Error> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        loop {
            let _ = terminal.draw(|frame| ui::draw(frame, self));

            if crossterm::event::poll(std::time::Duration::from_millis(250))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        match self.mode {
                            Mode::Insert => self.handle_insert_mode_press(key.code),
                            Mode::Normal => self.handle_normal_mode_press(key.code),
                            Mode::Command => self.handle_command_mode_press(key.code),
                            Mode::SaveAs => self.handle_save_as_mode_press(key.code),
                            Mode::Delete => self.handle_delete_mode_press(key.code),
                        }
                    }
                }

                if self.should_quit {
                    break;
                }

                if self.status_message.time.elapsed() > Duration::new(5, 0) {
                    self.status_message = StatusMessage::empty();
                }
            }
        }

        Ok(())
    }

    fn move_viewbox(&mut self) {
        if self.cursor_position.row < self.viewbox_anchor.row { // move viewbox up
            self.viewbox_anchor.row = self.cursor_position.row;
        }

        if self.cursor_position.col < self.viewbox_anchor.col { // move viewbox left
            self.viewbox_anchor.col = self.cursor_position.col;
        }

        // TODO: unhardcode this
        let viewbox_height = 8;
        let viewbox_width = 8;

        if self.viewbox_anchor.row.saturating_add(viewbox_height) <= self.cursor_position.row { // move viewbox down
            self.viewbox_anchor.row = self.cursor_position.row.saturating_sub(viewbox_height - 1);
        }

        if self.viewbox_anchor.col.saturating_add(viewbox_height) <= self.cursor_position.col { // move viewbox right
            self.viewbox_anchor.col = self.cursor_position.col.saturating_sub(viewbox_width - 1);
        }
    }

    fn handle_normal_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                self.cursor_position.row = self.cursor_position.row.saturating_add(1);
                self.move_viewbox();
            },
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                self.cursor_position.row = self.cursor_position.row.saturating_sub(1);
                self.move_viewbox();
            },
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                self.cursor_position.col = self.cursor_position.col.saturating_sub(1);
                self.move_viewbox();
            },
            crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                self.cursor_position.col = self.cursor_position.col.saturating_add(1);
                self.move_viewbox();
            },
            crossterm::event::KeyCode::Char('i') => self.mode = Mode::Insert,
            crossterm::event::KeyCode::Char(':') => self.mode = Mode::Command,
            crossterm::event::KeyCode::Char('d') => self.mode = Mode::Delete,
            crossterm::event::KeyCode::Char('o') => {
                self.command = Cell::from("irb".to_string());
                self.execute_command();
                self.handle_normal_mode_press(crossterm::event::KeyCode::Char('j'));
                self.mode = Mode::Insert;
            },
            crossterm::event::KeyCode::Char('O') => {
                self.command = Cell::from("ira".to_string());
                self.execute_command();
                self.mode = Mode::Insert;
            },
            crossterm::event::KeyCode::Esc => self.mode = Mode::Normal,
            _ => {}
        }
    }

    fn handle_insert_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Esc => self.mode = Mode::Normal,
            crossterm::event::KeyCode::Char(c) => {
                self.document.insert_at(&self.cursor_position, c);
            },
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Right => {
                if let Some(cell) = self.document.get_mut_cell(&self.cursor_position) {
                    cell.move_cursor(key);
                }
            },
            crossterm::event::KeyCode::Backspace | crossterm::event::KeyCode::Delete => {
                if let Some(cell) = self.document.get_mut_cell(&self.cursor_position) {
                    cell.handle_delete(key);
                }
            }
            _ => {}
        }
    }

    fn handle_command_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Esc => {
                self.command = Cell::default();
                self.mode = Mode::Normal;
            }
            crossterm::event::KeyCode::Char(c) => self.command.insert(c),
            crossterm::event::KeyCode::Enter => self.execute_command(),
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Right => {
                self.command.move_cursor(key);
            },
            crossterm::event::KeyCode::Delete | crossterm::event::KeyCode::Backspace => {
                self.command.handle_delete(key);
            },
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        match self.command.to_str() {
            "q" => {
                self.should_quit = true;
            }
            "w" => {
                let result = self.save();
                
                match result {
                    Ok(()) => {
                        self.status_message = StatusMessage::from("Success");
                    },
                    Err(err) => {
                        if err.kind() == std::io::ErrorKind::Other {
                            self.command = Cell::default();
                            return;
                        } 

                        self.status_message = StatusMessage::from(err.to_string());
                    }
                };
            }
            "ira" => {
                self.document.insert_row(self.cursor_position.row);
            },
            "irb" => {
                self.document.insert_row(self.cursor_position.row.saturating_add(1));
            }
            _ => {
                self.status_message = StatusMessage::from(
                    format!("Unrecognized command: {}", self.command.to_string())
                );
            } 
        }

        self.command = Cell::default();
        self.mode = Mode::Normal;
    }

    fn handle_delete_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Char(' ') => self.document.delete_cell(&self.cursor_position),
            crossterm::event::KeyCode::Char('r') => self.document.delete_row(self.cursor_position.row),
            crossterm::event::KeyCode::Esc => self.mode = Mode::Normal,
            _ => self.status_message = StatusMessage::from("Unrecognized command"),
        }

        self.mode = Mode::Normal;
    }

    fn handle_save_as_mode_press(&mut self, key: crossterm::event::KeyCode) {
        let mut filename = if let Some(filename) = &self.document.filename {
            String::from(filename)
        } else {
            String::new()
        };

        match key {
            crossterm::event::KeyCode::Char(c) => {
                if !c.is_control() {
                    filename.push(c);
                }
            },
            crossterm::event::KeyCode::Backspace => {
                filename.truncate(filename.len().saturating_sub(1));
            },
            crossterm::event::KeyCode::Esc => {
                self.document.filename = None;
                self.mode = Mode::Normal;
                self.status_message = StatusMessage::from("Save aborted");

                return;
            },
            crossterm::event::KeyCode::Enter => {
                self.mode = Mode::Normal;

                match self.save() {
                    Ok(()) => {
                        self.status_message = StatusMessage::from("Success");
                    },
                    Err(err) => {
                        self.status_message = StatusMessage::from(err.to_string());
                    }
                };

                return;
            },
            _ => {}
        }
        
        self.status_message = StatusMessage::from(format!("Save as: {filename}"));
        
        if !filename.is_empty() {
            self.document.filename = Some(filename);
        }
    }

    fn save(&mut self) -> std::io::Result<()> {
        if self.document.filename.is_none() {
            self.mode = Mode::SaveAs;
            self.status_message = StatusMessage::from("Save as: ");
        }

        self.document.save()
    }

    pub fn get_text(&self, col: usize, row: usize) -> String {
        if let Some(cell) = self.document.get_cell(col, row) {
            cell.text()
        } else {
            String::new()
        }
    }

    pub fn get_mode(&self) -> Mode {
        self.mode.clone()
    }
}
