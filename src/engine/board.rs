pub enum CellState {
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
}

