use bevy::prelude::*;
use europa_terrain::TerrainPlugin;

mod camera;
mod constants;
mod sky;
mod timeflow;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            timeflow::TimeFlowPlugin,
            camera::CameraPlugin,
            sky::SkyPlugin,
            TerrainPlugin::europa_default(),
        ));
    }
}
