use bevy::prelude::*;

use yaoyorozu_core::GameFlowState;

// ============================================================
// タイトルUIプラグイン
// ============================================================

pub struct YamatoTitleUiPlugin;

impl Plugin for YamatoTitleUiPlugin {
    fn build(&self, app: &mut App) {
        println!("=== YamatoTitleUiPlugin REGISTERED ===");

        app.add_systems(OnEnter(GameFlowState::Title), setup_title_ui)
            .add_systems(Update, button_system.run_if(in_state(GameFlowState::Title)))
            .add_systems(Update, menu_action.run_if(in_state(GameFlowState::Title)))
            .add_systems(
                OnExit(GameFlowState::Title),
                despawn_screen::<OnTitleScreen>,
            );
    }
}
// ============================================================
// タイトル画面マーカー
// ============================================================

#[derive(Component)]
struct OnTitleScreen;

// ============================================================
// ボタンの種類
// ============================================================

#[derive(Component)]
enum MenuButtonAction {
    Start,
    Setting,
    Exit,
}

// ============================================================
// タイトル画面生成
// ============================================================

fn setup_title_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("=== TITLE UI START ===");

    // ========================================================
    // タイトル画面専用カメラ
    // ========================================================

    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1,
            ..default()
        },
        OnTitleScreen,
    ));

    // ========================================================
    // タイトル画面ルート
    // ========================================================

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            OnTitleScreen,
        ))
        .with_children(|parent| {
            // ==================================================
            // 背景
            // ==================================================

            let background = asset_server.load("ui/image/op_title_screen.png");

            parent.spawn((
                ImageNode::new(background),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    position_type: PositionType::Absolute,

                    ..default()
                },
            ));

            // ==================================================
            // タイトルロゴ
            // ==================================================

            let title_logo = asset_server.load("ui/image/title_logo.png");

            parent.spawn((
                ImageNode::new(title_logo),
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(160.0),

                    position_type: PositionType::Absolute,

                    top: Val::Px(60.0),

                    left: Val::Px(100.0),

                    ..default()
                },
            ));

            // ==================================================
            // ボタン
            // ==================================================

            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,

                    width: Val::Px(350.0),
                    height: Val::Auto,

                    bottom: Val::Px(80.0),

                    left: Val::Px(100.0),

                    flex_direction: FlexDirection::Column,

                    align_items: AlignItems::Center,

                    row_gap: Val::Px(12.0),

                    ..default()
                },))
                .with_children(|parent| {
                    spawn_menu_button(
                        parent,
                        &asset_server,
                        "ui/image/start_txr.png",
                        MenuButtonAction::Start,
                    );

                    spawn_menu_button(
                        parent,
                        &asset_server,
                        "ui/image/setting_txt.png",
                        MenuButtonAction::Setting,
                    );

                    spawn_menu_button(
                        parent,
                        &asset_server,
                        "ui/image/endgame_txt.png",
                        MenuButtonAction::Exit,
                    );
                });
        });

    println!("=== TITLE UI CREATED ===");
}

// ============================================================
// ボタン生成
// ============================================================

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    asset_server: &Res<AssetServer>,
    image_path: &'static str,
    action: MenuButtonAction,
) {
    let image = asset_server.load(image_path);

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(300.0),
                height: Val::Px(70.0),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                ..default()
            },
            BackgroundColor(Color::NONE.into()),
            action,
            OnTitleScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(image),
                Node {
                    width: Val::Px(260.0),
                    height: Val::Px(60.0),
                    ..default()
                },
            ));
        });
}

// ============================================================
// ボタンの見た目
// ============================================================

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::srgb(0.35, 0.35, 0.40).into();
            }

            Interaction::Hovered => {
                *background_color = Color::srgb(0.25, 0.25, 0.30).into();
            }

            Interaction::None => {
                *background_color = Color::srgb(0.12, 0.12, 0.15).into();
            }
        }
    }
}

// ============================================================
// ボタン処理
// ============================================================

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameFlowState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            MenuButtonAction::Start => {
                println!("=== GAME START ===");

                next_state.set(GameFlowState::CharacterSelection);
            }

            MenuButtonAction::Setting => {
                println!("=== SETTING ===");
            }

            MenuButtonAction::Exit => {
                println!("=== GAME EXIT ===");

                app_exit.write(AppExit::Success);
            }
        }
    }
}

// ============================================================
// タイトル画面削除
// ============================================================

fn despawn_screen<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
