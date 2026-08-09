// yaoyorozu_ui/src/lib.rs - 八百万駆動 UI・独自スクリプト管理層

use bevy::prelude::*;

pub struct YamatoUiPlugin;

impl Plugin for YamatoUiPlugin {
    fn build(&self, app: &App) {
        app.add_systems(Startup, load_yamato_script_system);
    }
}

// 独自言語「八百万スクリプト（.8g）」のメインシステム初期化をシミュレート
fn load_yamato_script_system() {
    println!("「mainSystem.8g」のスクリプト構文解析を完了したのだ！");
    println!("防御全振り・クラス無しのステータスツリーを展開中……");
}
