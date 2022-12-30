use cgmath::Vector2;

#[derive(Clone, Copy)]
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

    pub fn is_inside(coords: Vector2<usize>) -> bool {
        coords.x < Self::WIDTH && coords.y < Self::HEIGHT
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bounds_check() {
        let coords_out_x = Vector2::<usize>::new(10, 9);
        let coords_out_y = Vector2::<usize>::new(9, 20);
        let coords_in = Vector2::<usize>::new(9, 19);

        assert_eq!(Board::is_inside(coords_out_x), false);
        assert_eq!(Board::is_inside(coords_out_y), false);
        assert_eq!(Board::is_inside(coords_in), true);
    }
}