// src/core/mod.rs
// command.rs をモジュールとして公開
pub mod command;
pub mod executor;
pub mod parser_jp;
// main.rsから eight_g_script::run_script_command で呼べるようにする
