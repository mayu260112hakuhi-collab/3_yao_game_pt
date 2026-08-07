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

    if let Some(開始位置) = クリーンコード
        .find('（')
        .or_else(|| クリーンコード.find('"'))
    {
        let 区切り文字 = クリーンコード.chars().nth(開始位置).unwrap();
        let 閉じ文字 = if 区切り文字 == '（' { '）' } else { '"' };

        if let Some(相対終了位置) = クリーンコード[開始位置 + 1..].find(閉じ文字)
        {
            let 終了位置 = 開始位置 + 1 + 相対終了位置;
            let 引数 = クリーンコード[開始位置 + 1..終了位置].to_string();

            let 残り = &クリーンコード[終了位置 + 1..];
            if let Some(動詞開始) = 残り.find("を") {
                let 動詞テキスト = 残り[動詞開始 + "を".len()..].trim().trim_end_matches('。');
                let 動詞 = 命令種別::from_str(動詞テキスト)?;

                return Ok(命令 { 動詞, 引数 });
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

        if 行.starts_with("もし") && 行.contains('｛') {
            let 条件開始 = 行.find('（').unwrap_or(0);
            let 条件終了 = 行.rfind('）').unwrap_or(行.len());
            let 条件文 = 行[条件開始 + 1..条件終了].to_string();

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
