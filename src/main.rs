#![warn(clippy::all, clippy::pedantic)]

mod cell;
mod document;
mod row;

use cell::Cell;
use document::Document;
use row::Row;

fn main() {
    let mut document = Document::default();

    document.insert_at(0, 0, 'c');
    document.insert_at(0, 0, 'h');
    document.insert_at(0, 0, 'a');
    document.insert_at(0, 0, 'r');

    println!("{}", document.get_cell(0, 0).unwrap().to_str());
}
