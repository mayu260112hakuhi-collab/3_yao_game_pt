use bevy::prelude::*;

pub mod engine_bridge;
pub mod main_system;
pub mod save_system;

pub use engine_bridge::*;
pub use main_system::*;
pub use save_system::*;

pub struct YaoyorozuBevyBundlePlugin;

impl Plugin for YaoyorozuBevyBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YaoyorozuBridgePlugin,
            YamatoMainSystemRunnerPlugin,
            YamatoBebyPlugin,
        ));
    }
}

pub struct YamatoBebyPlugin;

impl Plugin for YamatoBebyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_scene_camera_system,
                spawn_directional_light_system,
                spawn_toki_no_kane_glbsystem,
            ),
        );
    }
}

fn spawn_scene_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_directional_light_system(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// アニメーションなしのテスト用.glb（時の鐘）をスポーンするシステム
fn spawn_toki_no_kane_glbsystem(mut commands: Commands, asset_server: Res<AssetServer>) {
    let toki_no_kane_scene = asset_server.load("scenes/start-movie01.glb#Scene0");

    commands.spawn((
        Name::new("Kawagoe_TokiNoKane_Model"),
        WorldAssetRoot(toki_no_kane_scene),
        Transform::from_xyz(0.0, 0.0, -10.0).with_scale(Vec3::splat(1.0)),
        GlobalTransform::default(),
    ));

    println!("「時の鐘」のGLBオブジェクト（アニメなし）を配置したのだ！");
}
