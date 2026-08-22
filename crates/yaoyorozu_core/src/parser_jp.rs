#![allow(non_snake_case)]

use std::str::FromStr;

use crate::command::{二項演算子, 値, 式, 文, 命令, 命令引数, 命令種別};

pub fn スクリプト全体を解析(ソースコード: &str) -> Result<Vec<文>, String> {
    let 正規化 = コメントを除去して正規化(ソースコード);
    let 文字列: Vec<char> = 正規化.chars().collect();
    let mut 位置 = 0;
    ブロックを解析(&文字列, &mut 位置, false)
}

fn ブロックを解析(
    文字列: &[char],
    位置: &mut usize,
    閉じ括弧で終了: bool,
) -> Result<Vec<文>, String> {
    let mut 文一覧 = Vec::new();

    loop {
        空白を飛ばす(文字列, 位置);
        if *位置 >= 文字列.len() {
            break;
        }

        if 閉じ括弧で終了 && 文字列[*位置] == '}' {
            *位置 += 1;
            break;
        }

        if 先頭一致(文字列, *位置, "もし") {
            文一覧.push(条件分岐を解析(文字列, 位置)?);
            continue;
        }

        let 文文字列 = 文末まで取得(文字列, 位置)?;
        let 文文字列 = 文文字列.trim();
        if 文文字列.is_empty() {
            continue;
        }

        let 文 = 単文を解析(文文字列)?;
        文一覧.push(文);
    }

    Ok(文一覧)
}

fn 条件分岐を解析(文字列: &[char], 位置: &mut usize) -> Result<文, String> {
    *位置 += "もし".chars().count();
    空白を飛ばす(文字列, 位置);

    let 条件文字列 = 括弧内を取得(文字列, 位置)?;
    let 条件 = 式を解析(&条件文字列)?;

    空白を飛ばす(文字列, 位置);
    if *位置 >= 文字列.len() || 文字列[*位置] != '{' {
        return Err("もし の後に { が必要です".to_string());
    }
    *位置 += 1;

    let 真のブロック = ブロックを解析(文字列, 位置, true)?;
    空白を飛ばす(文字列, 位置);

    let mut 偽のブロック = Vec::new();
    if 先頭一致(文字列, *位置, "それ以外") {
        *位置 += "それ以外".chars().count();
        空白を飛ばす(文字列, 位置);

        if *位置 < 文字列.len() && 文字列[*位置] == '{' {
            *位置 += 1;
            偽のブロック = ブロックを解析(文字列, 位置, true)?;
        } else if *位置 < 文字列.len() && 文字列[*位置] == '（' {
            let 条件文字列 = 括弧内を取得(文字列, 位置)?;
            let 条件 = 式を解析(&条件文字列)?;
            空白を飛ばす(文字列, 位置);
            if *位置 >= 文字列.len() || 文字列[*位置] != '{' {
                return Err("それ以外（条件）の後に { が必要です".to_string());
            }
            *位置 += 1;
            let 真 = ブロックを解析(文字列, 位置, true)?;
            偽のブロック.push(文::条件分岐 {
                条件,
                真のブロック: 真,
                偽のブロック: Vec::new(),
            });
        } else {
            return Err("それ以外 の後に { または （条件） が必要です".to_string());
        }
    }

    Ok(文::条件分岐 {
        条件,
        真のブロック,
        偽のブロック,
    })
}

fn 単文を解析(入力: &str) -> Result<文, String> {
    let 入力 = 文末記号を除去(入力.trim());

    let 複合 = 最上位で分割(入力, &["+", "＋"]);
    if 複合.len() > 1 {
        let mut 文一覧 = Vec::new();
        for 部分 in 複合 {
            文一覧.push(単文を解析(部分.trim())?);
        }
        return Ok(文::複合(文一覧));
    }

    if let Some((変数名, 右辺)) = 代入を分解(入力)? {
        return Ok(文::代入 {
            変数名,
            値: 式を解析(右辺.trim())?,
        });
    }

    Ok(文::命令(命令を解析(入力)?))
}

