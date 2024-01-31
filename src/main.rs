use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_voxels::{camera::PlayerCameraPlugin, raymarch::RaymarchPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .add_plugins(RaymarchPlugin)
        .add_plugins(PlayerCameraPlugin)
        .run();
}
