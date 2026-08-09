// voxelmapSettings.rs - 八百万駆動 ボクセルマップ設定モジュール

use bevy::prelude::*;

// 1ブロックの実世界メートル換算（約0.333m = 1/3m）
// 身長180cmのキャラクターが約5.4ブロック分の高さになる基準値なのだ。
pub const BLOCK_UNIT_SIZE: f32 = 0.33333;

// 1チャンクあたりのサイズ定義（埼玉階層・宿場町用グリッド）
pub const CHUNK_WIDTH: usize = 16; // X軸方向のブロック数
pub const CHUNK_HEIGHT: usize = 64; // Y軸方向（高さ）のブロック数（時の鐘なども収まる高さ）
pub const CHUNK_DEPTH: usize = 16; // Z軸方向のブロック数

// ボクセルブロックの種類を定義する列挙型
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VoxelType {
    Air,       // 空気（何もない状態）
    Tatami,    // 畳（床用）
    StoneWall, // 城壁・石垣
    WoodLog,   // 木材（柱・梁）
    ClayRoof,  // 瓦屋根
}

// ひとつのチャンクが持つボクセルデータを保持するECSコンポーネント
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

// 八百万エンジンのボクセルマップ初期化プラグイン
pub struct YamatoVoxelMapPlugin;

impl Plugin for YamatoVoxelMapPlugin {
    // コンパイルを通すために、appを可変参照（&mut App）に修正したのだ！
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_initial_yamato_chunk);
    }
}

// 初期チャンク（埼玉・川越ステージの足元）を生成するシステム
fn setup_initial_yamato_chunk(mut commands: Commands) {
    let mut initial_chunk = VoxelChunk::default();

    // 底面（Y=0）の全範囲に石垣と畳の基礎を敷き詰める処理
    for x in 0..CHUNK_WIDTH {
        for z in 0..CHUNK_DEPTH {
            initial_chunk.blocks[x][0][z] = VoxelType::StoneWall;
        }
    }

    // Bevy 0.19 のエンティティ生成構文に準拠
    commands.spawn((
        Name::new("Yamato_Initial_Chunk_Saitama"),
        initial_chunk,
        Transform::default(),
        GlobalTransform::default(),
    ));
}
