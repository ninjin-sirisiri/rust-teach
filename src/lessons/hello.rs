use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "Hello Worldとプロジェクト構造".to_string(),
        content: r#"Rustの最もシンプルなプログラムと、Cargoが作成するプロジェクト構造を学びます。

### 1. プロジェクト構造
`cargo new hello_world` コマンドを実行すると、以下のようなディレクトリとファイルが自動生成されます。

```text
hello_world/        <-- プロジェクトルート
├── Cargo.toml      <-- プロジェクトの設定ファイル (依存ライブラリや著者情報など)
└── src/            <-- ソースコードを入れる場所
    └── main.rs     <-- プログラムの「入り口」となるファイル
```

- **src/main.rs**: ここに実際のRustコードを書きます。
- **Cargo.toml**: JavaScriptの `package.json` に相当します。

### 2. Hello Worldのコード解説
`src/main.rs` の中身を見てみましょう。

```rust
fn main() {
    println!("Hello, world!");
}
```

- **`fn main() { ... }`**: プログラムの開始位置（エントリポイント）です。全てのRustプログラムはここから始まります。
- **`println!`**: コンソールに出力する機能です。関数のようですが、末尾に `!` があるので **「マクロ」** です。
- **`;` (セミコロン)**: 文（Statement）の終わりを示します。

**Concept: マクロ vs 関数**
`println!` が関数ではなくマクロである理由は、可変長の引数を受け取ったり、コンパイル時にフォーマット文字列をチェックしたりするためです。これにより安全かつ柔軟な出力が可能になります。"#.to_string(),
        code_examples: vec![
            "// src/main.rs
fn main() {
    // マクロを使用して標準出力に表示
    // 行末のセミコロンを忘れずに！
    println!(\"Hello, world!\"); 
}".to_string(),
            "# プロジェクトの作成と実行
cargo new hello_world
cd hello_world

# cargo run は「コンパイル」と「実行」を一度に行います
cargo run
#   Compiling hello_world v0.1.0 (path/to/hello_world)
#    Finished dev [unoptimized + debuginfo] target(s) in 0.5s
#     Running `target/debug/hello_world`
# Hello, world!".to_string(),
        ],
        learning_points: vec![
            "main関数はRustプログラムのエントリーポイント（開始地点）です。".to_string(),
            "`println!` のように `!` がつくものは関数ではなく「マクロ」です。".to_string(),
            "`cargo run` コマンドで簡単にビルドと実行が行えます。".to_string(),
        ],
    }
}
