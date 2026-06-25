//use crate::f8g::{GameListWrapper, replace_all_8g_tags};

use regex::Regex;

use std::collections::HashMap;

pub struct YaoyorozuEngine {
    pub registry: HashMap<String, fn(&str) -> String>,
    block_regex: Regex,
    func_regex: Regex,
}

impl YaoyorozuEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            registry: HashMap::new(),
            // 8g形式のブロックを抽出
            block_regex: Regex::new(r"(?s)<s_8g;(.+?)e_8g;>").unwrap(),
            // 関数呼び出し <8g(関数名){引数};> を抽出
            func_regex: Regex::new(r"<8g\((.+?)\)\{(.+?)\};>").unwrap(),
        };
        engine.register_base_functions();
        engine
    }

    // ここで日本語関数も含めて一括登録するよ！
    fn register_base_functions(&mut self) {
        self.registry.insert("title".to_string(), |_| "八百万エディタ".to_string());
        
        // 日本語関数の登録
        self.registry.insert("区画".to_string(), |_| {
            format!("<div class='name'>content</div>")
        });
        
        self.registry.insert("get_title".to_string(), |_| "八百万エディタ".to_string());
        self.registry.insert("get_time".to_string(), |_| "20:30".to_string());
    }

    pub fn parse_full_template(&self, content: &str) -> String {
        let mut result = content.to_string();

        if let Some(caps) = self.block_regex.captures(content) {
            let block_content = caps[1].to_string();

            let processed_block = self.func_regex.replace_all(&block_content, |c: &regex::Captures| {
                let func_name = &c[1];
                let args = &c[2]; // 引数も取得可能にしたよ
                
                match self.registry.get(func_name) {
                    Some(func) => func(args),
                    None => format!("<8g({0}){{{1}}};>", func_name, args), // 見つからない場合はそのまま戻す
                }
            });
            result = processed_block.to_string();
        }
        result
    }
    
}