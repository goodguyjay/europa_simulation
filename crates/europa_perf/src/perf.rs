use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct PerfPlugin;

impl Plugin for PerfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Update, log_fps);
    }
}

fn log_fps(d: Res<DiagnosticsStore>, time: Res<Time>, mut timer: Local<Timer>) {
    if timer.is_paused() && timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.5, TimerMode::Repeating);
    }

    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    if let Some(diag) = d.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = diag.smoothed().or_else(|| diag.average()) {
            println!("fps ~ {:.1}", fps);
        }
    }
}
