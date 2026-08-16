use bevy::prelude::*;

pub mod command;
pub mod executor;
pub mod parser_jp;

pub struct YamatoCoreEnginePlugin;

impl Plugin for YamatoCoreEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameFlowState>()
            .add_systems(Startup, boot_yamato_engine_system);
    }
}

// ============================================================
// ゲーム全体の状態
// ============================================================

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameFlowState {
    #[default]
    Title,

    CharacterSelection,

    Loading,

    Gameplay,

    Settings,
}

// ============================================================
// 八百万コア起動
// ============================================================

fn boot_yamato_engine_system() {
    println!("「八百万システムを初期化。」");
    println!("「ゲーム起動」を開始。");
    println!("埼玉階層・川越宿場町のチャンクデータを展開中なのだ！");
}

// ============================================================
// セーブデータ
// ============================================================

#[derive(Component, Clone, Debug)]
pub struct YamatoSaveData {
    pub character_name: String,
    pub last_checkpoint: String,
    pub play_time_seconds: u64,
}

impl Default for YamatoSaveData {
    fn default() -> Self {
        Self {
            character_name: "主".to_string(),
            last_checkpoint: "川越・時の鐘下".to_string(),
            play_time_seconds: 0,
        }
    }
}
