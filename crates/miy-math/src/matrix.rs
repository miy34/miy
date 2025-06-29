use crate::WorldDimensions2D;

#[derive(Copy, Clone, Debug)]
pub struct ProjectionMatrix([f32; 16]);

impl ProjectionMatrix {
    pub const fn identity() -> Self {
        let values = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];

        Self(values)
    }

    pub const fn as_ptr(&self) -> *const f32 {
        self.0.as_ptr()
    }

    pub const fn to_array(self) -> [f32; 16] {
        self.0
    }

    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.0.as_ptr().cast(), 16 * 4) }
    }

    /// Column-major orthographic projection matrix
    pub fn with_ortho_from_world(world_dimensions: WorldDimensions2D) -> Self {
        let near = 0.0;
        let far = 1.0;

        let x = 2.0 / world_dimensions.width().get();
        let y = 2.0 / world_dimensions.height().get();

        let z = -(2.0 / (far - near));
        let za = -((far + near) / (far - near));

        #[rustfmt::skip]
        let values = [
              x,  0.0, 0.0, 0.0,
            0.0,    y, 0.0, 0.0,
            0.0,  0.0,   z, 0.0,
            0.0, -1.0,  za, 1.0,
        ];

        Self(values)
    }
}
