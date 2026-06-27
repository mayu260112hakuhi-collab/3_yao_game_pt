use regex::Regex;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameData { 
    pub id: u32, 
    pub name: String,
    //pub f8game_path: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameListWrapper { 
    pub game_list: Vec<GameData> 
}

// 汎用的なテンプレート置換関数
// テンプレート文字列内のプレースホルダーをゲームデータで置換する
fn render_game_template(inner_template: &str, game: &GameData) -> String {
    inner_template.replace("{game.name}", &game.name)
                  .replace("{game.id}", &game.id.to_string())
                  .replace("{game.f8game_path}", &format!("E:/3_yg_v3/3_ymtc/{}.8game", game.name)) // ここでパスを生成
                  .replace("{game.description}", &format!("Description for {}", "一例")) // 仮の説明
                  .replace("{game.image}", &format!("game{}.png", "game_icon.png")) // 仮の画像パス
                  .replace("{game.category}", &format!("Category for {}", "etc")) // 仮のカテゴリ


}

#[tauri::command]
pub fn load_8g_file(path: String) -> Result<String, String> {
    // 1. テンプレートファイル読み込み
    let template = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    
    // JSON読み込み
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let json_path = PathBuf::from(manifest_dir)
        .join("src")
        .join("game_list_json")
        .join("game_list.json");
    
    let json_content = fs::read_to_string(&json_path).map_err(|e| e.to_string())?;
    let v: GameListWrapper = serde_json::from_str(&json_content).map_err(|e| e.to_string())?;

    // 2. 独自タグブロックを HTML に置換
    // タグの中身（inner_template）を抽出し、ループして置換処理を行う
    let re = Regex::new(r"(?s)<s-8g;.*?>([\s\S]*?)<\/s-8g;>").unwrap();
    
    let final_html = re.replace_all(&template, |caps: &regex::Captures| {
        let inner_template = caps.get(1).map_or("", |m| m.as_str());
        let mut html_output = String::new();
        
        if v.game_list.is_empty() {
            // 空のとき用: クラス名を自由に変えられるようテンプレートに置くのがベストだけど、
            // 今は一旦ここを直書き。あとで <s-empty;> のようなタグを作って分離するのもアリ！
            html_output.push_str(r#"<div class="kikaku-ichiran-empty">表示できるゲームデータがありません。</div>"#);
        } else {
            for game in &v.game_list {
                html_output.push_str(&render_game_template(inner_template, game));
            }
        }
        html_output
    });

    Ok(final_html.to_string())
}

#[tauri::command]
pub fn get_game_list() -> Result<GameListWrapper, String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let json_path = PathBuf::from(manifest_dir).join("src").join("game_list_json").join("game_list.json");
    let json_content = fs::read_to_string(&json_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&json_content).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_game_project(id: String) {
    // 1. JSON を再度読み込んで、IDに一致するパスを探す
    // 2. std::process::Command を使ってエディタを開く
    println!("プロジェクト {} を開こうとしています...", id);
}