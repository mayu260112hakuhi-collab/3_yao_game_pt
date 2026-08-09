// src/main.rs - 八百万エンジン 起動エントリーポイント（全張替版）

use bevy::prelude::*;

// ツリー構造に合わせたモジュール宣言
mod yamato_crafttowers;
mod yaoyorozu_beby;
mod yaoyorozu_core;
mod yaoyorozu_ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 各コア・拡張プラグインを統合登録
        .add_plugins((
            yaoyorozu_core::YamatoCoreEnginePlugin,
            yaoyorozu_beby::YaoyorozuBevyBundlePlugin,
            yaoyorozu_ui::YamatoUiPlugin,
        ))
        .run();
}
