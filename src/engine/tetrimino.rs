use cgmath::Vector2;

pub(crate) enum TType {
    Square,
    Line,
    T,
    J,
    L,
    S,
    Z,
}

pub(crate) enum Rotation {
    N, S, E, W,
}

pub(super) struct Tetrimino {
    ttype: TType,
    position: Vector2<usize>,
    rotation: Rotation,
}

impl Tetrimino {
    pub fn new(ttype: TType, position: Vector2<usize>, rotation: Rotation) -> Self {
        Self {
            ttype,
            position,
            rotation,
        }
    }
}

