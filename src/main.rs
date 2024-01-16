#![warn(clippy::all, clippy::pedantic)]

mod cell;
mod document;
mod row;

use cell::Cell;
use document::Document;
use row::Row;

fn main() {
    let mut document = Document::default();

    document.insert_row(1);
    
    if let Some(row) = document.mut_row(0) {
        row.insert_at(0, 'c');
        row.insert_at(0, 'h');
        row.insert_at(0, 'a');
        row.insert_at(0, 'r');
    }

    println!("{}", document.at(0, 0).unwrap().to_str());
}
