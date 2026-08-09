use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameFlowState {
    #[default]
    Title,
    Gameplay,
    Loading,
}

#[derive(Component)]
pub struct SelectedCharacter {
    pub name: String,
    pub last_checkpoint: String,
}

pub struct YamatoMainSystemRunnerPlugin;

impl Plugin for YamatoMainSystemRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameFlowState>()
            .add_systems(Update, check_character_status_system);
    }
}

fn check_character_status_system(query: Query<&SelectedCharacter>) {
    for character in query.iter() {
        if character.name.is_empty() {
            println!("キャラクター名が未設定なのだ！");
        } else {
            println!("現在のチェックポイント: {}", character.last_checkpoint);
        }
    }
}
