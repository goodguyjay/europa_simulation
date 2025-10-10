use bevy::prelude::*;
use bevy::window::{PresentMode, WindowPlugin};
use europa_scene::ScenePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Europa View".into(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "../../assets".into(),
                    ..default()
                }),
        )
        .add_plugins(ScenePlugin)
        .run();
}
