mod board;
mod tetrimino;

use crate::engine::tetrimino::{TType, Tetrimino};
use board::Board;
use rand::prelude::{SliceRandom, ThreadRng};
use crate::engine::board::CellState;


type Coordinate = cgmath::Point2<usize>;
type Offset = cgmath::Vector2<isize>;


pub struct Engine {
    board: Board,
    bag: Vec<TType>,
    rng: ThreadRng,
    cursor: Option<Tetrimino>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::empty(),
            bag: Vec::new(),
            rng: rand::thread_rng(),
            cursor: None,
        }
    }

    // Generates a new bag and shuffles it
    fn regen_bag(&mut self) {
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

    fn place_cursor(&mut self) {
        let cursor = self.cursor.take().expect("Called 'place_cursor' with a pieceless cursor");
        while let Some(cells) = cursor.cells() {
            for coord in cells {
                let tetrimino = self.board.get_mut(coord).expect("Tried to get an out-of-bounds Tetrimino");
                debug_assert_eq!(*tetrimino, CellState::Empty);
                *tetrimino = CellState::Occupied;
            }
        }
    }
}
