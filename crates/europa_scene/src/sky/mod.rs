use super::timeflow::{SimSet, SimTime};
use bevy::prelude::*;
use europa_math::smoothstep;
use std::f32::consts::PI;

pub mod jupiter;
mod planetshine;
mod starfield;
mod sun;

#[derive(Resource)]
pub struct SkyAssets {
    pub jupiter_tex: Handle<Image>,
    pub sun_tex: Handle<Image>,
    pub starfield_tex: Handle<Image>,
    pub all_loaded: bool,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AssetLoadState {
    #[default]
    Loading,
    Ready,
}

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
            ambient_brightness: 0.05,
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
        app.init_state::<AssetLoadState>()
            .add_systems(Startup, load_sky_assets)
            .add_systems(
                Update,
                check_sky_assets_loaded.run_if(in_state(AssetLoadState::Loading)),
            )
            .init_resource::<SkySettings>()
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

fn load_sky_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let jupiter_tex: Handle<Image> = asset_server.load("sky/Jupiter.jpg");
    let sun_tex: Handle<Image> = asset_server.load("sky/Sun.jpg");
    let starfield_tex: Handle<Image> = asset_server.load("sky/Starfield.jpg");

    commands.insert_resource(SkyAssets {
        jupiter_tex,
        sun_tex,
        starfield_tex,
        all_loaded: false,
    });
}

fn check_sky_assets_loaded(
    mut assets: ResMut<SkyAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AssetLoadState>>,
) {
    if assets.all_loaded {
        return;
    }

    let jupiter_loaded = asset_server.is_loaded_with_dependencies(&assets.jupiter_tex);
    let sun_loaded = asset_server.is_loaded_with_dependencies(&assets.sun_tex);
    let starfield_loaded = asset_server.is_loaded_with_dependencies(&assets.starfield_tex);

    if jupiter_loaded && sun_loaded && starfield_loaded {
        assets.all_loaded = true;
        next_state.set(AssetLoadState::Ready);
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
