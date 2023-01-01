mod board;
mod tetrimino;

use crate::engine::board::CellState;
use crate::engine::tetrimino::{TType, Tetrimino};
use board::Board;
use rand::prelude::{SliceRandom, ThreadRng};

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
        let cursor = self
            .cursor
            .take()
            .expect("Called 'place_cursor' with a pieceless cursor");
        while let Some(cells) = cursor.cells() {
            for coord in cells {
                let tetrimino = self
                    .board
                    .get_mut(coord)
                    .expect("Tried to get an out-of-bounds Tetrimino");
                debug_assert_eq!(*tetrimino, CellState::Empty);
                *tetrimino = CellState::Occupied;
            }
        }
    }

    fn move_cursor(&mut self, direction: &MoveDirection) -> Result<(), ()> {
        let Some(cursor) =  self.cursor.as_mut() else {
            return Ok(());
        };

        let offset = direction.offset();
        let future_tetrimino =
            Tetrimino::new(cursor.ttype, cursor.position + offset, cursor.rotation);

        if future_tetrimino.cells().is_some() {
            *cursor = future_tetrimino;
        } else {
            return Err(());
        }

        Ok(())
    }
}

pub enum MoveDirection {
    Left,
    Right,
}

impl MoveDirection {
    fn offset(&self) -> Offset {
        match self {
            MoveDirection::Left => Offset::new(-1, 0),
            MoveDirection::Right => Offset::new(1, 0),
        }
    }
}
