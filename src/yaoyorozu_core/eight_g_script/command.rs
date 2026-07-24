// src/core/command.rs
use crate::core::parser::YaoyorozuEngine;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum 命令種別 {
    表示,
    読む,
    保存,
    作成,
    削除,
    リネーム,
    設定,
    待機,
    ドラッグ,
    ドロップ,
    移動,
    開く,
    閉じる,
    拡大,
    縮小,
    回転,
    座標,
    取得,
    ディレクトリ内の画像を表示,
    ディレクトリの中身を取得,
    初期化,
    終了,
    // アクション用追加
    ジャンプ,
    加速,
    減速,
    衝突,
    反射,
    追跡,
    攻撃,
    被弾,
    回復,
    無敵,
    発射,
    再生,
    遷移,
    // 制御・システム系
    停止,
    再開,
    検索,
    置換,
    描画,
    フェードイン,
    フェードアウト,
    押下,
    通信,
    ログ出力,
    その他(String),
}

#[derive(Debug)]
pub struct 命令 {
    pub 動詞: 命令種別,
    pub 引数: String,
}
impl FromStr for 命令種別 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "表示" => Ok(命令種別::表示),
            "読む" => Ok(命令種別::読む),
            "保存" => Ok(命令種別::保存),
            "作成" => Ok(命令種別::作成),
            "削除" => Ok(命令種別::削除),
            "リネーム" => Ok(命令種別::リネーム),
            "ディレクトリ内の画像を表示" => Ok(命令種別::ディレクトリ内の画像を表示),
            "ディレクトリの中身を取得" => Ok(命令種別::ディレクトリの中身を取得),
            // 他の命令も必要に応じて追加してください
            _ => Ok(命令種別::その他(s.to_string())),
        }
    }
}

// コード用文章はコメント用に先頭に // を付ける。
#[tauri::command]
pub fn run_script_command(動詞: String, 引数: String) -> Result<String, String> {
    let verb = 命令種別::from_str(&動詞)?;
    // 処理中に引数が必要な場合のためにクローンしておく
    let 引数_copy = 引数.clone();
    let cmd = 命令 {
        動詞: verb, 引数
    };

    match cmd.動詞 {
        命令種別::読む => {
            let path = Path::new(&cmd.引数);
            let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
            Ok(content)
        }
        命令種別::保存 => {
            // 「パス,書き込む内容」形式で引数を受け取る想定
            let parts: Vec<&str> = cmd.引数.splitn(2, ',').collect();
            if parts.len() == 2 {
                let path = Path::new(parts[0].trim());
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::write(path, parts[1]).map_err(|e| e.to_string())?;
                Ok("成功".to_string())
            } else {
                Err("保存の引数形式が不正です（パス,内容）".to_string())
            }
        }
        命令種別::ディレクトリ内の画像を表示 => {
            let path = Path::new(&引数_copy);
            let mut files = Vec::new();
            if path.is_dir() {
                for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
                    if let Ok(e) = entry {
                        if let Some(name) = e.file_name().to_str() {
                            files.push(name.to_string());
                        }
                    }
                }
            }
            Ok(files.join(","))
        }
        命令種別::ディレクトリの中身を取得 => {
            let path = Path::new(&cmd.引数);
            // 渡されたパスがファイルなら、その親ディレクトリを採用する
            let target_path = if path.is_file() {
                path.parent().unwrap_or(path)
            } else {
                path
            };
            let mut items = Vec::new();
            if target_path.is_dir() {
                for entry in fs::read_dir(target_path).map_err(|e| e.to_string())? {
                    if let Ok(e) = entry {
                        let name = e.file_name().to_string_lossy().into_owned();
                        let is_dir = e.file_type().map_err(|e| e.to_string())?.is_dir();
                        items.push(format!("{}:{}", name, if is_dir { "dir" } else { "file" }));
                    }
                }
            }
            Ok(items.join(","))
        }
        命令種別::作成 => {
            // 例: 引数にファイルパス、あるいは「パス|type(file/dir)」などを渡す想定の土台
            let path = Path::new(&cmd.引数);
            if path.extension().is_some() {
                // 拡張子がある場合はファイルとして空作成
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::write(path, "").map_err(|e| e.to_string())?;
            } else {
                // フォルダ作成
                fs::create_dir_all(path).map_err(|e| e.to_string())?;
            }
            Ok("成功".to_string())
        }
        命令種別::削除 => {
            let path = Path::new(&cmd.引数);
            if path.is_dir() {
                fs::remove_dir_all(path).map_err(|e| e.to_string())?;
            } else if path.is_file() {
                fs::remove_file(path).map_err(|e| e.to_string())?;
            }
            Ok("成功".to_string())
        }
        命令種別::リネーム => {
            // 「旧パス,新パス」形式で引数を受け取る想定
            let parts: Vec<&str> = cmd.引数.split(',').collect();
            if parts.len() == 2 {
                fs::rename(parts[0].trim(), parts[1].trim()).map_err(|e| e.to_string())?;
                Ok("成功".to_string())
            } else {
                Err("リネームの引数形式が不正です（旧パス,新パス）".to_string())
            }
        }
        _ => {
            let mut engine = YaoyorozuEngine::new();
            // ここでエラーハンドリングを統一
            engine.execute_script_logic(&format!("{}({})", 動詞, 引数_copy))
        }
    }
}
