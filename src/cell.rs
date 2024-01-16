#[derive(Default, Clone)] // TODO: implement Copy?
pub struct Cell {
    text: String
}

impl Cell {
    pub fn to_str(&self) -> &String {
        &self.text // maybe add a | at the end to differentiate between cells?
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn insert(&mut self, c: char) {
        self.text.push(c);
    }
}
