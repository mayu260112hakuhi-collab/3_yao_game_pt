// 1_yao_core/src/lib.rs
pub mod parser;
pub mod math;
pub mod interpreter;
pub mod f8g;
// 外部からはこの名前で呼び出せるようにする
pub use parser::YaoyorozuEngine;