use std::ops::{Index, IndexMut};

use crate::engine::tetrimino::Tetrimino;
use crate::engine::Coordinate;

use super::tetrimino::Color;

pub(crate) struct Board([Option<Color>; Self::CELL_COUNT]);

impl Board {
    // Dimensions of the game grid in cells
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;

    const CELL_COUNT: usize = Self::WIDTH * Self::HEIGHT;

    pub(crate) fn empty() -> Self {
        Self([None; Self::CELL_COUNT])
    }

    pub fn is_inside(coords: Coordinate) -> bool {
        coords.x < Self::WIDTH && coords.y < Self::HEIGHT
    }

    pub fn is_valid_coord(coords: Coordinate) -> bool {
        coords.x < Self::WIDTH
    }

    pub fn can_be_placed(&self, tetrimino: &Tetrimino) -> bool {
        let Some(cells) = tetrimino.cells() else {return false};
        cells
            .into_iter()
            .all(|coords| Board::is_inside(coords) && self[coords].is_none())
    }

    pub fn is_clipping(&self, tetrimino: &Tetrimino) -> bool {
        let Some(cells) = tetrimino.cells() else {return true};
        cells
            .into_iter()
            .all(|coords| !Board::is_inside(coords) || self[coords].is_some())
    }

    pub fn coord_index(coord: Coordinate) -> usize {
        coord.y * Self::WIDTH + coord.x
    }
}

impl Index<Coordinate> for Board {
    type Output = Option<Color>;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(Self::is_inside(coord));
        &self.0[Self::coord_index(coord)]
    }
}

impl IndexMut<Coordinate> for Board {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(Self::is_inside(coord));
        &mut self.0[Self::coord_index(coord)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bounds_check() {
        let coords_out_x = Coordinate::new(10, 9);
        let coords_out_y = Coordinate::new(9, 20);
        let coords_in = Coordinate::new(9, 19);

        assert_eq!(Board::is_inside(coords_out_x), false);
        assert_eq!(Board::is_inside(coords_out_y), false);
        assert_eq!(Board::is_inside(coords_in), true);
    }
}
