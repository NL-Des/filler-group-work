pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<char>,
}

impl Piece {
    pub fn new(width: usize, height: usize) -> Self {
        Piece {
            width,
            height,
            cells: vec!['.'; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: char) {
        self.cells[y * self.width + x] = value;
    }
}
