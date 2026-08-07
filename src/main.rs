#![allow(non_snake_case)] // これをファイルの先頭に追加

// main.rs
//
mod yaoyorozu_core; // これで src/yaoyorozu_core/mod.rs が読み込まれる

use bevy::prelude::*;
use bevy_skein::SkeinPlugin;

// 修正したパスでインポート
use crate::yaoyorozu_core::executor::ASTを実行;
use crate::yaoyorozu_core::parser_jp::スクリプト全体を解析;

fn main() {
    App::new()
        .register_type::<Player>()
        .add_plugins((DefaultPlugins, SkeinPlugin::default()))
        .add_systems(Startup, (setup_environment, run_script_system))
        .run();
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
pub struct Player {
    pub name: String,
    pub power: f32,
    pub test: i32,
}

/// 3Dカメラ、ライト、glTFモデルのロード（Bevy環境初期化）
fn setup_environment(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 3Dカメラ配置
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ライティング配置
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Skein連携シーン（glTF/glb）の配置
    commands.spawn(WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/test.glb")),
    ));
}

/// 八百万駆動スクリプトのロード・解析・実行システム
fn run_script_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let メインスクリプト = r#"
// mainSystem.8g - 八百万駆動 メインシステムスクリプト

「"八百万システムを初期化。"」をログ出力。
「"ゲーム起動"」を実行。

「"ゲーム"」を初期化。
「"ゲームタイトルムービー"」を再生 + 「"ゲームタイトル画面、キャラクターリスト"」をロード。
「"キャラクター"」を選択。

もし（キャラクター === なし）｛
    「"新規キャラクター作成"」を実行。
｝それ以外｛
    「"キャラクター選択画面"」へ移動。
｝

「"武蔵野平野_時の鐘"」を初期化。
「"プレイヤー1"」をロード。

// メインイベントループ
「"ゲームシステム起動完了"」を表示。
"#;

    info!("=== 八百万駆動スクリプトエンジン起動 ===");

    match スクリプト全体を解析(メインスクリプト) {
        Ok(ast_list) => {
            let キャラ未選択フラグ = true; // キャラクターが存在しない想定
            for node in ast_list {
                ASTを実行(&node, &mut commands, &asset_server, キャラ未選択フラグ);
            }
        }
        Err(err) => {
            error!("スクリプトのパースエラー: {}", err);
        }
    }
}
