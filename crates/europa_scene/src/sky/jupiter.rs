use crate::constants::{JUPITER_ANGULAR_DIAMETER_DEG, JUPITER_OBLIQUITY_DEG, SKY_RADIUS};
use crate::sky::SkySettings;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::prelude::*;

pub struct JupiterPlugin;

#[derive(Component)]
struct Jupiter;

impl Plugin for JupiterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_jupiter)
            .add_systems(Update, place_and_scale_jupiter);
    }
}

fn spawn_jupiter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let tex: Handle<Image> = asset_server.load("sky/Jupiter.jpg");

    let mat = mats.add(StandardMaterial {
        base_color_texture: Some(tex),
        unlit: false,
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Opaque,
        ..default()
    });

    let sphere_mesh = meshes.add(Mesh::from(Sphere::new(1.0).mesh().uv(64, 32)));

    commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(mat),
        Transform::default(),
        Jupiter,
        NoFrustumCulling,
        Name::new("Jupiter"),
    ));
}

fn place_and_scale_jupiter(
    settings: Res<SkySettings>,
    cam_q: Query<(&Transform, &Projection), (With<Camera3d>, Without<Jupiter>)>,
    mut jup_q: Query<&mut Transform, (With<Jupiter>, Without<Camera3d>)>,
) {
    let Ok((cam_t, proj)) = cam_q.single() else {
        return;
    };
    let Ok(mut t) = jup_q.single_mut() else {
        return;
    };

    let far = match proj {
        Projection::Perspective(p) => p.far,
        Projection::Orthographic(o) => o.far,
        _ => return,
    };
    let sky_r = (far * 0.85).min(SKY_RADIUS);

    let dir = settings.jupiter_dir.normalize_or_zero();
    t.translation = cam_t.translation + dir * sky_r;

    let theta = JUPITER_ANGULAR_DIAMETER_DEG.to_radians();
    let radius = sky_r * (0.5 * theta).tan();
    t.scale = Vec3::splat(radius);

    // orientation
    let forward = (-dir).normalize_or_zero();
    let mut up_hint = Vec3::Y;
    let axis = up_hint.cross(forward).normalize_or_zero();
    if axis.length_squared() > 0.0 {
        up_hint = Quat::from_axis_angle(axis, JUPITER_OBLIQUITY_DEG.to_radians()) * up_hint;
    }

    t.look_to(forward, up_hint);

    let right = t.right().as_vec3();

    t.rotate(Quat::from_axis_angle(right, -std::f32::consts::FRAC_PI_2));

    // todo: spin jupiter slowly about its forward axis
    // let spin = Quat::from_axis_angle(f, sim.0 * (2.0*PI / (9.9*3600)) * spin_time_scale);
    // t.rotation = spin * t.rotation;
}
