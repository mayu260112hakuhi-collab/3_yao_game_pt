use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
async fn open_project_in_editor_cmd(app: AppHandle, path: String) -> Result<(), String> {
    // 1. AppHandle を Manager として扱うために、トレイトのメソッドを呼び出します
    // すでに開いているウィンドウを取得
    if let Some(editor) = app.get_webview_window("editor") {
        // ウィンドウが見つかった場合、フォーカスする
        editor.set_focus().map_err(|e: tauri::Error| e.to_string())?;
    } else {
        // 2. なければ新規作成
        WebviewWindowBuilder::new(
            &app,
            "editor",
            WebviewUrl::App("editor.html".into())
        )
        .title("八百万駆動 - エディタ")
        .inner_size(1000.0, 700.0)
        .build()
        .map_err(|e: tauri::Error| e.to_string())?; // ここで型を tauri::Error に固定
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_project_in_editor_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}