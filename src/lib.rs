// yaoyorozu_core/src/lib.rs - 八百万エンジン コアモジュール

use bevy::prelude::*;

pub const BLOCK_UNIT_SIZE: f32 = 0.33333; // 1/3メートル基準
pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 64;
pub const CHUNK_DEPTH: usize = 16;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VoxelType {
    Air,
    Tatami,
    StoneWall,
    WoodLog,
    ClayRoof,
}

#[derive(Component, Clone)]
pub struct VoxelChunk {
    pub blocks: [[[VoxelType; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
}

impl Default for VoxelChunk {
    fn default() -> Self {
        Self {
            blocks: [[[VoxelType::Air; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
        }
    }
}

pub struct YamatoCorePlugin;

impl Plugin for YamatoCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_saitama_kawagoe_stage);
    }
}

fn init_saitama_kawagoe_stage(mut commands: Commands) {
    let mut chunk = VoxelChunk::default();
    for x in 0..CHUNK_WIDTH {
        for z in 0..CHUNK_DEPTH {
            chunk.blocks[x][0][z] = VoxelType::StoneWall;
        }
    }

    commands.spawn((
        Name::new("Yamato_Saitama_Kawagoe_Foundation"),
        chunk,
        Transform::default(),
        GlobalTransform::default(),
    ));
}
