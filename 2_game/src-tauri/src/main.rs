// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 共通のライブラリ（yao_game_lib）をインポート
fn main() {
    // ランチャー用の初期化処理
    println!("ランチャーを起動します");
    // ここで共通ライブラリのランチャー起動関数を呼び出す
    // yao_game_lib::run_launcher(); 
}