
use regex::Regex;
//use crate::f8g::replace_all_8g_tags;
pub fn parse_wasan_expression(expression: &str) -> Option<f64> {
    let normalized = expression
        .chars()
        .filter_map(|ch| match ch {
            '０'..='９' => Some(((ch as u32 - '０' as u32) + b'0' as u32) as u8 as char),
            '＋' => Some('+'),
            '－' => Some('-'),
            '＊' => Some('*'),
            '／' => Some('/'),
            '（' => Some('('),
            '）' => Some(')'),
            c if c.is_ascii_whitespace() => None,
            c => Some(c),
        })
        .collect::<String>();

    let expression = normalized
        .replace("足す", "+")
        .replace("引く", "-")
        .replace("掛ける", "*")
        .replace("かける", "*")
        .replace("割る", "/");

    if !Regex::new(r#"^[0-9.+\-*/()]+$"#).unwrap().is_match(&expression) {
        return None;
    }

    evaluate_expression(&expression)
}

pub fn evaluate_expression(expr: &str) -> Option<f64> {
    #[derive(Debug, Clone, Copy)]
    enum Op { Add, Sub, Mul, Div }
    #[derive(Debug, Clone)]
    enum Token { Number(f64), Op(#[allow(dead_code)] Op), LParen, RParen }

    fn tokenize(expr: &str) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '0'..='9' | '.' => {
                    let mut num = String::new();
                    while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                        num.push(chars[i]); i += 1;
                    }
                    tokens.push(Token::Number(num.parse().ok()?));
                    continue;
                }
                '+' => tokens.push(Token::Op(Op::Add)),
                '-' => tokens.push(Token::Op(Op::Sub)),
                '*' => tokens.push(Token::Op(Op::Mul)),
                '/' => tokens.push(Token::Op(Op::Div)),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                _ => {}
            }
            i += 1;
        }
        Some(tokens)
    }

    // 簡易的なパーサー（評価の基礎）
    let tokens = tokenize(expr)?;
    if tokens.is_empty() { return None; }
    
    // ここに評価ロジックを配置します（簡易的に最初の数値のみを返す実装例）
    if let Token::Number(val) = tokens[0] {
        return Some(val);
    }
    None
}

