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

// ============================================================
// カメラ移動
// ============================================================

/// カメラ移動用コンポーネント
#[derive(Component)]
pub struct FlyCamera;

/// カメラ移動システム
fn camera_movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<FlyCamera>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let speed = 5.0;
    let mut direction = Vec3::ZERO;

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

// ============================================================
// Bevyメインプラグイン
// ============================================================

pub struct YamatoBebyPlugin;

impl Plugin for YamatoBebyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameFlowState::Loading),
            (
                spawn_scene_camera_system,
                spawn_directional_light_system,
                spawn_toki_no_kane_glbsystem,
                spawn_bodybase_glbsystem,
            ),
        );

        app.add_systems(Update, camera_movement_system);
    }
}

// ============================================================
// カメラ
// ============================================================

fn spawn_scene_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        FlyCamera,
        Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// ============================================================
// ライト
// ============================================================

fn spawn_directional_light_system(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// ============================================================
// 時の鐘
// Bevy 0.19対応
// ============================================================

fn spawn_toki_no_kane_glbsystem(mut commands: Commands, asset_server: Res<AssetServer>) {
    let toki_no_kane_scene =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/tokinokane.glb"));

    commands.spawn((
        Name::new("Kawagoe_TokiNoKane_Model"),
        WorldAssetRoot(toki_no_kane_scene),
        Transform::from_xyz(0.0, 0.0, -10.0).with_scale(Vec3::splat(1.0)),
    ));

    println!("「時の鐘」のGLBオブジェクト（アニメなし）を配置したのだ！");
}

// ============================================================
// bodybase
// Bevy 0.19対応
// ============================================================

fn spawn_bodybase_glbsystem(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bodybase_scene =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/bodybase.glb"));

    commands.spawn((
        Name::new("BodyBase_Model_Entity"),
        WorldAssetRoot(bodybase_scene),
        Transform::from_xyz(0.0, 0.0, -30.0).with_scale(Vec3::splat(1.0)),
        Visibility::default(),
    ));

    println!("「bodybase.glb」をスポーンしたのだ！");
}
