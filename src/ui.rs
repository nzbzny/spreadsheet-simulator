use crate::Editor;
use crate::editor::Mode;

use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

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
        size.width /= 8;
        size.height /= 8;

        let mut viewbox_row: u16 = 0;
        let mut viewbox_col: u16 = 0;
        let mut row = editor.viewbox_anchor.row;
        let mut col = editor.viewbox_anchor.col;

        while viewbox_row < 8 {
            while viewbox_col < 8 {
                let text = editor.get_text(col as usize, row as usize).clone();
                let rect = Rect {
                    x: size.x + (size.width * viewbox_col),
                    y: size.y + (size.height * viewbox_row),
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

                viewbox_col += 1;
                col += 1;
            }

            col -= 8;
            row += 1;
            viewbox_col = 0;
            viewbox_row += 1;
        }
    }

    fn draw_status_message(frame: &mut Frame, editor: &Editor) {
        let message = if editor.get_mode() == &Mode::Command {
            format!(":{}", &editor.command.to_string())
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

    pub fn draw(frame: &mut Frame, editor: &Editor) {
        draw_spreadsheet(frame, editor);
        draw_status_message(frame, editor);
    }
