#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
}

impl WorldPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
