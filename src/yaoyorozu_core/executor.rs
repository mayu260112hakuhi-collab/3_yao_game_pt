#![allow(non_snake_case)]

use crate::Player;
use crate::yaoyorozu_core::command::{命令, 命令種別};
use crate::yaoyorozu_core::parser_jp::AstNode;
use bevy::prelude::*;

/// MMO風のあらすじ演出やムービー状態を管理するBevyリソース
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
                        power: 100.0,
                        test: 1,
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
            } else {
                info!("【実行】 {}", 命令.引数);
            }
        }
        命令種別::ロード => {
            info!("【ロード】 3Dモデル/アセット読み込み: {}", 命令.引数);
            let path = format!("scenes/{}.glb#Scene0", 命令.引数);

            // Bevy 0.19 の新仕様に合わせて WorldAssetRoot を使用します
            commands.spawn((
                WorldAssetRoot(asset_server.load(&path)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        }
        命令種別::再生 => {
            info!("【MMOムービー再生開始】 背景イメージ/音楽: {}", 命令.引数);
            cutscene_state.is_playing = true;
            cutscene_state.current_bg_image = 命令.引数.clone();
        }
        命令種別::選択 => {
            info!("【選択】 オブジェクト: {}", 命令.引数);
        }
        命令種別::移動 => {
            info!("【移動】 シーン/座標: {}", 命令.引数);
        }
        命令種別::その他(詳細) => {
            info!("【汎用命令】 {}: {}", 詳細, 命令.引数);
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
    キャラ未選択フラグ: bool,
) {
    match ノード {
        AstNode::単一命令(cmd) => {
            命令を実行(cmd.clone(), commands, asset_server, cutscene_state)
        }
        AstNode::複合命令(cmd_list) => {
            for cmd in cmd_list {
                命令を実行(cmd.clone(), commands, asset_server, cutscene_state);
            }
        }
        AstNode::条件分岐 {
            条件文,
            真のブロック,
            偽のブロック,
        } => {
            info!("【条件分岐】 評価対象: {}", 条件文);
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
                    キャラ未選択フラグ,
                );
            }
        }
    }
}
