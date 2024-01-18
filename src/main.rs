#![warn(clippy::all, clippy::pedantic)]

mod cell;
mod document;
mod editor;
mod row;

use cell::Cell;
use editor::Editor;
use row::Row;

fn main() -> Result<(), std::io::Error> {
    let mut editor = Editor::default();

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let _ = editor.run();

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
