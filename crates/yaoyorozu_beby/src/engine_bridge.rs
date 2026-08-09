// src/yaoyorozu_beby/engine_bridge.rs - エンジンブリッジ（全張替版）

use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct GameState {
    pub is_initialized: bool,
    pub current_scene: String,
    pub is_loading: bool,
}

pub struct YaoyorozuBridgePlugin;

impl Plugin for YaoyorozuBridgePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState {
            is_initialized: false,
            current_scene: "idle".to_string(),
            is_loading: false,
        })
        .add_systems(Update, process_yaoyorozu_commands);
    }
}

pub fn process_yaoyorozu_commands(mut commands: Commands, mut game_state: ResMut<GameState>) {
    if !game_state.is_initialized {
        println!("八百万スクリプトのブリッジを初期化中...");
        game_state.is_initialized = true;
    }

    if game_state.current_scene == "idle" {
        game_state.current_scene = "title_screen".to_string();
    }
}