pub fn 式を解析(入力: &str) -> Result<式, String> {
    let 入力 = 入力.trim();
    if 入力.is_empty() {
        return Err("空の式です".to_string());
    }

    let 優先順位: &[(&[&str], fn(&str) -> 二項演算子)] = &[
        (&["または", "||"], |_| 二項演算子::または),
        (&["かつ", "&&"], |_| 二項演算子::かつ),
        (&["===", "==", "!=", "<=", ">=", "<", ">"], |op| match op {
            "===" => 二項演算子::厳密等価,
            "==" => 二項演算子::等価,
            "!=" => 二項演算子::不等価,
            "<=" => 二項演算子::以下,
            ">=" => 二項演算子::以上,
            "<" => 二項演算子::小なり,
            ">" => 二項演算子::大なり,
            _ => unreachable!(),
        }),
        (&["+", "-", "."], |op| match op {
            "+" => 二項演算子::加算,
            "-" => 二項演算子::減算,
            "." => 二項演算子::文字列連結,
            _ => unreachable!(),
        }),
        (&["*", "/"], |op| match op {
            "*" => 二項演算子::乗算,
            "/" => 二項演算子::除算,
            _ => unreachable!(),
        }),
    ];

    for (演算子候補, 変換) in 優先順位 {
        if let Some((位置, 演算子)) = 最上位演算子を探す(入力, 演算子候補) {
            let 左 = 入力[..位置].trim();
            let 右 = 入力[位置 + 演算子.len()..].trim();
            if 左.is_empty() || 右.is_empty() {
                continue;
            }
            return Ok(式::二項 {
                左: Box::new(式を解析(左)?),
                演算子: 変換(演算子),
                右: Box::new(式を解析(右)?),
            });
        }
    }

    原子式を解析(入力)
}

fn 原子式を解析(入力: &str) -> Result<式, String> {
    let 入力 = 入力.trim();

    if 入力.starts_with('「') && 入力.ends_with('」') {
        let 中身 = &入力['「'.len_utf8()..入力.len() - '」'.len_utf8()];
        return Ok(式::値(値::文字列(中身.to_string())));
    }

    if 入力.starts_with('（') && 入力.ends_with('）') && 全体が一組の括弧(入力) {
        let 中身 = &入力['（'.len_utf8()..入力.len() - '）'.len_utf8()];
        let 中身 = 中身.trim();

        if 変数名として妥当(中身) {
            return Ok(式::変数(中身.to_string()));
        }

        return 式を解析(中身);
    }

    if 入力.starts_with('(') && 入力.ends_with(')') && 全体が一組のASCII括弧(入力) {
        return 式を解析(&入力[1..入力.len() - 1]);
    }

    if let Ok(数値) = 入力.parse::<f64>() {
        return Ok(式::値(値::数値(数値)));
    }

    match 入力 {
        "真" => Ok(式::値(値::真偽(true))),
        "偽" => Ok(式::値(値::真偽(false))),
        "なし" => Ok(式::値(値::なし)),
        _ => Ok(式::値(値::記号(入力.to_string()))),
    }
}

fn 命令を解析(入力: &str) -> Result<命令, String> {
    let 入力 = 文末記号を除去(入力.trim());
    let (動詞文字列, 前半) = 末尾動詞を分離(入力)?;
    let 動詞 = 命令種別::from_str(動詞文字列)?;
    let 引数 = 命令引数を解析(前半)?;

    Ok(命令 { 動詞, 引数 })
}

fn 命令引数を解析(入力: &str) -> Result<Vec<命令引数>, String> {
    let mut 結果 = Vec::new();
    let mut 残り = 入力.trim();

    while !残り.is_empty() {
        let (式文字列, 後ろ) = 先頭式を切り出す(残り)?;
        let 値 = 式を解析(式文字列)?;
        let (助詞, 次) = 助詞を切り出す(後ろ);
        結果.push(命令引数 { 助詞, 値 });
        残り = 次.trim();
    }

    Ok(結果)
}

fn 末尾動詞を分離(入力: &str) -> Result<(&str, &str), String> {
    let mut 境界 = None;
    let mut 文字列中 = false;
    let mut 括弧深度 = 0i32;

    for (idx, ch) in 入力.char_indices() {
        match ch {
            '「' => 文字列中 = true,
            '」' => 文字列中 = false,
            '（' | '(' if !文字列中 => 括弧深度 += 1,
            '）' | ')' if !文字列中 => 括弧深度 -= 1,
            c if c.is_whitespace() && !文字列中 && 括弧深度 == 0 => 境界 = Some(idx),
            _ => {}
        }
    }

    if let Some(idx) = 境界 {
        let 前半 = 入力[..idx].trim();
        let 動詞 = 入力[idx..].trim();
        if !動詞.is_empty() {
            return Ok((動詞, 前半));
        }
    }

    for 接尾 in ["する", "表示", "終了", "開始", "初期化", "選択", "実行", "再生", "読み込み", "ロード", "スポーン", "移動", "ログ出力"] {
        if let Some(前半) = 入力.strip_suffix(接尾) {
            return Ok((接尾, 前半.trim()));
        }
    }

    Err(format!("命令の動詞を判定できません: {入力}"))
}

