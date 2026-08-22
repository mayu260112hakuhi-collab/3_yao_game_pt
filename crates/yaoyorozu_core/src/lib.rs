#![allow(non_snake_case)]

use std::fs;

use bevy::prelude::*;

pub mod command;
pub mod executor;
pub mod parser_jp;
pub mod runtime;

use executor::{MovieCutsceneState, RequestStateTransition, 八百万プログラム, プログラムを実行};
use parser_jp::スクリプト全体を解析;
use runtime::{八百万スクリプト設定, 八百万実行環境};

pub struct YamatoCoreEnginePlugin;

impl Plugin for YamatoCoreEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameFlowState>()
            .init_resource::<八百万実行環境>()
            .init_resource::<八百万スクリプト設定>()
            .init_resource::<八百万プログラム>()
            .init_resource::<MovieCutsceneState>()
            .add_message::<RequestStateTransition>()
            .add_systems(Startup, 起動8gを読み込む)
            .add_systems(Update, プログラムを実行);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameFlowState {
    #[default]
    Title,
    CharacterSelection,
    Loading,
    Gameplay,
    Settings,
}

fn 起動8gを読み込む(
    設定: Res<八百万スクリプト設定>,
    mut プログラム: ResMut<八百万プログラム>,
) {
    info!("【八百万駆動】8gランタイムを起動します");
    info!("【8g読込】{}", 設定.起動スクリプト);

    let ソース = match fs::read_to_string(&設定.起動スクリプト) {
        Ok(v) => v,
        Err(err) => {
            error!(
                "【8g読込失敗】{}: {}",
                設定.起動スクリプト,
                err
            );
            return;
        }
    };

    match スクリプト全体を解析(&ソース) {
        Ok(文一覧) => {
            info!("【8g解析成功】{} 文", 文一覧.len());
            プログラム.文一覧 = 文一覧;
            プログラム.実行済み = false;
        }
        Err(err) => {
            error!("【8g構文エラー】{}", err);
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct YamatoSaveData {
    pub character_name: String,
    pub last_checkpoint: String,
    pub play_time_seconds: u64,
}

impl Default for YamatoSaveData {
    fn default() -> Self {
        Self {
            character_name: "主".to_string(),
            last_checkpoint: "川越・時の鐘下".to_string(),
            play_time_seconds: 0,
        }
    }
}
