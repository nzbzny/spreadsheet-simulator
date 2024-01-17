#![warn(clippy::all, clippy::pedantic)]

mod cell;
mod document;
mod row;

use cell::Cell;
use document::Document;
use row::Row;

use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::widgets::Borders;
use ratatui::widgets::BorderType;
use ratatui::widgets::Block;
use ratatui::Terminal;

fn main() -> Result<(), std::io::Error> {
    let mut document = Document::default();

    document.insert_at(0, 0, 'c');
    document.insert_at(0, 0, 'h');
    document.insert_at(0, 0, 'a');
    document.insert_at(0, 0, 'r');

    println!("{}", document.get_cell(0, 0).unwrap().to_str());

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let _ = terminal.draw(|frame| {
        let border_type = BorderType::Rounded;
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(border_type)
            .title(format!("BorderType::{border_type:#?}"));
        let mut size = frame.size();
        size.width /= 8;
        size.height /= 8;

        frame.render_widget(block, size);
    });

    Ok(())
}
