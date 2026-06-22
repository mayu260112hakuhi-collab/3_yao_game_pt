// main.rs
use tauri::Manager;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub path: String,
    pub description: String,
    pub version: String,
    pub initial_scene: String,
    pub window_width: u32,
    pub window_height: u32,
    pub debug_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectInfo {
    pub name: String,
    pub directory: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProjectData {
    pub name: String,
    pub version: String,
    pub description: String,
    pub window_width: u32,
    pub window_height: u32,
    pub resizable: bool,
    pub fullscreen: bool,
    pub target_fps: u32,
    pub vsync: bool,
    pub initial_scene: String,
    pub grid_base_size: f64,
    pub grid_ratio: f64,
    pub debug_enabled: bool,
}

#[tauri::command]
fn scan_projects(base_path: String) -> Result<Vec<ProjectInfo>, String> {
    let path = Path::new(&base_path);
    if !path.is_dir() {
        return Err(format!("パスがディレクトリではありません: {}", base_path));
    }

    let mut projects = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                // E:\3_yg_v2\ 直下のディレクトリのみを対象
                if entry_path.is_dir() {
                    println!("スキャン中: {:?}", entry_path);
                    // ディレクトリ内の *.8game ファイルを探す
                    if let Ok(dir_entries) = fs::read_dir(&entry_path) {
                        for dir_entry in dir_entries.flatten() {
                            let file_path = dir_entry.path();
                            if let Some(file_name) = file_path.file_name() {
                                if let Some(name_str) = file_name.to_str() {
                                    println!("ファイル検出: {}", name_str);
                                    if name_str.ends_with(".8game") {
                                        println!("読み込み中: {:?}", file_path);
                                        match fs::read_to_string(&file_path) {
                                            Ok(content) => {
                                                match serde_json::from_str::<serde_json::Value>(&content) {
                                                    Ok(json) => {
                                                        let name = json.get("name")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("Unknown Project")
                                                            .to_string();
                                                        let description = json.get("description")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("")
                                                            .to_string();
                                                        
                                                        println!("プロジェクト追加: {}", name);
                                                        projects.push(ProjectInfo {
                                                            name,
                                                            directory: entry_path.to_string_lossy().to_string(),
                                                            description,
                                                        });
                                                    }
                                                    Err(e) => println!("JSON パースエラー: {}", e),
                                                }
                                            }
                                            Err(e) => println!("ファイル読み込みエラー: {}", e),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => return Err(format!("ディレクトリ読み込みエラー: {}", e)),
    }

    println!("スキャン完了: {} プロジェクト見つかりました", projects.len());
    Ok(projects)
}

#[tauri::command]
async fn open_project_in_editor_cmd(app: AppHandle, _path: String) -> Result<(), String> {
    if let Some(editor) = app.get_webview_window("editor") {
        editor.show().map_err(|e| e.to_string())?;
        editor.set_focus().map_err(|e| e.to_string())?;
    } else {
        WebviewWindowBuilder::new(&app, "editor", WebviewUrl::App("editor.html".into()))
            .title("八百万駆動 - エディタ")
            .inner_size(1200.0, 800.0)
            .visible(true)
            .build()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn open_settings_window(app: AppHandle) -> Result<(), String> {
    if let Some(settings) = app.get_webview_window("settings") {
        settings.show().map_err(|e| e.to_string())?;
        settings.set_focus().map_err(|e| e.to_string())?;
    } else {
        WebviewWindowBuilder::new(&app, "settings", WebviewUrl::App("settings.html".into()))
            .title("八百万駆動 - システム設定")
            .inner_size(900.0, 700.0)
            .visible(true)
            .build()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn open_new_project_window(app: AppHandle) -> Result<(), String> {
    if let Some(new_project) = app.get_webview_window("new-project") {
        new_project.show().map_err(|e| e.to_string())?;
        new_project.set_focus().map_err(|e| e.to_string())?;
    } else {
        WebviewWindowBuilder::new(&app, "new-project", WebviewUrl::App("new-project.html".into()))
            .title("八百万駆動 - 新規企画作成")
            .inner_size(700.0, 900.0)
            .visible(true)
            .build()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn open_file_dialog_window(app: AppHandle) -> Result<(), String> {
    if let Some(file_dialog) = app.get_webview_window("open-project") {
        file_dialog.show().map_err(|e| e.to_string())?;
        file_dialog.set_focus().map_err(|e| e.to_string())?;
    } else {
        WebviewWindowBuilder::new(&app, "open-project", WebviewUrl::App("open-project.html".into()))
            .title("八百万駆動 - プロジェクトを開く")
            .inner_size(600.0, 500.0)
            .visible(true)
            .build()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn create_new_project(project_data: NewProjectData) -> Result<String, String> {
    // E:\3_yg_v2\ 配下に新しいディレクトリを作成
    let base_path = "E:\\3_yg_v2";
    let project_dir_name = project_data.name.replace(" ", "_").to_lowercase();
    let project_path = Path::new(base_path).join(&project_dir_name);

    // ディレクトリが既に存在するかチェック
    if project_path.exists() {
        return Err(format!("ディレクトリ '{}' は既に存在します", project_dir_name));
    }

    // ディレクトリを作成
    fs::create_dir_all(&project_path)
        .map_err(|e| format!("ディレクトリ作成エラー: {}", e))?;

    // .8game ファイルを作成
    let game_file_name = format!("{}.8game", &project_dir_name);
    let game_file_path = project_path.join(&game_file_name);

    let game_config = serde_json::json!({
        "name": project_data.name,
        "version": project_data.version,
        "description": project_data.description,
        "display": {
            "window": {
                "width": project_data.window_width,
                "height": project_data.window_height,
                "resizable": project_data.resizable,
                "fullscreen": project_data.fullscreen,
            },
            "rendering": {
                "target_fps": project_data.target_fps,
                "vsync": project_data.vsync,
            }
        },
        "initial_scene": project_data.initial_scene,
        "grid": {
            "base_size": project_data.grid_base_size,
            "ratio": project_data.grid_ratio,
        },
        "debug_enabled": project_data.debug_enabled,
    });

    let game_config_json = serde_json::to_string_pretty(&game_config)
        .map_err(|e| format!("JSON シリアライズエラー: {}", e))?;

    fs::write(&game_file_path, game_config_json)
        .map_err(|e| format!(".8game ファイル書き込みエラー: {}", e))?;

    println!("新規プロジェクト作成完了: {}", project_path.display());
    Ok(format!("プロジェクト '{}' を作成しました", project_data.name))
}

#[tauri::command]
fn load_project_config(path: String) -> Result<ProjectConfig, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: ProjectConfig = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
fn save_file_cmd(path: String, content: String) -> Result<String, String> {
    println!("\n=== ファイル保存コマンド開始 ===");
    println!("受け取ったパス: {}", path);
    println!("パスが存在するか: {}", std::path::Path::new(&path).exists());
    println!("パスの親ディレクトリ: {:?}", std::path::Path::new(&path).parent());
    println!("コンテンツサイズ: {} bytes", content.len());
    
    match fs::write(&path, &content) {
        Ok(_) => {
            println!("✓ ファイル保存成功: {}", path);
            println!("=== ファイル保存コマンド終了 ===\n");
            Ok(format!("ファイルを保存しました: {}", path))
        }
        Err(e) => {
            let error_msg = format!("ファイル書き込みエラー: {} (パス: {})", e, path);
            println!("✗ {}", error_msg);
            println!("=== ファイル保存コマンド終了 ===\n");
            Err(error_msg)
        }
    }
}

fn find_game_files_recursive(dir: &Path, projects: &mut Vec<ProjectInfo>) -> Result<(), String> {
    println!("フォルダ探索: {:?}", dir);
    
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(name_str) = file_name.to_str() {
                            println!("  ファイル: {}", name_str);
                            
                            if name_str.ends_with(".8game") {
                                println!("    ✓ .8game ファイル検出!");
                                let file_path = path.to_string_lossy().to_string();
                                println!("    -> ファイルパス: {}", file_path);
                                
                                match fs::read_to_string(&path) {
                                    Ok(content) => {
                                        match serde_json::from_str::<serde_json::Value>(&content) {
                                            Ok(json) => {
                                                let name = json.get("name")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("Unknown Project")
                                                    .to_string();
                                                let description = json.get("description")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                
                                                println!("    -> プロジェクト追加: {}", name);
                                                projects.push(ProjectInfo {
                                                    name,
                                                    directory: file_path,
                                                    description,
                                                });
                                            }
                                            Err(e) => println!("    JSON パースエラー: {}", e),
                                        }
                                    }
                                    Err(e) => println!("    ファイル読み込みエラー: {}", e),
                                }
                            }
                        }
                    }
                } else if path.is_dir() {
                    // サブディレクトリを再帰的に探索
                    println!("  → サブフォルダ検出: {:?}", path.file_name());
                    if let Err(e) = find_game_files_recursive(&path, projects) {
                        println!("    サブディレクトリ探索エラー: {}", e);
                    }
                }
            }
        }
        Err(e) => return Err(format!("ディレクトリ読み込みエラー: {:?} -> {}", dir, e)),
    }
    Ok(())
}

#[tauri::command]
fn get_game_files_from_3ymtc_if_exists() -> Result<Vec<ProjectInfo>, String> {
    let base_path = Path::new("E:\\3_yg_v2");
    
    println!("=== プロジェクト検索開始 ===");
    println!("ベースパス: {:?}", base_path);
    println!("ベースパス存在確認: {}", base_path.exists());
    
    // E:\3_yg_v2\ が存在するかチェック
    if !base_path.exists() {
        let err_msg = format!("ベースパスが見つかりません: {:?}", base_path);
        println!("エラー: {}", err_msg);
        return Err(err_msg);
    }

    // E:\3_yg_v2\ 配下のディレクトリ一覧を取得
    println!("\nE:\\3_yg_v2\\ 配下のディレクトリ一覧:");
    let mut target_dir = None;
    match fs::read_dir(base_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(dir_name) = path.file_name() {
                        if let Some(name_str) = dir_name.to_str() {
                            println!("  - ディレクトリ: {} (path: {:?})", name_str, path);
                            if name_str == "3_ymtc" {
                                target_dir = Some(path);
                                println!("  ✓ 3_ymtc を検出しました");
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            let err_msg = format!("E:\\3_yg_v2\\ の読み込みエラー: {}", e);
            println!("エラー: {}", err_msg);
            return Err(err_msg);
        }
    }

    // 3_ymtc ディレクトリが見つからない場合
    let target_dir = match target_dir {
        Some(dir) => {
            println!("\n3_ymtc パス: {:?}", dir);
            dir
        }
        None => {
            let err_msg = "3_ymtc ディレクトリが見つかりません".to_string();
            println!("エラー: {}", err_msg);
            return Err(err_msg);
        }
    };

    println!("\n3_ymtc ディレクトリを再帰的に探索中...\n");

    // 3_ymtc ディレクトリ以下の .8game ファイルを再帰的に検索
    let mut projects = Vec::new();
    if let Err(e) = find_game_files_recursive(&target_dir, &mut projects) {
        let err_msg = format!("ファイル検索エラー: {}", e);
        println!("エラー: {}", err_msg);
        return Err(err_msg);
    }

    if projects.is_empty() {
        let err_msg = "3_ymtc 内に .8game ファイルが見つかりません".to_string();
        println!("警告: {}", err_msg);
        return Ok(projects);
    }

    println!("\n=== 検索完了: {} 個の .8game ファイル見つかりました ===", projects.len());
    println!("\n=== 返却するプロジェクト情報 ===");
    for (i, project) in projects.iter().enumerate() {
        println!("[{}] 名前: {}", i, project.name);
        println!("    ディレクトリ: {}", project.directory);
        println!("    説明: {}", project.description);
    }
    println!("=== 返却完了 ===\n");
    Ok(projects)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            open_project_in_editor_cmd,
            open_settings_window,
            open_new_project_window,
            open_file_dialog_window,
            load_project_config,
            save_file_cmd,
            scan_projects,
            create_new_project,
            get_game_files_from_3ymtc_if_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}