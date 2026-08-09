use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveGameData {
    pub player_name: String,
    pub current_layer: String,
}

pub fn save_game_to_disk(data: &SaveGameData) -> Result<(), Box<dyn std::error::Error>> {
    let encoded: Vec<u8> = bincode::serialize(data)?;
    let mut file = std::fs::File::create("yamato_save.dat")?;
    file.write_all(&encoded)?;
    println!("八百万データのセーブが完了したのだ！");
    Ok(())
}
