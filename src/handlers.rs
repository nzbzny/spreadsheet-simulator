use crate::cell::Cell;
use crate::editor::Mode;
use crate::editor::{Editor, SearchMode, StatusMessage};

pub fn handle_normal_mode_press(editor: &mut Editor, key: crossterm::event::KeyCode, mods: crossterm::event::KeyModifiers) {
    match key {
        crossterm::event::KeyCode::Left
        | crossterm::event::KeyCode::Right
        | crossterm::event::KeyCode::Char('h' | 'l') => {
            // TODO: use ctrl+(left|right|h|l) to move the cursor of the display text
            // debating if i actually want to do it this way, or if i want to introduce a new mode
            // for moving the display text cursor. kind of prefer this way given that it's an
            // explicit distinction of behavior, because display text is _not_ editable by users
            if mods.contains(crossterm::event::KeyModifiers::CONTROL) {
                if let Some(cell) = editor.document.get_mut_cell(&editor.cursor_position) {
                    cell.move_cursor(key);
                } else {
                    editor.move_cursor(key);
                }
            } else {
                editor.move_cursor(key);
            }
        }

        crossterm::event::KeyCode::Down
        | crossterm::event::KeyCode::Up
        | crossterm::event::KeyCode::Char('j' | 'k') => {
            editor.move_cursor(key);
        }
        crossterm::event::KeyCode::Char('i') => {
            editor.mode = Mode::Insert;

            if let Some(current_cell) = editor.document.get_mut_cell(&editor.cursor_position) {
                current_cell.clear_display_text()
            }
        }
        crossterm::event::KeyCode::Char(':') => editor.mode = Mode::Command,
        crossterm::event::KeyCode::Char('d') => editor.mode = Mode::Delete,
        crossterm::event::KeyCode::Char('o') => {
            editor.command = Cell::from("irb".to_string());
            editor.execute_command();
            handle_normal_mode_press(editor, crossterm::event::KeyCode::Char('j'), crossterm::event::KeyModifiers::empty());
            editor.mode = Mode::Insert;
        }
        crossterm::event::KeyCode::Char('O') => {
            editor.command = Cell::from("ira".to_string());
            editor.execute_command();
            editor.mode = Mode::Insert;
        }
        crossterm::event::KeyCode::Char('/') => editor.mode = Mode::Search,
        crossterm::event::KeyCode::Esc => editor.mode = Mode::Normal,
        _ => {}
    }
}

pub fn handle_insert_mode_press(editor: &mut Editor, key: crossterm::event::KeyCode) {
    match key {
        crossterm::event::KeyCode::Esc => {
            editor.mode = Mode::Normal;

            if let Some(current_cell) = editor.document.get_mut_cell(&editor.cursor_position) {
                current_cell.evaluate_cell()
            }
        }
        crossterm::event::KeyCode::Char(c) => {
            editor.document.insert_at(&editor.cursor_position, c);
        }
        crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Right => {
            if let Some(cell) = editor.document.get_mut_cell(&editor.cursor_position) {
                cell.move_cursor(key);
            }
        }
        crossterm::event::KeyCode::Backspace | crossterm::event::KeyCode::Delete => {
            if let Some(cell) = editor.document.get_mut_cell(&editor.cursor_position) {
                cell.handle_delete(key);
            }
        }
        _ => {}
    }
}

pub fn handle_command_mode_press(editor: &mut Editor, key: crossterm::event::KeyCode) {
    match key {
        crossterm::event::KeyCode::Esc => {
            editor.command = Cell::default();
            editor.mode = Mode::Normal;
        }
        crossterm::event::KeyCode::Char(c) => {
            if !c.is_control() {
                editor.command.insert(c);
            }
        }
        crossterm::event::KeyCode::Enter => editor.execute_command(),
        crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Right => {
            editor.command.move_cursor(key);
        }
        crossterm::event::KeyCode::Delete | crossterm::event::KeyCode::Backspace => {
            editor.command.handle_delete(key);
        }
        _ => {}
    }
}

pub fn handle_delete_mode_press(editor: &mut Editor, key: crossterm::event::KeyCode) {
    match key {
        crossterm::event::KeyCode::Char(' ') => editor.document.clear_cell(&editor.cursor_position),
        crossterm::event::KeyCode::Char('r') => {
            editor.document.delete_row(editor.cursor_position.row)
        }
        crossterm::event::KeyCode::Char('c') => {
            editor.document.delete_column(editor.cursor_position.col)
        }
        crossterm::event::KeyCode::Esc => editor.mode = Mode::Normal,
        _ => editor.status_message = StatusMessage::from("Unrecognized command"),
    }

    editor.mode = Mode::Normal;
}

pub fn handle_save_as_mode_press(editor: &mut Editor, key: crossterm::event::KeyCode) {
    let mut filename = if let Some(filename) = &editor.document.filename {
        String::from(filename)
    } else {
        String::new()
    };

    match key {
        crossterm::event::KeyCode::Char(c) => {
            if !c.is_control() {
                filename.push(c);
            }
        }
        crossterm::event::KeyCode::Backspace => {
            filename.truncate(filename.len().saturating_sub(1));
        }
        crossterm::event::KeyCode::Esc => {
            editor.document.filename = None;
            editor.mode = Mode::Normal;
            editor.status_message = StatusMessage::from("Save aborted");

            return;
        }
        crossterm::event::KeyCode::Enter => {
            editor.mode = Mode::Normal;

            match editor.save() {
                Ok(()) => {
                    editor.status_message = StatusMessage::from("Success");
                }
                Err(err) => {
                    editor.status_message = StatusMessage::from(err.to_string());
                }
            };

            return;
        }
        _ => {}
    }

    editor.status_message = StatusMessage::from(format!("Save as: {filename}"));

    if !filename.is_empty() {
        editor.document.filename = Some(filename);
    }
}

pub fn handle_search_mode_press(editor: &mut Editor, key: crossterm::event::KeyCode) {
    match key {
        crossterm::event::KeyCode::Esc => {
            editor.search_text = Cell::default();
            editor.mode = Mode::Normal;
            editor.search_mode = SearchMode::None;
        }
        crossterm::event::KeyCode::Enter => {
            editor.search();
        }
        crossterm::event::KeyCode::Delete | crossterm::event::KeyCode::Backspace => {
            if editor.search_mode == SearchMode::None || editor.search_mode == SearchMode::Error {
                editor.search_text.handle_delete(key);
                editor.search_mode = SearchMode::None;
            }
        }
        crossterm::event::KeyCode::Down
        | crossterm::event::KeyCode::Up
        | crossterm::event::KeyCode::Left
        | crossterm::event::KeyCode::Right => {
            editor.move_cursor(key);
        }
        crossterm::event::KeyCode::Char(c) => {
            if editor.search_mode != SearchMode::None && editor.search_mode != SearchMode::Error {
                if editor.search_mode != SearchMode::None
                    && (c == 'j' || c == 'k' || c == 'h' || c == 'l')
                {
                    editor.move_cursor(key);
                }

                if c == '/' {
                    editor.search_text = Cell::default();
                    editor.search_mode = SearchMode::None;
                }

                return;
            }

            if !c.is_control() {
                editor.search_text.insert(c);
                editor.search_mode = SearchMode::None;
            }
        }
        _ => {}
    }
}
