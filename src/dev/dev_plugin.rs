use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin};

use crate::dev::systems::{dev_startup, dev_update, draw_colliders};

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        // Bevy Inspector
        app.add_plugins((DefaultInspectorConfigPlugin, WorldInspectorPlugin::new()));

        // Frame Diagnostics
        app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));

        app.add_systems(Startup, dev_startup);
        app.add_systems(Update, dev_update);
        app.add_systems(Update, draw_colliders);
    }
}
