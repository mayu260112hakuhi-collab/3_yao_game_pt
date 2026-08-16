// mod.rs - 八百万駆動 統合モジュール

use bevy::prelude::*;

// 先ほど配置したボクセルマップ設定をインポート
pub mod voxelmap_settings;
use voxelmap_settings::YamatoVoxelMapPlugin;

// 八百万エンジンのメインシステムを管理するプラグイン
pub struct YamatoCoreEnginePlugin;

impl Plugin for YamatoCoreEnginePlugin {
    // ここを &mut App に修正しないとエラーになるのだ！
    fn build(&self, app: &mut App) {
        // エンジン起動時のログ出力と初期化処理
        // 合わせて .glb の読み込みシステムも登録するのだ！
        app.add_plugins(YamatoVoxelMapPlugin).add_systems(
            Startup,
            (boot_yamato_engine_system, load_start_movie_system),
        );
    }
}

// 「mainSystem.8g」の起動フローに対応するRust側の初期化システム
fn boot_yamato_engine_system() {
    println!("「八百万システムを初期化。」");
    println!("「ゲーム起動」を開始。");
    println!("埼玉階層・川越宿場町のチャンクデータを展開中なのだ！");
}

// ここで .glb を読み込むのだ！
fn load_start_movie_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("「scenes/start-movie01.glb」の読み込みシステムが発動したのだ！");

    // glbファイルを指定してハンドルを取得するのだ
    let scene_handle = asset_server.load("scenes/tokinokane.glb#Scene0");

    // SceneRootを使ってエンティティをスポーンするのだ
    commands.spawn((
        Name::new("Yamato_Start_Movie_Entity"),
        SceneRoot(scene_handle), // ← SceneRoot から戻す
        Transform::default(),
        GlobalTransform::default(),
    ));
}

// セーブデータを管理する構造体
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
