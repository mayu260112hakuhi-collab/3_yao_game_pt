use crate::command::{命令, 命令種別};
use std::str::FromStr;

/// 日本語スクリプト文字列（例: 「"ジャンプ"をジャンプ。」）をパースして `命令` 構造体を返す
pub fn 命令を解析(ソースコード: &str) -> Result<命令, String> {
    // 例: 「（引数）を動詞。」または「"引数"を動詞。」という形式を解析
    if let Some(開始位置) = ソースコード.find('（').or_else(|| ソースコード.find('"')) {
        let 区切り文字 = ソースコード.chars().nth(開始位置).unwrap();
        let 閉じ文字 = if 区切り文字 == '（' { '）' } else { '"' };

        if let Some(相対終了位置) = ソースコード[開始位置 + 1..].find(閉じ文字) {
            let 終了位置 = 開始位置 + 1 + 相対終了位置;
            let 引数 = ソースコード[開始位置 + 1..終了位置].to_string();

            let 残り = &ソースコード[終了位置 + 1..];
            if let Some(動詞開始) = 残り.find("を") {
                let 動詞テキスト = 残り[動詞開始 + 3..].trim().trim_end_matches('。');
                let 動詞 = 命令種別::from_str(動詞テキスト)?;

                return Ok(命令 { 動詞, 引数 });
            }
        }
    }

    Err(format!("構文エラー: 解析できませんでした -> {}", ソースコード))
}