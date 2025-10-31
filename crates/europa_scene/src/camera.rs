use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::view::Hdr;

#[derive(Resource, Default)]
struct CamLock {
    mode: LockMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LockMode {
    Free,
    Sun,
    Jupiter,
    // future things
}

impl Default for LockMode {
    fn default() -> Self {
        LockMode::Free
    }
}

pub struct CameraPlugin;

#[derive(Component)]
struct FlyCam {
    yaw: f32,
    pitch: f32,
    speed: f32,
    sensitivity: f32,
}

use crate::sky::SkySettings;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamLock>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (toggle_lock, mouse_look, kb_move, lock_aim_update));
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut t = Transform::from_xyz(0.0, 20.0, 8.0);
    t.look_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y);
    let (yaw, pitch, _roll) = t.rotation.to_euler(EulerRot::YXZ);

    commands.spawn((
        Camera3d::default(),
        Hdr,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        t,
        Projection::Perspective(PerspectiveProjection {
            near: 0.1,
            far: 50_000.0,
            fov: std::f32::consts::FRAC_PI_3,
            ..default()
        }),
        FlyCam {
            yaw,
            pitch,
            speed: 20.0,
            sensitivity: 0.002,
        },
    ));
}

fn toggle_lock(keys: Res<ButtonInput<KeyCode>>, mut lock: ResMut<CamLock>) {
    if keys.just_pressed(KeyCode::KeyF) {
        lock.mode = match lock.mode {
            LockMode::Free => LockMode::Sun,
            LockMode::Jupiter => LockMode::Sun,
            _ => LockMode::Free,
        };
    }

    if keys.just_pressed(KeyCode::KeyJ) {
        lock.mode = match lock.mode {
            LockMode::Free => LockMode::Jupiter,
            LockMode::Sun => LockMode::Jupiter,
            _ => LockMode::Free,
        };
    }

    info!(
        "Camera lock: {:?}",
        match lock.mode {
            LockMode::Free => "Free",
            LockMode::Sun => "Sun",
            LockMode::Jupiter => "Jupiter",
        }
    );
}

fn mouse_look(
    mut ev: MessageReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut q: Query<(&mut Transform, &mut FlyCam)>,
) {
    if !buttons.pressed(MouseButton::Right) {
        return;
    }

    let mut delta = Vec2::ZERO;
    for m in ev.read() {
        delta += m.delta
    }

    let (mut t, mut c) = q.single_mut().unwrap();
    c.yaw -= delta.x * c.sensitivity;
    c.pitch -= delta.y * c.sensitivity;
    c.pitch = c.pitch.clamp(-1.54, 1.54);

    t.rotation = Quat::from_euler(EulerRot::YXZ, c.yaw, c.pitch, 0.0);
}

fn kb_move(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut Transform, &FlyCam)>,
) {
    let (mut t, c) = q.single_mut().unwrap();
    let mut v = Vec3::ZERO;

    // wasd + space/ctrl
    if keys.pressed(KeyCode::KeyW) {
        v += *t.forward();
    }
    if keys.pressed(KeyCode::KeyS) {
        v -= *t.forward();
    }
    if keys.pressed(KeyCode::KeyA) {
        v += *t.left();
    }
    if keys.pressed(KeyCode::KeyD) {
        v += *t.right();
    }
    if keys.pressed(KeyCode::Space) {
        v += Vec3::Y;
    }
    if keys.pressed(KeyCode::ControlLeft) {
        v -= Vec3::Y;
    }

    let boost = if keys.pressed(KeyCode::ShiftLeft) {
        10.0
    } else {
        1.0
    };
    if v.length_squared() > 0.0 {
        t.translation += v.normalize() * c.speed * boost * time.delta_secs();
    }
}

fn lock_aim_update(
    lock: Res<CamLock>,
    settings: Res<SkySettings>,
    mut q: Query<&mut Transform, With<Camera3d>>,
) {
    if lock.mode == LockMode::Free {
        return;
    }

    let Ok(mut t) = q.single_mut() else {
        return;
    };

    match lock.mode {
        LockMode::Sun => {
            let forward = settings.base_sun_dir.normalize();
            t.look_to(forward, Vec3::Y);
            // damp/smooth later
        }
        LockMode::Jupiter => {
            let forward = settings.base_jupiter_dir.normalize();
            t.look_to(forward, Vec3::Y);
            // damp/smooth later
        }
        LockMode::Free => {}
    }
}
