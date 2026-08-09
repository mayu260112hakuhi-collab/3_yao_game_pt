// crates/yaoyorozu_core/src/parser_jp.rs
// 八百万スクリプト(.8g)の構文解析・パースモジュール

use crate::command::{命令, 命令種別};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum AstNode {
    単一命令(命令),
    条件分岐 {
        条件文: String,
        真のブロック: Vec<AstNode>,
        偽のブロック: Vec<AstNode>,
    },
    複合命令(Vec<命令>),
}

pub fn 命令を解析(ソースコード: &str) -> Result<命令, String> {
    let クリーンコード = ソースコード.trim();

    // 検索した位置から文字境界を考慮して取得（「（」または「"」に対応）
    if let Some(開始位置) = クリーンコード
        .find('（')
        .or_else(|| クリーンコード.find('"'))
    {
        let 区切り文字 = クリーンコード[開始位置..].chars().next().unwrap();
        let 閉じ文字 = if 区切り文字 == '（' { '）' } else { '"' };

        // findで得た位置はバイトインデックスなので、そこから先を文字単位で探す
        if let Some(相対終了位置) =
            クリーンコード[開始位置 + 区切り文字.len_utf8()..].find(閉じ文字)
        {
            let 終了バイト位置 = 開始位置 + 区切り文字.len_utf8() + 相対終了位置;
            let 引数 = クリーンコード[開始位置 + 区切り文字.len_utf8()..終了バイト位置].to_string();

            let 残り = &クリーンコード[終了バイト位置 + 閉じ文字.len_utf8()..];
            // 「を」だけでなく、助詞や空白を柔軟にスキップして動詞を抽出
            let 動詞候補 = 残り.trim().trim_start_matches('を').trim_end_matches('。');

            if let Ok(動詞) = 命令種別::from_str(動詞候補) {
                return Ok(命令 { 動詞, 引数 });
            } else {
                // 部分一致や追加の動詞表現のフォールバック
                return Ok(命令 {
                    動詞: 命令種別::その他(動詞候補.to_string()),
                    引数,
                });
            }
        }
    }

    Err(format!(
        "構文エラー: 解析できませんでした -> {}",
        ソースコード
    ))
}

pub fn 行を解析(行テキスト: &str) -> Result<AstNode, String> {
    let 行 = 行テキスト.trim();
    if 行.is_empty() || 行.starts_with("//") {
        return Err("スキップ".to_string());
    }

    // 複合命令（「+」または「＋」で結ばれた同時実行命令）の分解
    if 行.contains('＋') || 行.contains('+') {
        let サブ要素: Vec<&str> = 行.split(['＋', '+']).collect();
        let mut 命令リスト = Vec::new();
        for 要素 in サブ要素 {
            if let Ok(cmd) = 命令を解析(要素) {
                命令リスト.push(cmd);
            }
        }
        if !命令リスト.is_empty() {
            return Ok(AstNode::複合命令(命令リスト));
        }
    }

    命令を解析(行).map(AstNode::単一命令)
}

pub fn スクリプト全体を解析(ソースコード: &str) -> Result<Vec<AstNode>, String> {
    let mut ノードリスト = Vec::new();
    let 行群: Vec<&str> = ソースコード.lines().collect();
    let mut i = 0;

    while i < 行群.len() {
        let 行 = 行群[i].trim();

        if 行.is_empty() || 行.starts_with("//") {
            i += 1;
            continue;
        }

        // 条件分岐（もし 〜 ｛ 〜 ｝ それ以外 ｛ 〜 ｝）のパース処理
        if 行.starts_with("もし") {
            let 条件文 = if let (Some(s), Some(e)) = (行.find('（'), 行.rfind('）')) {
                行[s + '（'.len_utf8()..e].to_string()
            } else {
                行.to_string()
            };

            let mut 真のブロック = Vec::new();
            let mut 偽のブロック = Vec::new();
            let mut 偽ブロック読み込み中 = false;

            i += 1;
            while i < 行群.len() {
                let ブロック行 = 行群[i].trim();
                if ブロック行 == "｝" {
                    i += 1;
                    break;
                } else if ブロック行.starts_with("｝それ以外｛")
                    || ブロック行.starts_with("それ以外｛")
                    || ブロック行.starts_with("｝ それ以外 ｛")
                {
                    偽ブロック読み込み中 = true;
                    i += 1;
                    continue;
                }

                if let Ok(ast) = 行を解析(ブロック行) {
                    if 偽ブロック読み込み中 {
                        偽のブロック.push(ast);
                    } else {
                        真のブロック.push(ast);
                    }
                }
                i += 1;
            }

            ノードリスト.push(AstNode::条件分岐 {
                条件文,
                真のブロック,
                偽のブロック,
            });
            continue;
        }

        if let Ok(ast) = 行を解析(行) {
            ノードリスト.push(ast);
        }
        i += 1;
    }

    Ok(ノードリスト)
}
