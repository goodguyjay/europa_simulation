use crate::sky::SkySettings;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::prelude::*;
use europa_math::smoothstep;

pub struct StarfieldPlugin;

#[derive(Component)]
struct StarDome;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_starfield)
            .add_systems(Update, (track_camera, dim_stars_near_sun));
    }
}

fn spawn_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let tex: Handle<Image> = asset_server.load("sky/Starfield.jpg");

    let dome_mesh = meshes.add(Mesh::from(Sphere::new(1.0).mesh().uv(128, 64)));
    let dome_mat = mats.add(StandardMaterial {
        base_color_texture: Some(tex),
        unlit: true,
        alpha_mode: AlphaMode::Opaque,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        Mesh3d(dome_mesh),
        MeshMaterial3d(dome_mat),
        Transform::from_scale(Vec3::splat(20_000.0)),
        StarDome,
        NoFrustumCulling,
        Name::new("Starfield Dome"),
    ));
}

fn track_camera(
    cam_q: Query<&Transform, (With<Camera3d>, Without<StarDome>)>,
    mut dome_q: Query<&mut Transform, (With<StarDome>, Without<Camera3d>)>,
) {
    let Ok(cam) = cam_q.single() else {
        return;
    };
    let Ok(mut t) = dome_q.single_mut() else {
        return;
    };

    // lock pos to camera so it never parallax-shifts
    t.translation = cam.translation;
}

// fade stars when the sun is near the view center... cheap "glare" ;)
fn dim_stars_near_sun(
    settings: Res<SkySettings>,
    cam_q: Query<(&Transform, &Projection), (With<Camera3d>, Without<StarDome>)>,
    star_q: Query<&MeshMaterial3d<StandardMaterial>, (With<StarDome>, Without<Camera3d>)>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let Ok((cam, proj)) = cam_q.single() else {
        return;
    };
    let Ok(star_mat) = star_q.single() else {
        return;
    };

    let view_dir = cam.forward().normalize();
    let sun_dir = settings.base_sun_dir.normalize();
    // angular separation between view center and the sun
    let sep = view_dir.dot(sun_dir).clamp(-1.0, 1.0).acos();

    // local glare cone around the sun
    let inner = 3.0_f32.to_radians(); // fully in glare core
    let outer = 12.0_f32.to_radians(); // full star brightness beyond this
    let local = smoothstep(inner, outer, sep);
    let local = 0.15 + 0.85 * local; // floor of 15%

    let fov = match proj {
        Projection::Perspective(p) => p.fov,
        Projection::Orthographic(_) => std::f32::consts::FRAC_PI_2,
        _ => return,
    };

    // start dimming if sun is within ~40% of the half-fov. gone by edge
    let g0 = fov * 0.15;
    let g1 = fov * 0.5;
    let global = smoothstep(g0, g1, sep);
    let global = 0.55 + 0.45 * global;

    let brightness = local * global;

    if let Some(mat) = mats.get_mut(&star_mat.0) {
        mat.base_color = Color::linear_rgb(brightness, brightness, brightness);
    }
}
