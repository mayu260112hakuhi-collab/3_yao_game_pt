use bevy::prelude::*; // 修正: 階層操作用インポート

// 八百万駆動のゲームフローを統合したステートなのだ
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameFlowState {
    #[default]
    Title,
    CharacterSelection,
    Loading,
    Gameplay,
    Settings,
}

// 画面切り替え用のマーカーコンポーネントなのだ
#[derive(Component)]
pub struct OnMainScreen;

pub struct YamatoMainSystemRunnerPlugin;

impl Plugin for YamatoMainSystemRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameFlowState>()
            .add_systems(Startup, boot_yamato_engine_system)
            // タイトル画面関連の処理を統合なのだ
            .add_systems(OnEnter(GameFlowState::Title), setup_title_ui)
            .add_systems(
                Update,
                (menu_action_system).run_if(in_state(GameFlowState::Title)),
            )
            .add_systems(OnExit(GameFlowState::Title), despawn_screen::<OnMainScreen>)
            // 各ステートへの遷移処理なのだ
            .add_systems(
                Update,
                (
                    character_selection_logic.run_if(in_state(GameFlowState::CharacterSelection)),
                    loading_logic.run_if(in_state(GameFlowState::Loading)),
                ),
            );
    }
}

// 1. システム初期化
fn boot_yamato_engine_system() {
    println!("「八百万システムを初期化。」");
    println!("「ゲーム起動」を開始。");
}

// 2. タイトルUIの構築
fn setup_title_ui(mut commands: Commands) {
    commands.spawn((Camera2d::default(), Camera::default(), OnMainScreen));
    println!("タイトル画面UIを構築中なのだ！");
}

// 3. 欠けていたシステムを追加
fn menu_action_system(mut next_state: ResMut<NextState<GameFlowState>>) {
    // ここにボタン操作等のロジックを入れるのだ
    // とりあえず遷移だけ実装しておくのだ
    // next_state.set(GameFlowState::CharacterSelection);
}

// 4. キャラクター選択ロジック
fn character_selection_logic(mut next_state: ResMut<NextState<GameFlowState>>) {
    let character_exists = false;
    if !character_exists {
        println!("「新規キャラクター作成」を実行するのだ！");
    }
    next_state.set(GameFlowState::Loading);
}

// 5. ロード処理
fn loading_logic(mut next_state: ResMut<NextState<GameFlowState>>) {
    println!("「ロード中画像」をループ再生中…ロード完了なのだ！");
    next_state.set(GameFlowState::Gameplay);
}

// 6. 画面一括削除
fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        despawn_with_children(entity, &children, &mut commands);
    }
}

// 再帰的に削除するヘルパー関数
fn despawn_with_children(entity: Entity, children: &Query<&Children>, commands: &mut Commands) {
    if let Ok(child_entities) = children.get(entity) {
        for &child in child_entities {
            despawn_with_children(child, children, commands);
        }
    }
    commands.entity(entity).despawn();
}
