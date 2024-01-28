use crate::Cell;
use crate::document::Document;
use crate::ui;

use std::env;
use std::time::{Duration, Instant};
use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[derive(PartialEq, Clone)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
    SaveAs,
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

impl StatusMessage {
    fn from(message: &str) -> Self {
        Self::from_string(String::from(message))
    }

    fn from_string(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

pub struct Editor {
    mode: EditorMode,
    should_quit: bool,
    pub cursor_position: Position,
    document: Document,
    pub command: Cell,
    pub status_message: StatusMessage,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        
        let document = if let Some(filename) = args.get(1) {
            Document::from(filename.to_string())

            // TODO: read from file
        } else {
            Document::default()
        };

        Self {
            mode: EditorMode::Normal,
            should_quit: false,
            cursor_position: Position::default(),
            document,
            command: Cell::default(),
            status_message: StatusMessage::from(""),
        }
    }

    pub fn run(
        &mut self,
    ) -> Result<(), std::io::Error> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        loop {
            let _ = terminal.draw(|frame| ui::draw(frame, &self));

            if crossterm::event::poll(std::time::Duration::from_millis(250))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        match self.mode {
                            EditorMode::Insert => self.handle_insert_mode_press(key.code),
                            EditorMode::Normal => self.handle_normal_mode_press(key.code),
                            EditorMode::Command => self.handle_command_mode_press(key.code),
                            EditorMode::SaveAs => self.handle_save_as_mode_press(key.code),
                        }
                    }
                }

                if self.should_quit {
                    break;
                }

                if self.status_message.time.elapsed() > Duration::new(5, 0) {
                    self.status_message = StatusMessage::from("");
                }
            }
        }

        Ok(())
    }

    fn handle_normal_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Char('i') => self.mode = EditorMode::Insert,
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                self.cursor_position.row = self.cursor_position.row.saturating_add(1)
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                self.cursor_position.row = self.cursor_position.row.saturating_sub(1)
            }
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                self.cursor_position.col = self.cursor_position.col.saturating_sub(1)
            }
            crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                self.cursor_position.col = self.cursor_position.col.saturating_add(1)
            }
            crossterm::event::KeyCode::Char(':') => {
                self.mode = EditorMode::Command;
            }
            crossterm::event::KeyCode::Esc => self.mode = EditorMode::Normal,
            _ => {}
        }
    }

    fn handle_insert_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Esc => self.mode = EditorMode::Normal,
            crossterm::event::KeyCode::Char(c) => {
                self.document
                    .insert_at(&self.cursor_position, c)
            },
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Right => {
                if let Some(cell) = self.document.get_mut_cell(&self.cursor_position) {
                    cell.move_cursor(key);
                }
            },
            crossterm::event::KeyCode::Backspace | crossterm::event::KeyCode::Delete => {
                if let Some(cell) = self.document.get_mut_cell(&self.cursor_position) {
                    cell.handle_delete(key)
                }
            }
            _ => {}
        }
    }

    fn handle_command_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Esc => {
                self.command = Cell::default();
                self.mode = EditorMode::Normal;
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
                    Ok(_) => {},
                    Err(err) => {
                        self.status_message = StatusMessage::from(&err.to_string());
                    }
                };

                if self.mode == EditorMode::SaveAs {
                    self.command = Cell::default();
                    return;
                }
            }
            _ => {
                self.status_message = StatusMessage::from_string(
                    format!("Unrecognized command: {}", self.command.to_string())
                );
            } 
        }

        self.command = Cell::default();
        self.mode = EditorMode::Normal;
    }

    fn handle_save_as_mode_press(&mut self, key: crossterm::event::KeyCode) {
        let mut filename = if let Some(filename) = &self.document.filename {
            String::from(filename)
        } else {
            "".to_string()
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
                self.mode = EditorMode::Normal;
                self.status_message = StatusMessage::from("Save aborted");

                return;
            },
            crossterm::event::KeyCode::Enter => {
                self.mode = EditorMode::Normal;

                match self.save() {
                    Ok(()) => {
                        self.status_message = StatusMessage::from("Success");
                    },
                    Err(err) => {
                        self.status_message = StatusMessage::from(&err.to_string());
                    }
                };

                return;
            },
            _ => {}
        }
        
        self.status_message = StatusMessage::from_string(format!("Save as: {}", filename));
        self.document.filename = Some(filename);
    }

    fn save(&mut self) -> std::io::Result<()> {
        if self.document.filename.is_none() {
            self.mode = EditorMode::SaveAs;
            self.status_message = StatusMessage::from("Save as: ");

            return Ok(());
        }

        self.document.save()
    }

    pub fn get_text(&self, col: usize, row: usize) -> String {
        if let Some(cell) = self.document.get_cell(col, row) {
            cell.text()
        } else {
            "".to_string()
        }
    }

    pub fn get_mode(&self) -> EditorMode {
        self.mode.clone()
    }
}
