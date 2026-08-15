use bevy::prelude::*;
// 必要なインポートを追加
use bevy::input::mouse::MouseMotion;

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

// カメラ移動用のコンポーネント（カメラに付けるタグ）
#[derive(Component)]
pub struct FlyCamera;

// カメラ移動システム
fn camera_movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<FlyCamera>>,
) {
    // 修正: query.single_mut() の結果を .expect() で取り出す
    let mut transform = query.single_mut().expect("FlyCameraが見つかりませんでした");
    let speed = 5.0;

    let mut direction = Vec3::ZERO;

    // 修正: transform に直接アクセスするのではなく、expectで取り出したものを使う
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= transform.forward().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyW) {
        direction += transform.forward().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= transform.right().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += transform.right().as_vec3();
    }

    transform.translation += direction.normalize_or_zero() * speed * time.delta_secs();
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
                spawn_bodybase_glbsystem,
            ),
        );
        app.add_systems(Update, camera_movement_system); // ★Updateに追加！
    }
}

// crates/yaoyorozu_beby/src/lib.rs 内
fn spawn_scene_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        // 3Dカメラを「0」に設定
        Camera {
            order: 0,
            ..default()
        },
        FlyCamera,
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
    let toki_no_kane_scene = asset_server.load("scenes/tokinokane.glb#Scene0");

    commands.spawn((
        Name::new("Kawagoe_TokiNoKane_Model"),
        WorldAssetRoot(toki_no_kane_scene),
        Transform::from_xyz(0.0, 0.0, -10.0).with_scale(Vec3::splat(1.0)),
        GlobalTransform::default(),
    ));

    println!("「時の鐘」のGLBオブジェクト（アニメなし）を配置したのだ！");
}

fn spawn_bodybase_glbsystem(mut commands: Commands, asset_server: Res<AssetServer>) {
    // assets/models/bodybase.glb を読み込む
    let bodybase_scene = asset_server.load("models/bodybase.glb#Scene0");

    commands.spawn((
        Name::new("BodyBase_Model_Entity"),
        WorldAssetRoot(bodybase_scene), // SceneRoot を使用
        Transform::from_xyz(0.0, 0.0, -30.0).with_scale(Vec3::splat(1.0)),
        Visibility::default(),
    ));

    println!("「bodybase.glb」をスポーンしたのだ！");
}
