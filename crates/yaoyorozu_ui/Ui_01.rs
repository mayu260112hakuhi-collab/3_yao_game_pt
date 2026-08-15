use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "川越の始まりの街 - UI Mock".into(),
                resolution: (1280.0, 720.0).into(),
                ..default::default()
            }),
            ..default::default()
        }))
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2Dカメラの生成
    commands.spawn(Camera2dBundle::default());

    // ルートノード（画面全体を覆うコンテナ）
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default::default()
        })
        .with_children(|parent| {
            // ==========================================
            // 上部エリア (プレイヤー情報、エリア名、ミニマップ)
            // ==========================================
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Start,
                    ..default::default()
                },
                ..default::default()
            }).with_children(|top| {
                // 【左上】プレイヤー ＆ パーティ情報
                top.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Px(250.0),
                        row_gap: Val::Px(10.0),
                        ..default::default()
                    },
                    ..default::default()
                }).with_children(|player_section| {
                    // メインプレイヤー
                    spawn_player_card(player_section, "プレイヤーネーム LV * *", Color::srgb(0.6, 0.9, 0.6));
                    // パーティメンバー（3人分）
                    spawn_party_member(player_section, "プレイヤーネーム");
                    spawn_party_member(player_section, "プレイヤーネーム");
                    spawn_party_member(player_section, "プレイヤーネーム");
                });

                // 【中央上】エリア名とサブテキスト
                top.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default::default()
                    },
                    ..default::default()
                }).with_children(|area| {
                    area.spawn(TextBundle::from_section(
                        "川越の始まりの街",
                        TextStyle {
                            font_size: 28.0,
                            color: Color::srgb(0.2, 0.1, 0.1),
                            ..default::default()
                        },
                    ));
                    area.spawn(TextBundle::from_section(
                        "運河と陸路が交わるこの日は、\n人と物と情報が集まる、賑わいの街である",
                        TextStyle {
                            font_size: 12.0,
                            color: Color::srgb(0.3, 0.2, 0.2),
                            ..default::default()
                        },
                    ));
                });

                // 【右上】ミニマップ ＆ クエスト情報
                top.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        row_gap: Val::Px(10.0),
                        ..default::default()
                    },
                    ..default::default()
                }).with_children(|right_section| {
                    // ミニマップ（円形風の四角ボックス）
                    right_section.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(140.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default::default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.8, 0.8, 0.7)),
                        border_color: BorderColor(Color::srgb(0.5, 0.4, 0.3)),
                        ..default::default()
                    });

                    // クエスト・目的表示
                    right_section.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(220.0),
                            height: Val::Px(70.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default::default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.95, 0.95, 0.93, 0.9)),
                        border_color: BorderColor(Color::srgb(0.8, 0.8, 0.7)),
                        ..default::default()
                    }).with_children(|quest| {
                        quest.spawn(TextBundle::from_section(
                            "目的：宿場へ向かう(1/2)",
                            TextStyle { font_size: 12.0, color: Color::BLACK, ..default::default() },
                        ));
                        quest.spawn(TextBundle::from_section(
                            "クエスト：川越の鐘を鳴らす",
                            TextStyle { font_size: 10.0, color: Color::DARK_GRAY, ..default::default() },
                        ));
                    });
                });
            });

            // ==========================================
            // 下部エリア (チャット欄 ＆ ホットバー)
            // ==========================================
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::End,
                    ..default::default()
                },
                ..default::default()
            }).with_children(|bottom| {
                // 【左下】チャット・ログウィンドウ
                bottom.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(260.0),
                        height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default::default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.95, 0.95, 0.93, 0.8)),
                    border_color: BorderColor(Color::srgb(0.5, 0.5, 0.4)),
                    ..default::default()
                }).with_children(|chat| {
                    // タブ部分 (世界 パーティ ギルド システム)
                    chat.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(25.0),
                            column_gap: Val::Px(8.0),
                            padding: UiRect::horizontal(Val::Px(5.0)),
                            align_items: AlignItems::Center,
                            ..default::default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.4, 0.5, 0.4)),
                        ..default::default()
                    }).with_children(|tabs| {
                        for name in &["世界", "パーティ", "ギルド", "システム"] {
                            tabs.spawn(TextBundle::from_section(
                                *name,
                                TextStyle { font_size: 11.0, color: Color::WHITE, ..default::default() },
                            ));
                        }
                    });

                    // ログ表示エリア（空のボックス）
                    chat.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            ..default::default()
                        },
                        ..default::default()
                    });
                });

                // 【下部中央〜右】ホットバー（アイテムスロット群）
                bottom.spawn(NodeBundle {
                    style: Style {
                        column_gap: Val::Px(8.0),
                        align_items: AlignItems::Center,
                        ..default::default()
                    },
                    ..default::default()
                }).with_children(|hotbar| {
                    // 左側の特殊スロット
                    spawn_slot(hotbar, Color::srgb(0.6, 0.9, 0.6));
                    
                    // 中央の6個連続スロット
                    for _ in 0..6 {
                        spawn_slot(hotbar, Color::srgb(0.9, 0.9, 0.88));
                    }

                    // 水色の特殊スロット
                    spawn_slot(hotbar, Color::srgb(0.6, 0.9, 1.0));

                    // 少し間隔を空けて右側のスロット群
                    hotbar.spawn(NodeBundle {
                        style: Style { width: Val::Px(20.0), ..default::default() },
                        ..default::default()
                    });

                    for _ in 0..6 {
                        spawn_slot(hotbar, Color::srgb(0.9, 0.9, 0.88));
                    }
                });
            });
        });
}

// プレイヤーカードを生成するヘルパー関数
fn spawn_player_card(parent: &mut ChildBuilder, name: &str, icon_color: Color) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(35.0),
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default::default()
        },
        ..default::default()
    }).with_children(|card| {
        // アイコン用の小さなボックス
        card.spawn(NodeBundle {
            style: Style { width: Val::Px(20.0), height: Val::Px(20.0), border: UiRect::all(Val::Px(1.0)) },
            background_color: BackgroundColor(icon_color),
            border_color: BorderColor(Color::BLACK),
            ..default::default()
        });
        // 名前とステータスバー風の表現
        card.spawn(TextBundle::from_section(
            name,
            TextStyle { font_size: 12.0, color: Color::BLACK, ..default::default() },
        ));
    });
}

// パーティメンバー用の簡易表示
fn spawn_party_member(parent: &mut ChildBuilder, name: &str) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(25.0),
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default::default()
        },
        ..default::default()
    }).with_children(|card| {
        card.spawn(NodeBundle {
            style: Style { width: Val::Px(15.0), height: Val::Px(15.0) },
            background_color: BackgroundColor(Color::srgb(0.6, 0.9, 0.6)),
            ..default::default()
        });
        card.spawn(TextBundle::from_section(
            name,
            TextStyle { font_size: 10.0, color: Color::DARK_GRAY, ..default::default() },
        ));
    });
}

// アイテムスロットを生成するヘルパー関数
fn spawn_slot(parent: &mut ChildBuilder, bg_color: Color) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(36.0),
            height: Val::Px(36.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default::default()
        },
        background_color: BackgroundColor(bg_color),
        border_color: BorderColor(Color::srgb(0.7, 0.7, 0.6)),
        ..default::default()
    });
}