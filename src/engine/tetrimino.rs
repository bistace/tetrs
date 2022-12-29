use cgmath::Vector2;

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

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum TType {
    Square,
    Line,
    T,
    J,
    L,
    S,
    Z,
}

impl TType {
    pub const ALL: [Self; 7] = [
        Self::Square,
        Self::Line,
        Self::T,
        Self::J,
        Self::L,
        Self::S,
        Self::Z,
    ];
}

pub(crate) enum Rotation {
    N,
    S,
    E,
    W,
}
