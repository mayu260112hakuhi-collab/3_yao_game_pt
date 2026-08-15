use bevy::prelude::*;

// ゲームの状態を管理するステートなのだ
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Title,
    Playing,
}

// ボタンの種類を判別するためのコンポーネントなのだ
#[derive(Component)]
enum MenuButtonAction {
    Start,
    Setting,
    Exit,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "八百万駆動 - 川越の始まりの街".into(),
                resolution: (1920.0, 1080.0).into(), // FHD設定なのだ！
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Title), setup_title_ui)
        .add_systems(
            Update,
            (button_system, menu_action).run_if(in_state(GameState::Title)),
        )
        .add_systems(OnExit(GameState::Title), despawn_screen::<OnTitleScreen>)
        .run();
}

// タイトル画面の要素に付けるマーカーコンポーネントなのだ
#[derive(Component)]
struct OnTitleScreen;

// タイトル画面のUIを構築するシステムなのだ
fn setup_title_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // カメラの生成（UI表示に必要だぞ）
    commands.spawn((Camera2dBundle::default(), OnTitleScreen));

    // 背景や全体のルートレイアウト（中央揃えの縦並び）なのだ
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OnTitleScreen,
        ))
        .with_children(|parent| {
            // 1. タイトルロゴ画像なのだ
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("ui/image/title_logo.png")),
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(450.0),
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
                ..default()
            });

            // 2. 「始める」ボタンなのだ
            spawn_menu_button(
                parent,
                &asset_server,
                "ui/image/start_txr.png",
                MenuButtonAction::Start,
            );

            // 3. 「設定」ボタンなのだ
            spawn_menu_button(
                parent,
                &asset_server,
                "ui/image/setting_txt.png",
                MenuButtonAction::Setting,
            );

            // 4. 「終了」ボタンなのだ
            spawn_menu_button(
                parent,
                &asset_server,
                "ui/image/endgame_txt.png",
                MenuButtonAction::Exit,
            );
        });
}

// ボタンを生成するヘルパー関数なのだ
fn spawn_menu_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    image_path: &str,
    action: MenuButtonAction,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(280.0),
                    height: Val::Px(75.0),
                    margin: UiRect::all(Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            action,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load(image_path)),
                style: Style {
                    width: Val::Px(210.0),
                    height: Val::Px(55.0),
                    ..default()
                },
                ..default()
            });
        });
}

// ボタンのホバー演出を管理するシステムなのだ
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.into();
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color::DARK_GRAY.into();
            }
        }
    }
}

// ボタンが押されたときのアクションを実行するシステムなのだ
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Start => {
                    next_state.set(GameState::Playing);
                }
                MenuButtonAction::Setting => {
                    println!("設定ボタンが押されたのだ！");
                }
                MenuButtonAction::Exit => {
                    exit.send(bevy::app::AppExit::Success);
                }
            }
        }
    }
}

// 画面を切り替えるときに古いUIを一括削除するための汎用システムなのだ
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
