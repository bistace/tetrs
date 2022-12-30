use cgmath::Vector2;
use crate::engine::board::Board;

pub(super) struct Tetrimino {
    ttype: TType,
    position: Vector2<isize>,
    rotation: Rotation,
}

impl Tetrimino {
    pub const CELL_COUNT: usize = 4;

    pub fn new(ttype: TType, position: Vector2<isize>, rotation: Rotation) -> Self {
        Self {
            ttype,
            position,
            rotation,
        }
    }

    pub fn cells(&self) -> Option<[Vector2<usize>; Self::CELL_COUNT]> {
        // Rotates and moves the cells
        let offsets: [Vector2<isize>; 4] = self.ttype.cells()
            .map(|cell| cell * self.rotation)
            .map(|cell| cell + self.position);

        let mut coords: [Vector2<usize>; 4] = [Vector2::<usize>::new(0, 0); 4];
        for(Vector2::<isize>{x, y}, coord) in offsets.into_iter().zip(&mut coords) {
            // Negatives bound-checking
            let tmp = match (x.try_into(), y.try_into()) {
                (Ok(x), Ok(y)) => Vector2::<usize>::new(x, y),
                _ => return None,
            };

            // Checks that the coords are not superior to the maximum value of x or y
            if Board::is_inside(tmp) {
                *coord = tmp;
            } else {
                return None;
            }
        }

        Some(coords)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum TType {
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

    fn cells(&self) -> [Vector2<isize>; Tetrimino::CELL_COUNT] {
        match self {
            TType::O => &[( 0, 0), ( 0, 1), (1, 0), (1, 1)],
            TType::I => &[(-1, 0), ( 0, 0), (1, 0), (2, 0)],
            TType::T => &[(-1, 0), ( 0, 0), (1, 0), (0, 1)],
            TType::J => &[(-1, 1), (-1, 0), (0, 0), (1, 0)],
            TType::L => &[(-1, 0), ( 0, 0), (1, 0), (1, 1)],
            TType::S => &[(-1, 0), ( 0, 0), (0, 1), (1, 1)],
            TType::Z => &[(-1, 1), ( 0, 1), (0, 0), (1, 0)],
        }.map(Vector2::<isize>::from)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Rotation {
    N,
    S,
    E,
    W,
}

impl<S> std::ops::Mul<Rotation> for Vector2<S>
where S: std::ops::Neg<Output=S> {
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {

        match rotation {
            Rotation::N => self,
            Rotation::S => Vector2::new(-self.x, -self.y),
            Rotation::E => Vector2::new( self.y, -self.x),
            Rotation::W => Vector2::new(-self.y,  self.x)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_s_tetrimino() {
        let s_north = Tetrimino {ttype: TType::S, position: Vector2::new(2, 3), rotation: Rotation::N};
        let s_south = Tetrimino {ttype: TType::S, position: Vector2::new(2, 3), rotation: Rotation::S};
        let s_east = Tetrimino {ttype: TType::S, position: Vector2::new(2, 3), rotation: Rotation::E};
        let s_west = Tetrimino {ttype: TType::S, position: Vector2::new(2, 3), rotation: Rotation::W};

        assert_eq!(
            s_north.cells(),
            Some([(1, 3), (2, 3), (2, 4), (3, 4)].map(Vector2::<usize>::from))
        );
        assert_eq!(
            s_south.cells(),
            Some([(3, 3), (2, 3), (2, 2), (1, 2)].map(Vector2::<usize>::from))
        );
        assert_eq!(
            s_east.cells(),
            Some([(2, 4), (2, 3), (3, 3), (3, 2)].map(Vector2::<usize>::from))
        );
        assert_eq!(
            s_west.cells(),
            Some([(2, 2), (2, 3), (1, 3), (1, 4)].map(Vector2::<usize>::from))
        );
    }
}