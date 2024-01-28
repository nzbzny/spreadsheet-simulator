use crate::Cell;
use crate::document::Document;
use crate::ui;

use std::io::Stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui::Terminal;

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
}

#[derive(Default, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Editor {
    mode: EditorMode,
    should_quit: bool,
    pub cursor_position: Position,
    document: Document,
    pub command: Cell,
    pub status_message: String,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            mode: EditorMode::Normal,
            should_quit: false,
            cursor_position: Position::default(),
            document: Document::default(),
            command: Cell::default(),
            status_message: String::from(""),
        }
    }

    pub fn run(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), std::io::Error> {
        loop {
            let _ = terminal.draw(|frame| ui::draw(frame, &self));

            if crossterm::event::poll(std::time::Duration::from_millis(250))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        match self.mode {
                            EditorMode::Insert => self.handle_insert_mode_press(key.code),
                            EditorMode::Normal => self.handle_normal_mode_press(key.code),
                            EditorMode::Command => self.handle_command_mode_press(key.code),
                        }
                    }
                }

                if self.should_quit {
                    break;
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
                self.command.insert(':') ;
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
                    .insert_at(self.cursor_position.col, self.cursor_position.row, c)
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
            ":q" => {
                self.should_quit = true;
            }
            ":w" => {
                self.save();
            }
            _ => {} // TODO: unrecognized command status message
        }
        self.command = Cell::default();
        self.mode = EditorMode::Normal;
    }

    fn save(&self) {
        println!("saving file");
    }

    pub fn get_text(&self, col: usize, row: usize) -> String {
        if let Some(cell) = self.document.get_cell(col, row) {
            cell.text()
        } else {
            "".to_string()
        }
    }
}
