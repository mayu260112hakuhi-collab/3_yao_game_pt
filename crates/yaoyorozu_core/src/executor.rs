#![allow(non_snake_case)]

use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;

use crate::command::{値, 文, 命令, 命令引数, 命令種別};
use crate::runtime::八百万実行環境;

#[derive(Message)]
pub struct RequestStateTransition(pub String);

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

#[derive(Resource, Debug, Clone, Default)]
pub struct MovieCutsceneState {
    pub is_playing: bool,
    pub current_bg_image: String,
    pub calligraphy_text: String,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct 八百万プログラム {
    pub 文一覧: Vec<文>,
    pub 実行済み: bool,
}

pub fn プログラムを実行(
    mut program: ResMut<八百万プログラム>,
    mut runtime: ResMut<八百万実行環境>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cutscene_state: ResMut<MovieCutsceneState>,
    mut state_writer: MessageWriter<RequestStateTransition>,
) {
    if program.実行済み || program.文一覧.is_empty() {
        return;
    }

    let 文一覧 = program.文一覧.clone();
    for 文 in &文一覧 {
        if let Err(err) = 文を実行(
            文,
            &mut runtime,
            &mut commands,
            &asset_server,
            &mut cutscene_state,
            &mut state_writer,
        ) {
            error!("【8g実行エラー】 {err}");
            return;
        }
    }

    program.実行済み = true;
    info!("【8g】起動スクリプトの実行が完了しました");
}

fn 文を実行(
    文: &文,
    runtime: &mut 八百万実行環境,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    cutscene_state: &mut ResMut<MovieCutsceneState>,
    state_writer: &mut MessageWriter<RequestStateTransition>,
) -> Result<(), String> {
    match 文 {
        文::代入 { 変数名, 値 } => {
            let 評価値 = runtime.式を評価(値)?;
            runtime.変数を設定(変数名.clone(), 評価値);
        }
        文::命令(命令) => {
            命令を実行(
                命令,
                runtime,
                commands,
                asset_server,
                cutscene_state,
                state_writer,
            )?;
        }
        文::条件分岐 {
            条件,
            真のブロック,
            偽のブロック,
        } => {
            let 条件値 = runtime.式を評価(条件)?;
            let 実行対象 = if 条件値.真か() {
                真のブロック
            } else {
                偽のブロック
            };
            for 子 in 実行対象 {
                文を実行(
                    子,
                    runtime,
                    commands,
                    asset_server,
                    cutscene_state,
                    state_writer,
                )?;
            }
        }
        文::複合(文一覧) => {
            for 子 in 文一覧 {
                文を実行(
                    子,
                    runtime,
                    commands,
                    asset_server,
                    cutscene_state,
                    state_writer,
                )?;
            }
        }
    }

    Ok(())
}

fn 命令を実行(
    命令: &命令,
    runtime: &八百万実行環境,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    cutscene_state: &mut ResMut<MovieCutsceneState>,
    state_writer: &mut MessageWriter<RequestStateTransition>,
) -> Result<(), String> {
    let 評価済み = 引数を評価(&命令.引数, runtime)?;
    let 主値 = 助詞値(&評価済み, "を")
        .or_else(|| 評価済み.last().map(|(_, v)| v))
        .cloned()
        .unwrap_or(値::なし);

    match &命令.動詞 {
        命令種別::ログ出力 => {
            info!("【8g】{}", 主値);
        }
        命令種別::表示 => {
            let text = 主値.表示文字列();
            info!("【8g表示】{}", text);
            cutscene_state.calligraphy_text = text;
        }
        命令種別::初期化 => {
            info!("【8g初期化】{}", 主値);
        }
        命令種別::開始 => {
            info!("【8g開始】{}", 主値);
        }
        命令種別::実行 => {
            if 主値.表示文字列() == "新規キャラクター作成" {
                commands.spawn((
                    Player {
                        name: "プレイヤー1".to_string(),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
            } else {
                info!("【8g実行】{}", 主値);
            }
        }
        命令種別::スポーン => {
            let 名前 = 主値.表示文字列();
            let 場所 = 助詞値(&評価済み, "に")
                .map(|v| v.表示文字列())
                .unwrap_or_default();
            info!("【8gスポーン】{} -> {}", 名前, 場所);
            commands.spawn((
                Player {
                    name: 名前,
                    ..default()
                },
                Transform::from_xyz(0.0, 1.0, 0.0),
            ));
        }
        命令種別::ロード | 命令種別::読み込み => {
            let 対象 = 主値.表示文字列();
            info!("【8gロード】{}", 対象);

            if 対象.contains("タイトル画面") {
                state_writer.write(RequestStateTransition("Title".to_string()));
            } else {
                let path = format!("scenes/{対象}.glb");
                let scene_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(path));
                commands.spawn((WorldAssetRoot(scene_handle), Transform::default()));
            }
        }
        命令種別::移動 | 命令種別::遷移 => {
            let 対象 = 助詞値(&評価済み, "へ")
                .or_else(|| 評価済み.last().map(|(_, v)| v))
                .map(|v| v.表示文字列())
                .unwrap_or_default();
            state_writer.write(RequestStateTransition(対象));
        }
        命令種別::再生 | 命令種別::ループ再生 => {
            cutscene_state.is_playing = true;
            cutscene_state.current_bg_image = 主値.表示文字列();
        }
        命令種別::選択 => {
            info!("【8g選択】{}", 主値);
        }
        命令種別::格納 => {
            info!("【8g格納】{}", 主値);
        }
        命令種別::終了 => {
            info!("【8g終了要求】{}", 主値);
        }
        命令種別::その他(名前) => {
            warn!("【8g未実装命令】{} / 引数={:?}", 名前, 評価済み);
        }
        その他 => {
            warn!("【8g未実装命令】{:?} / 引数={:?}", その他, 評価済み);
        }
    }

    Ok(())
}

fn 引数を評価(
    引数: &[命令引数],
    runtime: &八百万実行環境,
) -> Result<Vec<(Option<String>, 値)>, String> {
    引数.iter()
        .map(|引数| {
            Ok((
                引数.助詞.clone(),
                runtime.式を評価(&引数.値)?,
            ))
        })
        .collect()
}

fn 助詞値<'a>(引数: &'a [(Option<String>, 値)], 助詞: &str) -> Option<&'a 値> {
    引数.iter()
        .rev()
        .find(|(p, _)| p.as_deref() == Some(助詞))
        .map(|(_, v)| v)
}
