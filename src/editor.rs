use crate::document::Document;

use std::io::Stdout;
use std::io::stdout;

use ratatui::Frame;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::Borders;
use ratatui::widgets::BorderType;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Insert
}

#[derive(Default)]
pub struct Position {
    col: usize,
    row: usize
}

pub struct Editor {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    mode: EditorMode,
    should_quit: bool,
    cursor_position: Position,
    document: Document
}

impl Editor {
    pub fn default() -> Self {
        Self {
           terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
           mode: EditorMode::Normal,
           should_quit: false,
           cursor_position: Position::default(),
           document: Document::default()
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            let text = self.get_text(self.cursor_position.col, self.cursor_position.row);

            let _ = self.terminal.draw(|frame| {
                let paragraph = Paragraph::new(text).block(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded));
                let mut size = frame.size();
                size.width /= 8;
                size.height /= 8;

                frame.render_widget(paragraph, size);
            });

            if crossterm::event::poll(std::time::Duration::from_millis(250))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        match self.mode {
                            EditorMode::Insert => self.handle_insert_mode_press(key.code),
                            EditorMode::Normal => self.handle_normal_mode_press(key.code)
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
            crossterm::event::KeyCode::Char('q') => {
                self.should_quit = true
            },
            crossterm::event::KeyCode::Char('i') => {
                self.mode = EditorMode::Insert
            }
            crossterm::event::KeyCode::Esc => self.mode = EditorMode::Normal,
            _ => {}
        }
    }

    fn handle_insert_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Esc => self.mode = EditorMode::Normal,
            crossterm::event::KeyCode::Char(c) => {
                self.document.insert_at(self.cursor_position.col, self.cursor_position.row, c)
            }
            _ => {}
        }
    }

    fn get_text(&self, col: usize, row: usize) -> String {
        if let Some(cell) = self.document.get_cell(col, row) {
            cell.text()
        } else {
            "".to_string()
        }
    }
}
