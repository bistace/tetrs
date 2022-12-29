mod board;
use board::*;

pub struct Engine {
    board: Board,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::empty(),
        }
    }
}