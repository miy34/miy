use core::marker::PhantomData;

use crate::space;

mod tag {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Width;

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Height;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Dimensions2D<Space, Kind> {
    width: Width<Space, Kind>,
    height: Height<Space, Kind>,
}

pub type WorldDimensions2D = Dimensions2D<space::World, f32>;
pub type ScreenDimensions = Dimensions2D<space::Screen, u32>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Dimension<Space, Tag, Kind> {
    value: Kind,
    phantom: PhantomData<(Space, Tag)>,
}

pub type Width<Space, Kind> = Dimension<Space, tag::Width, Kind>;
pub type Height<Space, Kind> = Dimension<Space, tag::Height, Kind>;

pub type WorldWidth = Width<space::World, f32>;
pub type WorldHeight = Height<space::World, f32>;

pub type ScreenWidth = Width<space::Screen, u32>;
pub type ScreenHeight = Height<space::Screen, u32>;

impl<Space, Tag, Kind> Dimension<Space, Tag, Kind> {
    pub const fn get(&self) -> Kind
    where
        Kind: Copy,
    {
        self.value
    }
}

impl<Space, Tag> Dimension<Space, Tag, u32> {
    pub const fn new(value: u32) -> Option<Self> {
        if value > 0 {
            Some(Self {
                value,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }
}

impl<Space, Tag> Dimension<Space, Tag, f32> {
    pub const fn new(value: f32) -> Option<Self> {
        if value > 0.0 {
            Some(Self {
                value,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }
}

impl<Space, Kind> Dimensions2D<Space, Kind> {
    pub fn width(self) -> Width<Space, Kind> {
        self.width
    }

    pub fn height(self) -> Height<Space, Kind> {
        self.height
    }
}
impl<Space> Dimensions2D<Space, f32> {
    pub fn new(width: f32, height: f32) -> Option<Self> {
        Some(Self {
            width: Width::<Space, f32>::new(width)?,
            height: Height::<Space, f32>::new(height)?,
        })
    }
}

impl<Space> Dimensions2D<Space, u32> {
    pub fn new(width: u32, height: u32) -> Option<Self> {
        Some(Self {
            width: Width::<Space, u32>::new(width)?,
            height: Height::<Space, u32>::new(height)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dimension_invariants() {
        let world_dimensions = WorldDimensions2D::new(0.0, 1.0);
        assert_eq!(world_dimensions, None);

        let world_dimensions = WorldDimensions2D::new(1.0, 0.0);
        assert_eq!(world_dimensions, None);

        let world_dimensions = WorldDimensions2D::new(-1.0, 0.0);
        assert_eq!(world_dimensions, None);

        let world_dimensions = WorldDimensions2D::new(0.0, -1.0);
        assert_eq!(world_dimensions, None);

        let world_dimensions = WorldDimensions2D::new(1.0, 2.0).unwrap();
        assert_eq!(world_dimensions.width().get(), 1.0);
        assert_eq!(world_dimensions.height().get(), 2.0);

        let sceen_dimensions = ScreenDimensions::new(0, 1);
        assert_eq!(sceen_dimensions, None);

        let sceen_dimensions = ScreenDimensions::new(1, 0);
        assert_eq!(sceen_dimensions, None);

        let sceen_dimensions = ScreenDimensions::new(1, 2).unwrap();
        assert_eq!(sceen_dimensions.width().get(), 1);
        assert_eq!(sceen_dimensions.height().get(), 2);
    }
}
