use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "環境構築".to_string(),
        content: "Rustの開発環境セットアップは非常にシンプルです。公式ツールの `rustup` が全てを管理してくれます。

### ツールの役割関係図

```text
    [rustup] (インストーラ & バージョン管理)
       |
       +---> [rustc] (コンパイラ: コードを機械語に変換)
       |
       +---> [cargo] (パッケージマネージャ & ビルドツール)
               |
               +---> ビルド (Build)
               +---> テスト (Test)
               +---> パッケージ管理 (Dependencies)
```

1. **rustup**:
   - Pythonの `pyenv` や Node.jsの `nvm` に相当します。
   - `rustup update` で常に最新版のRustを入手できます。

2. **rustc**:
   - C言語の `gcc` や `clang` に相当します。
   - プログラムをコンパイルする本体ですが、通常は直接使いません。

3. **cargo**:
   - Node.jsの `npm` や Pythonの `pip` に相当し、さらにビルド機能も統合されています。
   - Rustプログラマーが普段最も使うツールです。

**Concept: なぜCargoが必要なのか？**
大規模なプロジェクトでは、多数の外部ライブラリ（Crate）やコンパイル設定を管理する必要があります。Cargoはこれらを一手に引き受け、どの環境でも「一貫したビルド」を保証してくれます。".to_string(),
        code_examples: vec![
            "## インストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## インストール確認
rustc --version
# output example: rustc 1.75.0 (82e1608df 2023-12-21)

cargo --version
# output example: cargo 1.75.0 (1d8b05cdd 2023-11-20)".to_string(),
            "## プロジェクト作成と実行の基本フロー
cargo new hello_world  # 新しいプロジェクト作成
cd hello_world         # ディレクトリ移動
cargo run              # ビルドして実行".to_string()
        ],
        learning_points: vec![
            "rustupはRust環境そのものを管理するツールです。".to_string(),
            "普段の開発では `cargo` コマンドをメインに使います。".to_string(),
            "rustcは裏側で動くコンパイラ本体です。".to_string(),
        ],
    }
}
