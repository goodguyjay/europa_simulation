use super::timeflow::{SimSet, SimTime};
use bevy::light::DirectionalLightShadowMap;
use bevy::prelude::*;

pub mod jupiter;
mod starfield;
pub mod sun;

pub struct SkyPlugin;

#[derive(Resource)]
pub struct SkySettings {
    pub sun_dir: Vec3,
    pub jupiter_dir: Vec3,
    pub sun_illuminance: f32,
    pub ambient_brightness: f32,
}

impl Default for SkySettings {
    fn default() -> Self {
        Self {
            sun_dir: Vec3::new(-0.15, 0.35, -0.92).normalize(),
            jupiter_dir: Vec3::new(0.35, 0.25, -0.90).normalize(),
            sun_illuminance: 4500.0, // sun at ~5 AU
            ambient_brightness: 0.02,
        }
    }
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkySettings>()
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_plugins((
                sun::SunPlugin,
                jupiter::JupiterPlugin,
                starfield::StarfieldPlugin,
            ))
            .add_systems(Update, animate_sky.in_set(SimSet::Animate));
    }
}

pub fn animate_sky(mut settings: ResMut<SkySettings>, sim: Res<SimTime>) {
    // fake orbit: rotate around +Y. period ~3.55 days for europa
    // one day every 720 secs
    let period = 1024.0;
    let theta = 2.0 * std::f32::consts::PI * (sim.0 / period);

    let rot = Quat::from_rotation_y(theta);
    let base_sun = Vec3::new(-0.15, 0.35, -0.92).normalize();
    let base_jup = Vec3::new(0.35, 0.25, -0.90).normalize();

    settings.sun_dir = (rot * base_sun).normalize();
    // jupiter on the opposite side of the sky
    settings.jupiter_dir = (rot * base_jup).normalize();
}
