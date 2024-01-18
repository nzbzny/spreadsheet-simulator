use crate::document::Document;

use std::io::Stdout;

use ratatui::Frame;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::widgets::Borders;
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
    pub col: usize,
    pub row: usize
}

pub struct Editor {
    mode: EditorMode,
    should_quit: bool,
    pub cursor_position: Position,
    document: Document
}

impl Editor {
    pub fn default() -> Self {
        Self {
           mode: EditorMode::Normal,
           should_quit: false,
           cursor_position: Position::default(),
           document: Document::default()
        }
    }
/*
pub struct Rect {
    /// The x coordinate of the top left corner of the rect.
    pub x: u16,
    /// The y coordinate of the top left corner of the rect.
    pub y: u16,
    /// The width of the rect.
    pub width: u16,
    /// The height of the rect.
    pub height: u16,
}
*/

    fn draw(frame: &mut Frame, editor: &Editor) {
        let mut size = frame.size();
        size.width /= 8;
        size.height /= 8;

        let mut i: u16 = 0;
        let mut j: u16 = 0;

        while i < 8 {
            while j < 8 {
                let text = editor.get_text(/*editor.cursor_position.row + */(j as usize), /*editor.cursor_position.col + */(i as usize));
                let paragraph = Paragraph::new(text).block(Block::new().borders(Borders::ALL));
                let rect = Rect {
                    x: size.x + (size.width * j),
                    y: size.y + (size.height * i),
                    width: size.width,
                    height: size.height
                };

                frame.render_widget(paragraph, rect);
                
                j += 1;
            }
            
            j = 0;
            i += 1;
        }
    }

    pub fn run(&mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), std::io::Error> {
        loop {
            let _ = terminal.draw(|frame| { Editor::draw(frame, &self)});

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
            crossterm::event::KeyCode::Char('q') => self.should_quit = true,
            crossterm::event::KeyCode::Char('i') => self.mode = EditorMode::Insert,
            crossterm::event::KeyCode::Down => self.cursor_position.row = self.cursor_position.row.saturating_add(1),
            crossterm::event::KeyCode::Up => self.cursor_position.row = self.cursor_position.row.saturating_sub(1),
            crossterm::event::KeyCode::Left => self.cursor_position.col = self.cursor_position.col.saturating_sub(1),
            crossterm::event::KeyCode::Right => self.cursor_position.col = self.cursor_position.col.saturating_add(1),
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
