use bevy::prelude::*;

pub mod engine_bridge;
pub mod main_system;
pub mod save_system;

pub use engine_bridge::*;
pub use main_system::*;
pub use save_system::*;

pub struct YaoyorozuBevyBundlePlugin;

impl Plugin for YaoyorozuBevyBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((YaoyorozuBridgePlugin, YamatoMainSystemRunnerPlugin));
    }
}
