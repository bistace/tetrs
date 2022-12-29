mod board;
mod tetrimino;

use board::Board;

pub struct Engine {
    board: Board,
    bag: Vec<tetrimino::TType>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::empty(),
            bag: Vec::new(),
        }
    }
}