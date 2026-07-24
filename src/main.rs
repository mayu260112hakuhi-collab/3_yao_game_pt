use bevy::prelude::*;
use bevy_skein::SkeinPlugin;

fn main() {
    App::new()
        // Bevyの型レジストリにPlayer構造体を登録（Skeinがこれを読み取りBlender等と連携します）
        .register_type::<Player>()
        .add_plugins((
            DefaultPlugins,
            SkeinPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

// Blender側でのデフォルト値適用やプリセット連携を円滑にするため Default を導出
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
struct Player {
    name: String,
    power: f32,
    test: i32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 3Dカメラの配置
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ライトの配置
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Skein経由でBlenderからエクスポートしたglTF/glbシーンをロードすると、
    // Blender側で付与した Player コンポーネントが自動的にアタッチされた状態で生成されます
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/test.glb")),
    ));
}