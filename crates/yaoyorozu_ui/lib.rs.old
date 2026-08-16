// crates/yaoyorozu_ui/lib.rs
use bevy::prelude::*;
pub mod ui_optitle;
use ui_optitle::YamatoTitleUiPlugin; // ここでインポート

pub struct YamatoUiPlugin;

impl Plugin for YamatoUiPlugin {
    fn build(&self, app: &mut App) {
        // ここでタイトルUIのプラグインも一緒に登録するのだ！
        app.add_plugins(YamatoTitleUiPlugin);
        app.add_systems(Startup, (load_yamato_script_system, setup_camera_system));
    }
}

// 独自言語「八百万スクリプト（.8g）」のメインシステム初期化をシミュレート
fn load_yamato_script_system() {
    println!("「mainSystem.8g」のスクリプト構文解析を完了したのだ！");
    println!("防御全振り・クラス無しのステータスツリーを展開中……");
}

// UI表示用のカメラをセットアップするのだ
// crates/yaoyorozu_ui/lib.rs 内
fn setup_camera_system(mut commands: Commands) {
    // 競合するのでコメントアウトするのだ！
    // commands.spawn(Camera2dBundle::default());
    println!("「UI用のカメラは ui_optitle.rs で管理するためスキップしたのだ！」");
}
