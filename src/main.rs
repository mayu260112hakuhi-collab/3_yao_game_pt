// src/main.rs - 八百万エンジン 起動エントリーポイント

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            yaoyorozu_core::YamatoCoreEnginePlugin,
            yaoyorozu_beby::YaoyorozuBevyBundlePlugin,
            yaoyorozu_ui::YamatoUiPlugin,
        ))
        .run();
}
