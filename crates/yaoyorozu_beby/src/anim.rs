// Bevyでのアニメーション再生例
use bevy::prelude::*;

#[derive(Component)]
struct PlayerAnimations {
    idle: Handle<AnimationClip>,
    walk: Handle<AnimationClip>,
    run: Handle<AnimationClip>,
}

fn play_character_movement(
    mut players: Query<&mut AnimationPlayer>,
    anim_assets: Res<PlayerAnimations>,
) {
    for mut player in &mut players {
        // Blender側で設定したexport_nameに合わせて再生
        player.play(anim_assets.walk.clone_weak()).repeat();
    }
}
