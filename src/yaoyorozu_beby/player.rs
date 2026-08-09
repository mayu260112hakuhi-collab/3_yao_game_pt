// src/yaoyorozu_beby/player.rs - プレイヤーコンポーネント（全張替版）

use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub power: f32,
    pub defense: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "主".to_string(),
            power: 10.0,
            defense: 9999.0,
        }
    }
}
