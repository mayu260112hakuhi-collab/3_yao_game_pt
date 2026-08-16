use bevy::prelude::*;

pub mod ui_optitle;

pub struct YamatoUiPlugin;

impl Plugin for YamatoUiPlugin {
    fn build(&self, app: &mut App) {
        println!("=== YamatoUiPlugin REGISTERED ===");

        app.add_plugins(ui_optitle::YamatoTitleUiPlugin)
            .add_systems(Startup, load_yamato_script_system);
    }
}

fn load_yamato_script_system() {
    println!("「mainSystem.8g」のスクリプト構文解析を完了したのだ！");
    println!("防御全振り・クラス無しのステータスツリーを展開中……");
}
