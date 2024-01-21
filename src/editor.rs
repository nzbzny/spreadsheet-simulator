use crate::document::Document;

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

#[derive(Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Editor {
    mode: EditorMode,
    should_quit: bool,
    cursor_position: Position,
    document: Document,
    command: String,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            mode: EditorMode::Normal,
            should_quit: false,
            cursor_position: Position::default(),
            document: Document::default(),
            command: String::from(""),
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

        let mut row: u16 = 0;
        let mut col: u16 = 0; // should start based on viewbox

        while row < 8 {
            while col < 8 {
                let text = editor.get_text(col as usize, row as usize);
                let rect = Rect {
                    x: size.x + (size.width * col),
                    y: size.y + (size.height * row),
                    width: size.width,
                    height: size.height,
                };

                let mut block = Block::new().borders(Borders::ALL);

                if (row as usize) == editor.cursor_position.row
                    && (col as usize) == editor.cursor_position.col
                {
                    block = block
                        .border_type(ratatui::widgets::BorderType::Thick)
                        .border_style(
                            ratatui::style::Style::new()
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        );
                }

                let widget = Paragraph::new(text).block(block);

                frame.render_widget(widget, rect);

                col += 1;
            }

            col = 0;
            row += 1;
        }
    }

    pub fn run(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), std::io::Error> {
        loop {
            let _ = terminal.draw(|frame| Editor::draw(frame, &self));

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
                self.mode = EditorMode::Command
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
            }
            _ => {}
        }
    }

    fn handle_command_mode_press(&mut self, key: crossterm::event::KeyCode) {
        match key {
            crossterm::event::KeyCode::Esc => {
                self.command = String::from("");
                self.mode = EditorMode::Normal;
            }
            crossterm::event::KeyCode::Char(c) => self.command.push(c),
            crossterm::event::KeyCode::Enter => self.execute_command(),
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        match self.command.as_str() {
            "q" => {
                self.should_quit = true;
            }
            "w" => {
                self.save();
            }
            _ => {}
        }
        self.command = String::from("");
        self.mode = EditorMode::Normal;
    }

    fn save(&self) {
        println!("saving file");
    }

    fn get_text(&self, col: usize, row: usize) -> String {
        if let Some(cell) = self.document.get_cell(col, row) {
            cell.text()
        } else {
            "".to_string()
        }
    }
}
