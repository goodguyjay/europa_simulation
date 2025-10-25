use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SimSet {
    Advance,
    Animate,
}

#[derive(Resource)]
pub struct TimeFlow {
    pub time_scale: f32, // seconds
}
impl Default for TimeFlow {
    fn default() -> Self {
        Self { time_scale: 1000.0 }
    }
}

#[derive(Resource, Default)]
pub struct SimTime(pub f32);

pub struct TimeFlowPlugin;
impl Plugin for TimeFlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeFlow>()
            .init_resource::<SimTime>()
            .configure_sets(Update, SimSet::Advance.before(SimSet::Animate))
            .add_systems(
                Update,
                (advance_time.in_set(SimSet::Advance), timeflow_controls),
            );
    }
}

fn advance_time(mut sim: ResMut<SimTime>, tf: Res<TimeFlow>, time: Res<Time>) {
    sim.0 += tf.time_scale * time.delta_secs();
}

fn timeflow_controls(keys: Res<ButtonInput<KeyCode>>, mut tf: ResMut<TimeFlow>) {
    // 1x real time, 600x (10 min/sec), 1200x (5 min/sec), 5000x
    let mut changed = false;

    if keys.just_pressed(KeyCode::Digit0) || keys.just_pressed(KeyCode::Numpad0) {
        tf.time_scale = 0.0;
        changed = true;
    }

    if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::Numpad1) {
        tf.time_scale = 1.0;
        changed = true;
    }

    if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::Numpad2) {
        tf.time_scale = 600.0;
        changed = true;
    }

    if keys.just_pressed(KeyCode::Digit3) || keys.just_pressed(KeyCode::Numpad3) {
        tf.time_scale = 1200.0;
        changed = true;
    }

    if keys.just_pressed(KeyCode::Digit4) || keys.just_pressed(KeyCode::Numpad4) {
        tf.time_scale = 5000.0;
        changed = true;
    }

    if changed {
        info!("Sim speed set to {}x", tf.time_scale);
    }
}
