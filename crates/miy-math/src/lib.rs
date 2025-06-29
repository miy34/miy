mod dimensions;
mod matrix;
mod position;

pub use dimensions::{
    ScreenDimensions, ScreenHeight, ScreenWidth, WorldDimensions2D, WorldHeight, WorldWidth,
};
pub use matrix::ProjectionMatrix;
pub use position::WorldPosition;

pub mod space {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct World;

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Screen;

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Clip;
}
