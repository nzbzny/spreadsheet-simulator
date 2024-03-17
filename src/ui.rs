use crate::Editor;
use crate::constants;
use crate::editor::Mode;
use crate::editor::SearchMode;

use crossterm::style::Stylize;
use ratatui::layout::Rect;
use ratatui::Frame;
use ratatui::style::Style;
use ratatui::style::Modifier;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;

use ratatui::widgets::Table;

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

fn draw_spreadsheet(frame: &mut Frame, editor: &Editor) {
    let mut size = frame.size();
    size.width /= constants::SHEET_VIEWBOX_WIDTH;
    size.height /= constants::SHEET_VIEWBOX_HEIGHT;

    let mut viewbox_row: u16 = 0;
    let mut viewbox_col: u16 = 0;
    let mut row = editor.viewbox_anchor.row;
    let mut col = editor.viewbox_anchor.col;

    while viewbox_row < constants::SHEET_VIEWBOX_HEIGHT {
        while viewbox_col < constants::SHEET_VIEWBOX_WIDTH {
            let text = editor.view(col, row);
            
            let rect = Rect {
                x: size.x + (size.width * viewbox_col),
                y: size.y + (size.height * viewbox_row),
                width: size.width,
                height: size.height,
            };

            let should_highlight = should_highlight_cell(editor, &text, col, row);
            let current_cell = (row == editor.cursor_position.row) && (col == editor.cursor_position.col);

            let border_type = if should_highlight || current_cell {
                ratatui::widgets::BorderType::Thick
            } else {
                ratatui::widgets::BorderType::Plain
            };

            let border_style = Style::new()
                .add_modifier(
                    if should_highlight || current_cell { Modifier::BOLD } else { Modifier::empty() }
                ).fg(
                    if should_highlight { ratatui::style::Color::Blue } else { ratatui::style::Color::White }
                );

            let block = Block::new().borders(Borders::ALL).border_type(border_type).border_style(border_style);

            let widget = Paragraph::new(text).block(block);

            frame.render_widget(widget, rect);

            viewbox_col += 1;
            col += 1;
        }

        col -= usize::from(constants::SHEET_VIEWBOX_WIDTH);
        row += 1;
        viewbox_col = 0;
        viewbox_row += 1;
    }
}

fn draw_status_message(frame: &mut Frame, editor: &Editor) {
    let message = if editor.get_mode() == &Mode::Command {
        format!(":{}", &editor.command.to_string())
    } else if editor.get_mode() == &Mode::Search && editor.search_mode != SearchMode::Error {
        format!("/{}", &editor.search_text.to_string())
    } else {
        editor.status_message.text.clone()
    };
    
    let widget = Paragraph::new(message.clone());

    let size = frame.size();
    let rect = Rect {
        x: size.x,
        y: size.y + size.height - 1,
        width: size.width,
        height: 1,
    };

    frame.render_widget(widget, rect);
}

fn should_highlight_cell(editor: &Editor, text: &str, col: usize, row: usize) -> bool {
    if editor.search_mode == SearchMode::None || editor.search_mode == SearchMode::Error {
        return false;
    }

    if editor.search_mode == SearchMode::Row && row != editor.cursor_position.row {
        return false;
    }

    if editor.search_mode == SearchMode::Column && col != editor.cursor_position.col {
        return false;
    }

    if text.contains(editor.search_text.to_string()) {
        return true;
    }

    false
}

fn draw_table(frame: &mut Frame, editor: &Editor) {
    let mut rows: Vec<ratatui::widgets::Row> = vec![];
    for row_idx in 0..8 {
        let mut row_vec: Vec<String> = vec![];
        for col_idx in 0..8 { 
            row_vec.push(editor.view(col_idx, row_idx));
        }

        rows.push(ratatui::widgets::Row::new(row_vec));
    }


    let widths = [
        ratatui::prelude::Constraint::Length(constants::CELL_VIEW_LEN.try_into().unwrap()),
        ratatui::prelude::Constraint::Length(constants::CELL_VIEW_LEN.try_into().unwrap()),
        ratatui::prelude::Constraint::Length(constants::CELL_VIEW_LEN.try_into().unwrap()),
        ratatui::prelude::Constraint::Length(constants::CELL_VIEW_LEN.try_into().unwrap()),
        ratatui::prelude::Constraint::Length(constants::CELL_VIEW_LEN.try_into().unwrap()),
        ratatui::prelude::Constraint::Length(constants::CELL_VIEW_LEN.try_into().unwrap()),
        ratatui::prelude::Constraint::Length(constants::CELL_VIEW_LEN.try_into().unwrap()),
        ratatui::prelude::Constraint::Length(constants::CELL_VIEW_LEN.try_into().unwrap()),
    ];

    let size = frame.size();
    let table = ratatui::widgets::Table::default().rows(rows).widths(widths).column_spacing(1);
    frame.render_widget(table, size);
}

pub fn draw(frame: &mut Frame, editor: &Editor) {
    draw_table(frame, editor);
//    draw_spreadsheet(frame, editor);
    draw_status_message(frame, editor);
}
