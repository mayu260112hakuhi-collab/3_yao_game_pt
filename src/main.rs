#![allow(non_snake_case)]

use bevy::prelude::*;
use std::fs;

// コアモジュールの読み込み
mod yaoyorozu_core;

use yaoyorozu_core::executor::{ASTを実行, MovieCutsceneState};
use yaoyorozu_core::parser_jp::スクリプト全体を解析;

/// プレイヤーコンポーネントなのだ
#[derive(Component, Debug)]
pub struct Player {
    pub name: String,
    pub power: f32,
    pub test: i32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<MovieCutsceneState>()
        .add_systems(Startup, (setup_camera, setup_script_engine))
        .run();
}

/// 3Dカメラの初期化なのだ
fn setup_camera(mut commands: Commands) {
    // Camera3d を直接スポーンさせるのだ！
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// スクリプトエンジンの起動と実行システムなのだ
fn setup_script_engine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cutscene_state: ResMut<MovieCutsceneState>,
) {
    info!("八百万システムを初期化。");

    let script_path = "assets/scripts/test.txt";
    let script_content = match fs::read_to_string(script_path) {
        Ok(content) => content,
        Err(_) => {
            "もし（キャラクター === なし）なら\n実行（新規キャラクター作成）\nもし終わり\nロード（test）\n再生（conceptart.png）\n表示（――かつて、八百万の神々と人が共に歩んだ地……）".to_string()
        }
    };

    match スクリプト全体を解析(&script_content) {
        Ok(ast_list) => {
            let キャラ未選択フラグ = true;
            for ast in &ast_list {
                ASTを実行(
                    ast,
                    &mut commands,
                    &asset_server,
                    &mut cutscene_state,
                    キャラ未選択フラグ,
                );
            }
        }
        Err(e) => {
            error!("スクリプト解析エラーなのだ: {}", e);
        }
    }
}
