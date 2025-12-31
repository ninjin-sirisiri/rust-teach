use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "変数と不変性".to_string(),
        content: r#"Rustの変数はデフォルトで**不変（Immutable）**です。これは「一度決めたら変えない」という設計思想で、バグを減らし並列処理を安全にします。

### 1. 不変と可変のイメージ

```text
[let x = 5;] (不変)
   +-------+
x: |   5   |  <-- ロックされている (書き換え不可)
   +-------+

[let mut y = 10;] (可変)
   +-------+
y: |  10   |  <-- 書き換え可能
   +-------+
      |
   (y = 15)
      v
   +-------+
y: |  15   |
   +-------+
```

### 2. シャドーイング (Shadowing)
同じ名前の変数を再宣言して、古い変数を「覆い隠す」機能です。

```text
let x = 5;      --> [x: 5 (i32)]
      
let x = "Rust"; --> [x: "Rust" (&str)]  <-- 前のxは見えなくなる
      
println!("{}", x); // "Rust" と表示される
```

- **不変変数:** `let x = 5;`（後から変更不可）
- **可変変数:** `let mut x = 5;`（変更可能）
- **シャドーイング:** 型を変える変身のようなことができます。

**Concept: なぜデフォルトで「不変」なのか？**
状態の管理がプログラミングの複雑さの大きな原因だからです。「不変」をデフォルトにすることで、プログラムの動作が予測しやすくなり、マルチスレッド環境でのデータ競合を根本から防ぐことができます。"#.to_string(),
        code_examples: vec![
            r#"fn main() {
    // 1. 不変変数 (Immutable by default)
    let x = 5;
    println!("xの値: {}", x);
    // x = 6; // コンパイルエラー！

    // 2. 可変変数 (Mutable)
    let mut y = 5;
    println!("yの値: {}", y);
    y = 6; // OK
    println!("yの値(変更後): {}", y);

    // 3. シャドーイング (Shadowing)
    let spaces = "   "; // 文字列型
    // 同じ名前で再定義（型も変えられる）
    let spaces = spaces.len(); // 数値型
    println!("スペースの数: {}", spaces);
}"#.to_string(),
        ],
        learning_points: vec![
            "Rustの変数はデフォルトで不変（immutable）です。".to_string(),
            "再代入が必要な場合は `mut` キーワードを使います。".to_string(),
            "シャドーイングを使うと、同じ名前の変数を再定義して型を変更できます。".to_string(),
        ],
    }
}
