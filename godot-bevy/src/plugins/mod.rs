use bevy::app::{App, Plugin};

pub mod core;
pub mod packed_scene;

pub struct DefaultGodotPlugin;

impl Plugin for DefaultGodotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(core::GodotCorePlugin)
            .add_plugins(packed_scene::PackedScenePlugin);
    }
}