fn 先頭式を切り出す(入力: &str) -> Result<(&str, &str), String> {
    let 入力 = 入力.trim_start();
    if 入力.starts_with('「') {
        let 開始長 = '「'.len_utf8();
        let 終了 = 入力[開始長..]
            .find('」')
            .map(|v| v + 開始長)
            .ok_or_else(|| "文字列の閉じ括弧 」 がありません".to_string())?;
        let 終了バイト = 終了 + '」'.len_utf8();
        return Ok((&入力[..終了バイト], &入力[終了バイト..]));
    }

    if 入力.starts_with('（') {
        let 終了 = 対応括弧位置(入力, '（', '）')?;
        let 終了バイト = 終了 + '）'.len_utf8();
        return Ok((&入力[..終了バイト], &入力[終了バイト..]));
    }

    let 終了 = 入力
        .char_indices()
        .find(|(_, c)| c.is_whitespace() || matches!(c, 'を' | 'に' | 'へ' | 'の' | 'と'))
        .map(|(i, _)| i)
        .unwrap_or(入力.len());

    Ok((&入力[..終了], &入力[終了..]))
}

fn 助詞を切り出す(入力: &str) -> (Option<String>, &str) {
    let 入力 = 入力.trim_start();
    for 助詞 in ["から", "まで", "より", "へ", "に", "を", "の", "と"] {
        if let Some(残り) = 入力.strip_prefix(助詞) {
            return (Some(助詞.to_string()), 残り);
        }
    }
    (None, 入力)
}

fn 代入を分解(入力: &str) -> Result<Option<(String, &str)>, String> {
    if !入力.starts_with('（') {
        return Ok(None);
    }

    let 終了 = 対応括弧位置(入力, '（', '）')?;
    let 左 = &入力['（'.len_utf8()..終了];
    if !変数名として妥当(左.trim()) {
        return Ok(None);
    }

    let 残り = 入力[終了 + '）'.len_utf8()..].trim_start();
    if let Some(右辺) = 残り.strip_prefix('=') {
        if 右辺.starts_with('=') {
            return Ok(None);
        }
        return Ok(Some((左.trim().to_string(), 右辺)));
    }

    Ok(None)
}

fn 最上位演算子を探す<'a>(入力: &'a str, 候補: &[&'a str]) -> Option<(usize, &'a str)> {
    let mut 文字列中 = false;
    let mut 全角括弧深度 = 0i32;
    let mut ASCII括弧深度 = 0i32;
    let mut 発見 = None;

    for (idx, ch) in 入力.char_indices() {
        match ch {
            '「' => {
                文字列中 = true;
                continue;
            }
            '」' => {
                文字列中 = false;
                continue;
            }
            '（' if !文字列中 => {
                全角括弧深度 += 1;
                continue;
            }
            '）' if !文字列中 => {
                全角括弧深度 -= 1;
                continue;
            }
            '(' if !文字列中 => {
                ASCII括弧深度 += 1;
                continue;
            }
            ')' if !文字列中 => {
                ASCII括弧深度 -= 1;
                continue;
            }
            _ => {}
        }

        if 文字列中 || 全角括弧深度 > 0 || ASCII括弧深度 > 0 {
            continue;
        }

        let 残り = &入力[idx..];
        for op in 候補 {
            if 残り.starts_with(op) {
                発見 = Some((idx, *op));
                break;
            }
        }
    }

    発見
}

fn 最上位で分割<'a>(入力: &'a str, 区切り: &[&str]) -> Vec<&'a str> {
    let mut 結果 = Vec::new();
    let mut 開始 = 0usize;
    let mut 文字列中 = false;
    let mut 括弧深度 = 0i32;

    for (idx, ch) in 入力.char_indices() {
        match ch {
            '「' => 文字列中 = true,
            '」' => 文字列中 = false,
            '（' | '(' if !文字列中 => 括弧深度 += 1,
            '）' | ')' if !文字列中 => 括弧深度 -= 1,
            _ => {}
        }

        if !文字列中 && 括弧深度 == 0 {
            let 残り = &入力[idx..];
            if let Some(一致) = 区切り.iter().find(|s| 残り.starts_with(**s)) {
                結果.push(&入力[開始..idx]);
                開始 = idx + 一致.len();
            }
        }
    }

    結果.push(&入力[開始..]);
    結果
}

fn 文末まで取得(文字列: &[char], 位置: &mut usize) -> Result<String, String> {
    let mut 結果 = String::new();
    let mut 文字列中 = false;
    let mut 括弧深度 = 0i32;

    while *位置 < 文字列.len() {
        let ch = 文字列[*位置];
        match ch {
            '「' => 文字列中 = true,
            '」' => 文字列中 = false,
            '（' | '(' if !文字列中 => 括弧深度 += 1,
            '）' | ')' if !文字列中 => 括弧深度 -= 1,
            ';' | '；' | '。' if !文字列中 && 括弧深度 == 0 => {
                *位置 += 1;
                break;
            }
            '}' if !文字列中 && 括弧深度 == 0 => {
                if 結果.trim().is_empty() {
                    return Err("予期しない } です".to_string());
                }
                break;
            }
            _ => {}
        }
        結果.push(ch);
        *位置 += 1;
    }

    Ok(結果)
}

