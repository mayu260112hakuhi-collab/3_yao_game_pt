#![allow(non_snake_case)]

use std::collections::HashMap;

use bevy::prelude::*;

use crate::command::{二項演算子, 値, 式};

#[derive(Resource, Debug, Clone, Default)]
pub struct 八百万実行環境 {
    pub 変数: HashMap<String, 値>,
}

impl 八百万実行環境 {
    pub fn 変数を設定(&mut self, 名前: impl Into<String>, 値: 値) {
        self.変数.insert(名前.into(), 値);
    }

    pub fn 変数を取得(&self, 名前: &str) -> 値 {
        self.変数.get(名前).cloned().unwrap_or(値::なし)
    }

    pub fn 式を評価(&self, 式: &式) -> Result<値, String> {
        match 式 {
            式::値(v) => Ok(v.clone()),
            式::変数(名前) => Ok(self.変数を取得(名前)),
            式::二項 { 左, 演算子, 右 } => {
                let 左値 = self.式を評価(左)?;
                let 右値 = self.式を評価(右)?;
                self.二項演算を評価(&左値, *演算子, &右値)
            }
        }
    }

    fn 二項演算を評価(
        &self,
        左: &値,
        演算子: 二項演算子,
        右: &値,
    ) -> Result<値, String> {
        use 二項演算子::*;

        match 演算子 {
            加算 | 減算 | 乗算 | 除算 => {
                let (l, r) = match (左, 右) {
                    (値::数値(l), 値::数値(r)) => (*l, *r),
                    _ => return Err("算術演算は数値同士で行ってください".to_string()),
                };

                let 結果 = match 演算子 {
                    加算 => l + r,
                    減算 => l - r,
                    乗算 => l * r,
                    除算 => {
                        if r == 0.0 {
                            return Err("0で除算できません".to_string());
                        }
                        l / r
                    }
                    _ => unreachable!(),
                };
                Ok(値::数値(結果))
            }
            文字列連結 => Ok(値::文字列(format!("{}{}", 左, 右))),
            等価 | 厳密等価 => Ok(値::真偽(左 == 右)),
            不等価 => Ok(値::真偽(左 != 右)),
            小なり | 以下 | 大なり | 以上 => {
                let (l, r) = match (左, 右) {
                    (値::数値(l), 値::数値(r)) => (*l, *r),
                    _ => return Err("大小比較は数値同士で行ってください".to_string()),
                };
                let 結果 = match 演算子 {
                    小なり => l < r,
                    以下 => l <= r,
                    大なり => l > r,
                    以上 => l >= r,
                    _ => unreachable!(),
                };
                Ok(値::真偽(結果))
            }
            かつ => Ok(値::真偽(左.真か() && 右.真か())),
            または => Ok(値::真偽(左.真か() || 右.真か())),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct 八百万スクリプト設定 {
    pub 起動スクリプト: String,
}

impl Default for 八百万スクリプト設定 {
    fn default() -> Self {
        Self {
            起動スクリプト: "crates/yamato_crafttowers/jascript/mainSystem.8g".to_string(),
        }
    }
}
