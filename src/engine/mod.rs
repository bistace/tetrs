mod board;
use board::*;

struct Engine {
    board: Board,
}

impl Engine {
    fn new() -> Self {
        Self {
            board: Board::empty(),
        }
    }
}