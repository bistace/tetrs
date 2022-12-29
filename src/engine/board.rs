pub enum CellState {
    Empty,
    Occupied,
}

struct Board([CellState; Self::WIDTH * Self::HEIGHT]);

impl Board {
    // Dimensions of the game grid in cells
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;

    const CELL_COUNT: usize = Self::WIDTH * Self::HEIGHT;

    fn empty() -> Self {
        Self([CellState::Empty; Self::CELL_COUNT])
    }
}

