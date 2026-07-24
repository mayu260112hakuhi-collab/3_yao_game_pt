## 1. 役割分担の基本方針
- 画面・ゲーム描画: Bevy (Rust)
- マップ・シーン配置: Blender (.glb / .gltf)
- 2D UI・メッセージ枠: HTML / CSS (Bevyのプラグイン経由で描画)
- ゲームの挙動・制御: 八百万エンジン (.8g 日本語スクリプト)

## 2. ディレクトリ構造
- yaoyorozu-engine/
  - ├── assets/                    # ゲーム内で使う素材・データ
  - │   ├── scenes/                # Blenderから書き出した .glb / .gltf ファイル
  - │   ├── scripts/               # 日本語スクリプト（.8g）ファイル
  - │   └── ui/                    # 2D UI用の HTML / CSS / JavaScript ファイル
  - │
  - ├── crates/                    # 機能ごとのモジュール（Rust Workspace）
  - │   ├── yaoyorozu_core/        # 日本語スクリプトのパース・評価器（純粋なRust）
  - │   ├── yaoyorozu_bevy/        # Bevy連携用プラグイン・ECS（Resource/Component）
  - │   └── yaoyorozu_ui/          # HTML UIとのブリッジ・レンダラー処理
  - │
  - ├── Cargo.toml
  - └── README.md

## 3. 整理のポイント
### assets/ 内の役割
- scenes/: Blenderで配置・作成して出力した .glb ファイルを置く場所。
- ui/: HTML/CSSなどのUIパーツをまとめる場所。
- scripts/: 日本語スクリプト（.8g）を置く場所。

### スクリプト処理（Core）とBevy処理の分離
- 「日本語スクリプトを解釈する純粋な処理（yaoyorozu_core）」と「BevyのECSに組み込む処理（yaoyorozu_bevy）」を別フォルダ（クレート）に分けておく。
- これにより、テストがしやすくなり、今後の改修でも壊れにくくなる。
