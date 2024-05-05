use nom::{IResult, ParseTo};
use nom::character::{is_digit, is_alphanumeric};
use nom::bytes::complete::take_while1;
use nom::Err;

use crate::document::Document;

struct CellRef {
    row: String,
    col: u32,
}

enum ValueItem {
    CellRef(CellRef),
    Number(u32),
    Error(String)
}

struct Node {
    children: Vec<Node>,
    value: ValueItem,
}

pub fn parse(input: Option<&str>, document: &Document) -> String {
    if input.is_none() {
        return "".to_string();
    }

    match parse_node(input.unwrap().as_bytes()).value {
        ValueItem::CellRef(val) => return "TODO: CellRef Not Yet Implemented".to_string(),
        ValueItem::Number(val) => return val.to_string(),
        ValueItem::Error(val) => return val
    }
}

// TODO: change function signature to return an IResult
fn parse_node(input: &[u8]) -> Node {
    let number: IResult<&[u8], &[u8]> = take_while1(is_digit)(input);

    match number {
        Ok((remainder, val)) => {
            let num: Option<u32> = val.parse_to();
            return Node{children: vec!(), value: ValueItem::Number(num.unwrap())}
        },
        Err(_) => {
            return Node{ children: vec!(), value: ValueItem::Error("TODO: Not Yet Implemented".to_string()) };
            // TODO: parse cell ref
        }
    }

//    let (remainder, (number, cell_ref)) = (take_while1(is_digit), take_while1(is_alphanumeric)).parse(input)?;
}
