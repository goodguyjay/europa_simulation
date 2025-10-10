use bevy::prelude::*;
mod height;
mod mesh;
mod params;
mod systems;

pub use height::{HeightFn, HeightSource, arc, comb, noise, warp};
pub use params::TerrainParams;

#[derive(Clone)]
pub struct TerrainPlugin {
    pub params: TerrainParams,
    pub height: HeightFn,
}

impl TerrainPlugin {
    pub fn with(params: TerrainParams, height: HeightFn) -> Self {
        Self { params, height }
    }

    pub fn europa_default() -> Self {
        use crate::height::arc;
        use crate::height::noise::{PerlinFbm, PerlinRidged};
        use comb::Bias;
        use noise::Perlin;
        use warp::Warp2D;

        let seed = 1337;
        let base = PerlinFbm {
            perlin: Perlin::new(seed),
            freq: 1.0 / 600.0,
            octaves: 5,
            lacunarity: 2.0,
            gain: 0.5,
            amplitude: 1.0,
        };

        // anisotropic ridges aligned by an oriented wrapper later
        let ridged = PerlinRidged {
            perlin: Perlin::new(seed ^ 0xB529_7A4D),
            freq: (1.0 / 600.0) * 2.5,
            octaves: 4,
            lacunarity: 2.2,
            gain: 0.75,
            amplitude: 1.0,
            z_anisotropy: 2.0,
        };

        // orient ridges along line_dir
        let line_dir = Vec2::new(0.8, 0.2).normalize();
        let oriented = warp::Oriented {
            source: ridged,
            dir: line_dir,
            main_scale: 1.0,
            ortho_scale: 0.35,
        };

        let warp = Warp2D {
            source: comb::Add2 {
                a: base,
                b: oriented,
            },
            perlin: Perlin::new(seed ^ 0x9E37_79B9),
            warp_amp: 40.0,
            warp_freq: (1.0 / 600.0) * 0.6,
            octaves: 3,
            lacunarity: 2.1,
            gain: 0.55,
        };

        let combined = Bias {
            s: comb::Scale {
                s: warp,
                scale: 1.0,
            },
            bias: -0.1,
        };
        let params = TerrainParams {
            amp: 12.0,
            size: 3000.0,
            res: 512,
            freq: 1.0 / 600.0,
            line_dir,
            seed,
        };

        Self {
            params,
            height: arc(combined),
        }
    }
}

#[derive(Resource)]
struct HeightResource(pub HeightFn);

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.params)
            .insert_resource(HeightResource(self.height.clone()))
            .add_systems(Startup, systems::spawn_europa);
    }
}
