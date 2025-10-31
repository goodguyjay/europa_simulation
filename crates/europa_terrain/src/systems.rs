use crate::{mesh::build_europa_mesh, params::TerrainParams, HeightResource};
use bevy::prelude::*;

pub(crate) fn spawn_europa(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    params: Res<TerrainParams>,
    height: Res<HeightResource>,
) {
    let mesh = build_europa_mesh(*params, height.0.as_ref());
    let handle = meshes.add(mesh);

    let mat = mats.add(StandardMaterial {
        // pale, icy
        base_color: Color::srgb(0.85, 0.88, 0.92),
        perceptual_roughness: 0.7,
        reflectance: 0.5,
        metallic: 0.0,
        alpha_mode: AlphaMode::Opaque,
        ..default()
    });

    commands.spawn((
        Mesh3d(handle),
        MeshMaterial3d(mat),
        Transform::from_translation(Vec3::ZERO),
        Name::new("Europa Terrain"),
    ));
}
