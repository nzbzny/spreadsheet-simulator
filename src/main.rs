#![warn(clippy::all, clippy::pedantic)]

mod cell;
mod document;
mod editor;
mod row;
mod ui;

use cell::Cell;
use editor::Editor;
use row::Row;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use std::io::stdout;

fn main() -> Result<(), std::io::Error> {
    let mut editor = Editor::default();

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let _ = editor.run(terminal);

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
