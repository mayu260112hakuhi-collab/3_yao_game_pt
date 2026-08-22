// crates/yaoyorozu_beby/src/main_system.rs

use bevy::ecs::message::MessageReader;
use bevy::prelude::*;

use yaoyorozu_core::executor::RequestStateTransition;

// ============================================================
// 八百万駆動・ゲームフロー
// ============================================================

pub use yaoyorozu_core::GameFlowState;

// ============================================================
// 画面切り替え用マーカー
// ============================================================

#[derive(Component)]
pub struct OnMainScreen;

// ============================================================
// 八百万メインシステムプラグイン
// ============================================================

pub struct YamatoMainSystemRunnerPlugin;

impl Plugin for YamatoMainSystemRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
            // ゲームフローを初期化
            .init_state::<GameFlowState>()
            // 八百万システム起動
            //.add_systems(Startup, boot_yamato_engine_system)
            // タイトル画面
            // タイトル画面中の操作
            .add_systems(
                Update,
                menu_action_system.run_if(in_state(GameFlowState::Title)),
            )
            // タイトル画面を出るときに削除
            .add_systems(OnExit(GameFlowState::Title), despawn_screen::<OnMainScreen>)
            // 八百万コアからの状態遷移要求
            .add_systems(Update, state_transition_listener);
        // キャラクター選択
        //.add_systems(               Update,                character_selection_logic.run_if(in_state(GameFlowState::CharacterSelection)),            )
        // ロード処理
        //.add_systems(
        // Update,
        // loading_logic.run_if(in_state(GameFlowState::Loading)),
        //);
    }
}

// ============================================================
// 八百万コア → Bevy
//
// Bevy 0.19では MessageReader を Option で包んで
// 未初期化時のパニックを防ぐのだ！
// ============================================================

fn state_transition_listener(
    events: Option<MessageReader<RequestStateTransition>>,
    mut next_state: ResMut<NextState<GameFlowState>>,
) {
    // メッセージリーダーがまだ初期化されていない場合は安全にスキップ
    let Some(mut events) = events else {
        return;
    };

    for event in events.read() {
        println!("【八百万状態遷移要求】 {}", event.0);

        match event.0.as_str() {
            "Title" => {
                println!("【遷移】 タイトル画面へ遷移中...");

                next_state.set(GameFlowState::Title);
            }

            "CharacterSelection" => {
                println!("【遷移】 キャラクター選択画面へ遷移中...");

                next_state.set(GameFlowState::CharacterSelection);
            }

            "Loading" => {
                println!("【遷移】 ロード画面へ遷移中...");

                next_state.set(GameFlowState::Loading);
            }

            "Gameplay" => {
                println!("【遷移】 ゲームプレイへ遷移中...");

                next_state.set(GameFlowState::Gameplay);
            }

            "Settings" => {
                println!("【遷移】 設定画面へ遷移中...");

                next_state.set(GameFlowState::Settings);
            }

            その他 => {
                warn!("【八百万状態遷移】 未知の状態: {}", その他);
            }
        }
    }
}

// ============================================================
// 1. タイトルメニュー操作
// ============================================================

fn menu_action_system(_next_state: ResMut<NextState<GameFlowState>>) {
    // ここにタイトル画面のボタン操作を追加する。
}

// ============================================================
// 4. キャラクター選択
// ============================================================

fn character_selection_logic(mut next_state: ResMut<NextState<GameFlowState>>) {
    let character_exists = false;

    if !character_exists {
        println!("「新規キャラクター作成」を実行するのだ！");
    }

    next_state.set(GameFlowState::Loading);
}

// ============================================================
// 5. ロード処理
// ============================================================

fn loading_logic(mut next_state: ResMut<NextState<GameFlowState>>) {
    println!("「ロード中画像」をループ再生中…ロード完了なのだ！");

    next_state.set(GameFlowState::Gameplay);
}

// ============================================================
// 6. 画面一括削除
// ============================================================

fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        despawn_with_children(entity, &children, &mut commands);
    }
}

// ============================================================
// 再帰的に子エンティティを削除
// ============================================================

fn despawn_with_children(entity: Entity, children: &Query<&Children>, commands: &mut Commands) {
    if let Ok(child_entities) = children.get(entity) {
        for child in child_entities.iter() {
            despawn_with_children(child, children, commands);
        }
    }

    commands.entity(entity).despawn();
}
