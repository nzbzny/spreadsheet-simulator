use std::rc::Rc;

use crate::Editor;
use crate::constants;
use crate::editor::Mode;
use crate::editor::Position;
use crate::editor::SearchMode;

use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::Frame;
use ratatui::style::Style;
use ratatui::style::Modifier;
use ratatui::symbols;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;

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

fn create_layouts(frame: &Frame) -> Vec<Rc<[Rect]>> {
    let layout = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
            Constraint::Ratio(1, 10),
    ]).split(frame.size());

    let mut sub_layouts: Vec<Rc<[Rect]>> = vec![];

    for i in 0..layout.len() {
        sub_layouts.push(Layout::default().direction(Direction::Vertical).constraints([
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::default(),
        ]).split(layout[i]))
    }

    return sub_layouts;
}

fn draw_spreadsheet(frame: &mut Frame, editor: &Editor) {
    let mut size = frame.size();
    size.width /= constants::SHEET_VIEWBOX_WIDTH;
    size.height /= constants::SHEET_VIEWBOX_HEIGHT;

    let mut viewbox_row: u16 = 0;
    let mut viewbox_col: u16 = 0;
    let mut row = editor.viewbox_anchor.row;
    let mut col = editor.viewbox_anchor.col;

    let layouts = create_layouts(frame);

    while viewbox_row < constants::SHEET_VIEWBOX_HEIGHT {
        while viewbox_col < constants::SHEET_VIEWBOX_WIDTH {
            let text = editor.view(col, row);

            let current_cell = (row == editor.cursor_position.row) && (col == editor.cursor_position.col);
            let should_highlight = should_highlight_cell(editor, &text, col, row);

            // TODO: the row and the column with the selected cell should have all 4 borders
            // can do this by comparing the current row/col with the selected cell's row/col,
            // if curr_row < selected_row then only do top border, if curr_row > selected_row only
            // do bottom border, if curr_row == selected_row do both. 
            // Similarly, if curr_col < selected_col then only do left border, 
            // if curr_col > selected_col then only do right border, and if curr_col ==
            // selected_col do both borders. this will let you still make the currently selected
            // cell have thickness

            let border_style = Style::new()
                .add_modifier(
                    if should_highlight || current_cell { Modifier::BOLD } else { Modifier::empty() }
                ).fg(
                    if should_highlight || current_cell { 
                        ratatui::style::Color::Rgb(220, 220, 220)
                    } else {
                        ratatui::style::Color::Rgb(100, 100, 100)
                    }
                );

            let border_type = if should_highlight || current_cell {
                BorderType::Thick
            } else {
                BorderType::Plain
            };

            let block = Block::new().borders(Borders::ALL).border_style(border_style).border_type(border_type);

            let widget = Paragraph::new(text).block(block);

            frame.render_widget(widget, layouts[usize::from(viewbox_col)][usize::from(viewbox_row)]);

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

pub fn draw(frame: &mut Frame, editor: &Editor) {
    draw_spreadsheet(frame, editor);
    draw_status_message(frame, editor);
}
