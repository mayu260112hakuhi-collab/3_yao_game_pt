#![allow(non_snake_case)]

use crate::command::{命令, 命令種別};
use crate::parser_jp::AstNode;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;

#[derive(Message)]
pub struct RequestStateTransition(pub String);

/// プレイヤーコンポーネント（防御全振り＆テスト値対応）
#[derive(Component, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub power: f32,
    pub defense: f32,
    pub test: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "主".to_string(),
            power: 10.0,
            defense: 9999.0,
            test: 1,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct MovieCutsceneState {
    pub is_playing: bool,
    pub current_bg_image: String,
    pub calligraphy_text: String,
}

impl Default for MovieCutsceneState {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_bg_image: String::new(),
            calligraphy_text: String::new(),
        }
    }
}

pub fn 命令を実行(
    命令: 命令,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    cutscene_state: &mut ResMut<MovieCutsceneState>,
    state_writer: &mut MessageWriter<RequestStateTransition>,
) {
    match 命令.動詞 {
        命令種別::ログ出力 | 命令種別::表示 => {
            info!("【八百万駆動・あらすじ表示】 {}", 命令.引数);
            cutscene_state.calligraphy_text = 命令.引数.clone();
        }

        命令種別::初期化 => {
            info!("【初期化】 システム/ステージ: {}", 命令.引数);
        }

        命令種別::実行 => {
            if 命令.引数 == "新規キャラクター作成" {
                info!("【Bevy連携】 Player エンティティを生成します");

                commands.spawn((
                    Player {
                        name: "プレイヤー1".to_string(),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
            }
        }

        命令種別::スポーン => {
            if 命令.引数.contains("キャラクター") {
                commands.spawn((
                    Player {
                        name: 命令.引数.clone(),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 1.0, 0.0),
                ));
            }
        }

        命令種別::ロード => {
            info!("【ロード】: {}", 命令.引数);

            if 命令.引数.contains("タイトル画面") {
                state_writer.write(RequestStateTransition("Title".to_string()));
            } else {
                // Bevy 0.19:
                // GLBそのものを読み込み、Scene(0)をWorldAssetRootで展開する。
                let path = format!("scenes/{}.glb", 命令.引数);

                info!("【GLTFロード要求】: {}", path);

                let scene_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(path));

                commands.spawn((WorldAssetRoot(scene_handle), Transform::default()));
            }
        }

        命令種別::再生 => {
            cutscene_state.is_playing = true;
            cutscene_state.current_bg_image = 命令.引数.clone();
        }

        _ => {
            warn!("【未実装命令】 {:?}: {}", 命令.動詞, 命令.引数);
        }
    }
}

pub fn ASTを実行(
    ノード: &AstNode,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    cutscene_state: &mut ResMut<MovieCutsceneState>,
    state_writer: &mut MessageWriter<RequestStateTransition>,
    キャラ未選択フラグ: bool,
) {
    match ノード {
        AstNode::単一命令(cmd) => {
            命令を実行(
                cmd.clone(),
                commands,
                asset_server,
                cutscene_state,
                state_writer,
            );
        }

        AstNode::複合命令(cmd_list) => {
            for cmd in cmd_list {
                命令を実行(
                    cmd.clone(),
                    commands,
                    asset_server,
                    cutscene_state,
                    state_writer,
                );
            }
        }

        AstNode::条件分岐 {
            条件文: _,
            真のブロック,
            偽のブロック,
        } => {
            let 実行対象 = if キャラ未選択フラグ {
                真のブロック
            } else {
                偽のブロック
            };

            for 子ノード in 実行対象 {
                ASTを実行(
                    子ノード,
                    commands,
                    asset_server,
                    cutscene_state,
                    state_writer,
                    キャラ未選択フラグ,
                );
            }
        }
    }
}
