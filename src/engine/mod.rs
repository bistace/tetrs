mod board;
mod tetrimino;

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

        assert!(
            self.board.can_be_placed(&cursor),
            "Tried to place a cursor at an invalid position: {cursor:?}"
        );

        while let Some(cells) = cursor.cells() {
            for coord in cells {
                self.board[coord] = Some(cursor.ttype.color());
            }
        }
    }

    // Moves the cursor in any MoveDirection
    fn move_cursor(&mut self, direction: &MoveDirection) -> Result<(), ()> {
        let Some(cursor) = self.cursor.as_mut() else { return Ok(()); };
        let future_tetrimino = Tetrimino::from_direction(cursor, direction);

        if self.board.is_clipping(&future_tetrimino) {
            return Err(());
        }

        Ok(*cursor = future_tetrimino)
    }

    // Drops the cursor by one cell down
    pub fn soft_drop(&mut self) -> Result<(), ()> {
        self.move_cursor(&MoveDirection::Bottom)
    }

    // Checks that the cursor did not hit the bottom of the board or another piece
    fn cursor_bottomed_out(&self) -> bool {
        let Some(cursor) = self.cursor.as_ref() else { return false };
        let future_tetrimino = Tetrimino::from_direction(cursor, &MoveDirection::Bottom);
        self.board.is_clipping(&future_tetrimino)
    }

    pub fn hard_drop(&mut self) {
        while self.move_cursor(&MoveDirection::Bottom).is_ok() {
            continue;
        }
        self.place_cursor();
    }
}

#[derive(PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
    Bottom,
}

impl MoveDirection {
    fn offset(&self) -> Offset {
        match self {
            MoveDirection::Left => Offset::new(-1, 0),
            MoveDirection::Right => Offset::new(1, 0),
            MoveDirection::Bottom => Offset::new(0, -1),
        }
    }
}
