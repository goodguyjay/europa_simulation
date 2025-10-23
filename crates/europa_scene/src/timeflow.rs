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
            .add_systems(Update, advance_time.in_set(SimSet::Advance));
    }
}

fn advance_time(mut sim: ResMut<SimTime>, tf: Res<TimeFlow>, time: Res<Time>) {
    sim.0 += tf.time_scale * time.delta_secs();
}

fn time_scale_for_day_duration(minutes_real: f32, europa_day_seconds: f32) -> f32 {
    europa_day_seconds / (minutes_real * 60.0)
}
