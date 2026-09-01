pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<char>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
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
