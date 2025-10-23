use super::{SkySettings, SkyState};
use crate::sky::SimSet;
use bevy::prelude::*;

pub struct PlanetshinePlugin;

#[derive(Component)]
struct PlanetshineLight;

impl Plugin for PlanetshinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planetshine)
            .add_systems(Update, update_planetshine.in_set(SimSet::Animate));
    }
}

fn spawn_planetshine(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 0.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::IDENTITY,
        PlanetshineLight,
        Name::new("Planetshine"),
    ));
}

fn update_planetshine(
    settings: Res<SkySettings>,
    state: Res<SkyState>,
    mut q: Query<(&mut DirectionalLight, &mut Transform), With<PlanetshineLight>>,
) {
    if settings.planetshine_max <= 0.0 {
        return;
    }

    if let Ok((mut light, mut t)) = q.single_mut() {
        t.rotation = Transform::IDENTITY
            .looking_to(-state.jupiter_dir.normalize(), Vec3::Y)
            .rotation;

        light.illuminance = settings.sun_illuminance * state.planetshine_factor;
    }
}
