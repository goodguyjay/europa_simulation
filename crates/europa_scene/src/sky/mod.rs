use super::timeflow::{SimSet, SimTime};
use bevy::prelude::*;
use europa_math::smoothstep;
use std::f32::consts::PI;

pub mod jupiter;
mod planetshine;
mod starfield;
mod sun;

pub struct SkyPlugin;

#[derive(Resource, Default, Clone, Copy)]
pub struct SkyState {
    pub eclipse_factor: f32,
    pub planetshine_factor: f32,
    pub sun_dir: Vec3,
    pub jupiter_dir: Vec3,
}

#[derive(Resource, Clone)]
pub struct SkySettings {
    pub base_jupiter_dir: Vec3,
    pub base_sun_dir: Vec3,
    pub sun_illuminance: f32,
    pub ambient_brightness: f32,
    pub europa_day_seconds: f32,
    pub orbit_normal: Vec3,
    pub jupiter_libration_lat: f32,
    pub jupiter_libration_lon: f32,
    pub sun_ang_radius: f32,
    pub jupiter_ang_radius: f32,
    pub planetshine_max: f32,
    pub eclipse_soft: f32,
}

impl Default for SkySettings {
    fn default() -> Self {
        Self {
            base_sun_dir: Vec3::new(-0.15, 0.35, -0.92).normalize(),
            base_jupiter_dir: Vec3::new(0.35, 0.25, -0.90).normalize(),
            sun_illuminance: 4500.0, // sun at ~5 AU
            ambient_brightness: 0.02,
            europa_day_seconds: 3.551181_f32 * 86_400.0,
            orbit_normal: Vec3::Y,
            jupiter_libration_lat: 2.0_f32.to_radians(), // tiny artistic wiggle
            jupiter_libration_lon: 2.0_f32.to_radians(),
            sun_ang_radius: (0.53_f32 / 5.2).to_radians() * 0.5, // half angle
            jupiter_ang_radius: 12.0_f32.to_radians() * 0.5,
            planetshine_max: 0.006,
            eclipse_soft: 1.0_f32.to_radians(),
        }
    }
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkySettings>()
            .init_resource::<SkyState>()
            .add_plugins((
                sun::SunPlugin,
                jupiter::JupiterPlugin,
                starfield::StarfieldPlugin,
                planetshine::PlanetshinePlugin,
            ))
            .add_systems(Update, animate_sky_physical.in_set(SimSet::Animate));
    }
}

pub fn animate_sky_physical(
    mut settings: ResMut<SkySettings>,
    mut state: ResMut<SkyState>,
    sim: Res<SimTime>,
) {
    // tidal lock + teeny libration
    let t = sim.0;
    let wob_lat =
        settings.jupiter_libration_lat * (0.3 * t / settings.europa_day_seconds * 2.0 * PI).sin();
    let wob_lon =
        settings.jupiter_libration_lon * (0.2 * t / settings.europa_day_seconds * 2.0 * PI).cos();

    let up = settings.orbit_normal.normalize();
    let right = settings.base_jupiter_dir.normalize().cross(up).normalize();
    let jup_wobble = Quat::from_axis_angle(right, wob_lat) * Quat::from_axis_angle(up, wob_lon);
    let jupiter_dir = (jup_wobble * settings.base_jupiter_dir).normalize();

    // sun laps the sky once per Europan day
    let phase = (t / settings.europa_day_seconds) * 2.0 * PI;
    let rot = Quat::from_axis_angle(up, phase);
    let noon = (-jupiter_dir).reject_from(up).normalize();
    let sun_dir = (rot * noon).normalize();

    // eclipse factor, dim direct sun when it passes behind jupiter
    let sep = sun_dir.angle_between(-jupiter_dir);
    let penumbra = settings.jupiter_ang_radius + settings.sun_ang_radius;
    let eclipse = smoothstep(penumbra, penumbra + settings.eclipse_soft, sep).clamp(0.0, 1.0);

    // planetshine
    let phase_brightness = (PI - sep) / PI; // 0..1
    let planetshine =
        (settings.planetshine_max * phase_brightness).clamp(0.0, settings.planetshine_max);

    state.sun_dir = sun_dir;
    state.jupiter_dir = jupiter_dir;
    state.eclipse_factor = eclipse;
    state.planetshine_factor = planetshine;

    settings.base_sun_dir = sun_dir;
}

trait Reject {
    fn reject_from(self, n: Vec3) -> Vec3;
}
impl Reject for Vec3 {
    fn reject_from(self, n: Vec3) -> Vec3 {
        self - self.project_onto(n)
    }
}
