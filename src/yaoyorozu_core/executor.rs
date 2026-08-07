#![allow(non_snake_case)] // これをファイルの先頭に追加

use crate::Player;
use crate::yaoyorozu_core::command::{命令, 命令種別};
use crate::yaoyorozu_core::parser_jp::AstNode;
use bevy::prelude::*;

pub fn 命令を実行(命令: 命令, commands: &mut Commands, _asset_server: &Res<AssetServer>) {
    match 命令.動詞 {
        命令種別::ログ出力 | 命令種別::表示 => {
            info!("【八百万駆動】 {}", 命令.引数);
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
            info!("【ロード】 保持アセット: {}", 命令.引数);
        }
        命令種別::再生 => {
            info!("【再生】 ムービー/アニメーション: {}", 命令.引数);
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
    キャラ未選択フラグ: bool,
) {
    match ノード {
        AstNode::単一命令(cmd) => 命令を実行(cmd.clone(), commands, asset_server),
        AstNode::複合命令(cmd_list) => {
            for cmd in cmd_list {
                命令を実行(cmd.clone(), commands, asset_server);
            }
        }
        AstNode::条件分岐 {
            条件文,
            真のブロック,
            偽のブロック,
        } => {
            info!("【条件分岐】 評価対象: {}", 条件文);
            // 「キャラクター === なし」などの判定条件のロジック
            let 実行対象 = if キャラ未選択フラグ {
                真のブロック
            } else {
                偽のブロック
            };
            for 子ノード in 実行対象 {
                ASTを実行(子ノード, commands, asset_server, キャラ未選択フラグ);
            }
        }
    }
}
