mod board;
mod tetrimino;

use crate::engine::tetrimino::TType;
use board::Board;
use rand::prelude::{SliceRandom, ThreadRng};

pub struct Engine {
    board: Board,
    bag: Vec<TType>,
    rng: ThreadRng,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::empty(),
            bag: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    // Generates a new bag and shuffles it
    pub fn regen_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.fill_bag();
        self.shuffle_bag();
    }

    // Creates a new bag of Tetriminos
    fn fill_bag(&mut self) {
        self.bag.extend_from_slice(&TType::ALL);
    }

    // Shuffles the bag
    fn shuffle_bag(&mut self) {
        self.bag.shuffle(&mut self.rng);
    }
}
