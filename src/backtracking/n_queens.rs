pub struct Board {
    board: Vec<Vec<bool>>,
    n: usize,
}

impl Board {
    pub fn new(n: usize) -> Board {
        Board {
            n: n,
            board: vec![vec![false; n]; n],
        }
    }

    pub fn place_queens(&mut self, column: usize) -> bool {
        unimplemented!()
    }

    pub fn is_valid(&self, row: usize, col: usize) -> bool {
        unimplemented!()
    }
}