fn 括弧内を取得(文字列: &[char], 位置: &mut usize) -> Result<String, String> {
    if *位置 >= 文字列.len() {
        return Err("条件式がありません".to_string());
    }

    let (開始, 終了) = match 文字列[*位置] {
        '（' => ('（', '）'),
        '(' => ('(', ')'),
        _ => return Err("条件式は （） または () で囲んでください".to_string()),
    };

    *位置 += 1;
    let mut 深度 = 1i32;
    let mut 文字列中 = false;
    let mut 結果 = String::new();

    while *位置 < 文字列.len() {
        let ch = 文字列[*位置];
        if ch == '「' {
            文字列中 = true;
        } else if ch == '」' {
            文字列中 = false;
        } else if !文字列中 && ch == 開始 {
            深度 += 1;
        } else if !文字列中 && ch == 終了 {
            深度 -= 1;
            if 深度 == 0 {
                *位置 += 1;
                return Ok(結果);
            }
        }

        結果.push(ch);
        *位置 += 1;
    }

    Err("括弧が閉じていません".to_string())
}

fn コメントを除去して正規化(入力: &str) -> String {
    let mut 結果 = String::new();

    for 行 in 入力.lines() {
        let mut 文字列中 = false;
        let mut 切断位置 = 行.len();
        let bytes = 行.as_bytes();
        let mut i = 0usize;

        while i < bytes.len() {
            let 残り = &行[i..];
            if 残り.starts_with('「') {
                文字列中 = true;
                i += '「'.len_utf8();
                continue;
            }
            if 残り.starts_with('」') {
                文字列中 = false;
                i += '」'.len_utf8();
                continue;
            }
            if !文字列中 && 残り.starts_with("//") {
                切断位置 = i;
                break;
            }
            i += 行[i..].chars().next().unwrap().len_utf8();
        }

        結果.push_str(&行[..切断位置]);
        結果.push('\n');
    }

    結果
}

fn 空白を飛ばす(文字列: &[char], 位置: &mut usize) {
    while *位置 < 文字列.len() && 文字列[*位置].is_whitespace() {
        *位置 += 1;
    }
}

fn 先頭一致(文字列: &[char], 位置: usize, 候補: &str) -> bool {
    let 候補: Vec<char> = 候補.chars().collect();
    文字列.get(位置..位置 + 候補.len()) == Some(候補.as_slice())
}

fn 対応括弧位置(入力: &str, 開始: char, 終了: char) -> Result<usize, String> {
    let mut 深度 = 0i32;
    let mut 文字列中 = false;

    for (idx, ch) in 入力.char_indices() {
        if ch == '「' {
            文字列中 = true;
            continue;
        }
        if ch == '」' {
            文字列中 = false;
            continue;
        }
        if 文字列中 {
            continue;
        }
        if ch == 開始 {
            深度 += 1;
        } else if ch == 終了 {
            深度 -= 1;
            if 深度 == 0 {
                return Ok(idx);
            }
        }
    }

    Err(format!("{} に対応する {} がありません", 開始, 終了))
}

fn 全体が一組の括弧(入力: &str) -> bool {
    対応括弧位置(入力, '（', '）')
        .map(|i| i + '）'.len_utf8() == 入力.len())
        .unwrap_or(false)
}

fn 全体が一組のASCII括弧(入力: &str) -> bool {
    対応括弧位置(入力, '(', ')')
        .map(|i| i + 1 == 入力.len())
        .unwrap_or(false)
}

fn 変数名として妥当(入力: &str) -> bool {
    !入力.is_empty()
        && !入力.chars().any(|c| {
            c.is_whitespace()
                || matches!(c, '=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' | '.' | '「' | '」')
        })
}

fn 文末記号を除去(入力: &str) -> &str {
    入力
        .trim_end_matches(|c| matches!(c, ';' | '；' | '。' | '．'))
        .trim_end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 文字列と変数を区別できる() {
        assert_eq!(
            式を解析("「キャラクター（K）」").unwrap(),
            式::値(値::文字列("キャラクター（K）".to_string()))
        );
        assert_eq!(
            式を解析("（キャラクター）").unwrap(),
            式::変数("キャラクター".to_string())
        );
    }

    #[test]
    fn 代入と条件分岐を解析できる() {
        let source = r#"
            （体力） = 100;
            もし（（体力） <= 0）{
                「ゲームオーバー」を表示;
            }
        "#;
        let ast = スクリプト全体を解析(source).unwrap();
        assert_eq!(ast.len(), 2);
    }
}
