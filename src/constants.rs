use std::time::Duration;

// Viewbox Sizes
pub const CELL_VIEW_LEN: usize = 12;
pub const SHEET_VIEWBOX_HEIGHT: u16 = 12;
pub const SHEET_VIEWBOX_WIDTH: u16 = 10;

// Times
pub const POLL_TIME: u64 = 250;
pub const STATUS_MESSAGE_ELAPSE_TIME: std::time::Duration = Duration::new(5, 0);
