
use std::env;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

mod f8g;
use f8g::{load_8g_file, get_game_list};
//mod editor;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WarehouseItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub rarity: String,
    pub icon_name: String,
    pub params: String,
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_8g_file, get_game_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}