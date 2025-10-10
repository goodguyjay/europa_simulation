use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct TerrainParams {
    /// world size in meters
    pub size: f32,
    /// grid resolution (N x N vertices)
    pub res: u32,
    /// vertical scale (meters)
    pub amp: f32,
    /// base frequency (smaller -> broader features)
    pub freq: f32,
    /// direction of "lineae" (normalized)
    pub line_dir: Vec2,
    /// rng seed
    pub seed: u32,
}

impl TerrainParams {
    pub fn europa_demo() -> Self {
        Self {
            size: 3000.0,
            res: 512,
            amp: 12.0,
            freq: 1.0 / 600.0,
            line_dir: Vec2::new(0.8, 0.2).normalize(),
            seed: 1337
        }
    }
}