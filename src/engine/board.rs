use crate::engine::Coordinate;
use crate::engine::tetrimino::Tetrimino;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CellState {
    Empty,
    Occupied,
}

pub(crate) struct Board([CellState; Self::CELL_COUNT]);

impl Board {
    // Dimensions of the game grid in cells
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;

    const CELL_COUNT: usize = Self::WIDTH * Self::HEIGHT;

    pub(crate) fn empty() -> Self {
        Self([CellState::Empty; Self::CELL_COUNT])
    }

    pub fn is_inside(coords: Coordinate) -> bool {
        coords.x < Self::WIDTH && coords.y < Self::HEIGHT
    }

    pub fn can_be_placed(&self, tetrimino: &Tetrimino) -> bool{
        let Some(cells) = tetrimino.cells() else {return false};
        cells.into_iter()
            .all(|coords| {
                self.0[Self::coord_index(coords)] == CellState::Empty
                }
            );
        true
    }

    pub fn get_mut(&mut self, coord: Coordinate) -> Option<&mut CellState> {
        if Self::is_inside(coord) {
            return Some(&mut self.0[Self::coord_index(coord)]);
        }
        None
    }

    pub fn coord_index(coord: Coordinate) -> usize {
        coord.y * Self::WIDTH + coord.x
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
