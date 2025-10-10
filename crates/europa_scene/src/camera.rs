use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
struct FlyCam {
    yaw: f32,
    pitch: f32,
    speed: f32,
    sensitivity: f32,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (mouse_look, kb_move));
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut t = Transform::from_xyz(0.0, 20.0, 8.0);
    t.look_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y);
    let (yaw, pitch, _roll) = t.rotation.to_euler(EulerRot::YXZ);

    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
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
