use crate::engine::board::Board;
use crate::engine::{Coordinate, MoveDirection, Offset};
use cgmath::Vector2;

#[derive(Debug)]
pub struct Tetrimino {
    pub ttype: TType,
    pub position: Offset,
    pub rotation: Rotation,
}

impl Tetrimino {
    pub const CELL_COUNT: usize = 4;

    pub fn new(ttype: TType, position: Offset, rotation: Rotation) -> Self {
        Self {
            ttype,
            position,
            rotation,
        }
    }

    pub fn from_direction(tetrimino: &Tetrimino, direction: &MoveDirection) -> Self {
        Self {
            ttype: tetrimino.ttype,
            position: tetrimino.position + direction.offset(),
            rotation: tetrimino.rotation,
        }
    }

    pub fn cells(&self) -> Option<[Coordinate; Self::CELL_COUNT]> {
        // Rotates and moves the cells
        let offsets: [Offset; 4] = self
            .ttype
            .cells()
            .map(|cell| self.rotate(cell))
            .map(|cell| cell + self.position);

        let mut coords: [Coordinate; 4] = [Coordinate::new(0, 0); 4];
        for (Offset { x, y }, coord) in offsets.into_iter().zip(&mut coords) {
            // Negatives bound-checking
            let tmp = match (x.try_into(), y.try_into()) {
                (Ok(x), Ok(y)) => Coordinate::new(x, y),
                _ => return None,
            };

            // Checks that the coords are not superior to the maximum value of x or y
            if Board::is_valid_coord(tmp) {
                *coord = tmp;
            } else {
                return None;
            }
        }

        Some(coords)
    }

    fn rotate(&self, cell: Offset) -> Offset {
        if self.ttype == TType::O {
            cell
        } else {
            let grid_offset = self.rotation.intrinsic_offset() * (self.ttype.grid_size() - 1);
            cell * self.rotation + grid_offset
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TType {
    O,
    I,
    T,
    J,
    L,
    S,
    Z,
}

impl TType {
    pub const ALL: [Self; 7] = [
        Self::O,
        Self::I,
        Self::T,
        Self::J,
        Self::L,
        Self::S,
        Self::Z,
    ];

    fn cells(&self) -> [Offset; Tetrimino::CELL_COUNT] {
        match self {
            TType::O => &[(1, 1), (1, 2), (2, 1), (2, 2)],
            TType::I => &[(0, 2), (1, 2), (2, 2), (3, 2)],
            TType::T => &[(0, 1), (1, 1), (2, 1), (1, 2)],
            TType::L => &[(0, 1), (1, 1), (2, 1), (2, 2)],
            TType::J => &[(0, 2), (0, 1), (1, 1), (2, 2)],
            TType::S => &[(0, 1), (1, 1), (1, 2), (2, 2)],
            TType::Z => &[(0, 2), (1, 2), (1, 1), (2, 1)],
        }
        .map(Vector2::<isize>::from)
    }

    pub fn grid_size(&self) -> isize {
        match self {
            TType::I => 4,
            _ => 3,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            TType::O => Color::Yellow,
            TType::I => Color::Cyan,
            TType::T => Color::Purple,
            TType::J => Color::Blue,
            TType::L => Color::Orange,
            TType::S => Color::Green,
            TType::Z => Color::Red,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rotation {
    N,
    E,
    S,
    W,
}

impl Rotation {
    fn intrinsic_offset(&self) -> Offset {
        match self {
            Rotation::N => Offset::new(0, 0),
            Rotation::E => Offset::new(0, 1),
            Rotation::S => Offset::new(1, 1),
            Rotation::W => Offset::new(1, 0),
        }
    }
}

impl<S> std::ops::Mul<Rotation> for Vector2<S>
where
    S: std::ops::Neg<Output = S>,
{
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::N => self,
            Rotation::E => Vector2::new(self.y, -self.x),
            Rotation::S => Vector2::new(-self.x, -self.y),
            Rotation::W => Vector2::new(-self.y, self.x),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Yellow,
    Cyan,
    Purple,
    Orange,
    Blue,
    Green,
    Red,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_s_tetrimino() {
        let s_north = Tetrimino {
            ttype: TType::S,
            position: Offset::new(2, 3),
            rotation: Rotation::N,
        };
        let s_east = Tetrimino {
            ttype: TType::S,
            position: Offset::new(2, 3),
            rotation: Rotation::E,
        };
        let s_south = Tetrimino {
            ttype: TType::S,
            position: Offset::new(2, 3),
            rotation: Rotation::S,
        };
        let s_west = Tetrimino {
            ttype: TType::S,
            position: Offset::new(2, 3),
            rotation: Rotation::W,
        };

        assert_eq!(
            s_north.cells(),
            Some([(2, 4), (3, 4), (3, 5), (4, 5)].map(Coordinate::from))
        );
        assert_eq!(
            s_east.cells(),
            Some([(3, 5), (3, 4), (4, 4), (4, 3)].map(Coordinate::from))
        );
        assert_eq!(
            s_south.cells(),
            Some([(4, 4), (3, 4), (3, 3), (2, 3)].map(Coordinate::from))
        );
        assert_eq!(
            s_west.cells(),
            Some([(3, 3), (3, 4), (2, 4), (2, 5)].map(Coordinate::from))
        );
    }
}
