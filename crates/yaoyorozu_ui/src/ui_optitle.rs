use bevy::prelude::*;

use yaoyorozu_core::GameFlowState;

pub struct YamatoTitleUiPlugin;

impl Plugin for YamatoTitleUiPlugin {
    fn build(&self, app: &mut App) {
        println!("=== YamatoTitleUiPlugin REGISTERED ===");

        app.add_systems(OnEnter(GameFlowState::Title), setup_title_ui)
            .add_systems(
                OnExit(GameFlowState::Title),
                despawn_screen::<OnTitleScreen>,
            );
    }
}

#[derive(Component)]
struct OnTitleScreen;

fn setup_title_ui(mut commands: Commands) {
    println!("=== TITLE UI TEST ===");

    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1,
            ..default()
        },
        OnTitleScreen,
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 0.0, 0.0).into()),
        OnTitleScreen,
    ));
}

fn despawn_screen<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
