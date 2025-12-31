use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "所有権（Ownership）".to_string(),
        content: r#"所有権はRustで最も重要かつユニークな概念です。

### 1. スタック vs ヒープ (メモリのイメージ)
値がどこに保存されるかを理解することが重要です。

```text
  [Stack] (固定サイズ・高速)      [Heap] (可変サイズ・低速)
   +------+                      +-----------+
   | x: 5 |                      | "hello"   |
   +------+                      |           |
   | y: 10|                      +-----------+
   +------+                            ^
      | ポインタ (ptr)                 |
   +--|--------------------------------+
   |  s1  |
   +------+
```

### 2. 所有権の移動 (Move)
`let s2 = s1;` を実行すると、スタック上のデータだけがコピーされ、所有権が移動します。

```text
  [Stack]
   s1 (無効!)      s2 (有効)        [Heap]
   +------+       +------+        +-----------+
   | ptr  |--X--> | ptr  |------> | "hello"   |
   | len  |       | len  |        +-----------+
   | cap  |       | cap  |
   +------+       +------+
```
s1 はもうデータを持っていないとみなされ、使えなくなります(二重解放エラーを防ぐため)。

### 3. 所有権の3つの規則
1. 各値は「所有者」と呼ばれる変数を持つ。
2. 所有者は一度に1人だけ。
3. 所有者がスコープから外れると、値は自動的に破棄（drop）される。

**Concept: スタック vs ヒープ**
Rustはデータがメモリのどこにあるかを正確に管理します。整数などのサイズ固定データは「スタック」へ、Stringなどの可変長データは「ヒープ」へ保存されます。所有権システムは、このヒープメモリの解放漏れを防ぐために存在します."#.to_string(),
        code_examples: vec![
            r#"fn main() {
    // 1. Move (所有権の移動)
    let s1 = String::from("hello");
    let s2 = s1; // 所有権が s2 に Move した！
    
    // println!("{}, world!", s1); // エラー！ s1 はもう無効

    println!("{}, world!", s2); // OK

    // 2. Clone (ディープコピー)
    let s3 = String::from("world");
    let s4 = s3.clone(); // ヒープ上のデータもコピーする
    println!("s3 = {}, s4 = {}", s3, s4); // 両方使える

    // 3. Borrow (借用)
    let s5 = String::from("Rust");
    let len = calculate_length(&s5); // '&' で参照だけ渡す
    println!("'{}' の長さは {}", s5, len); // s5 はまだ使える
}

fn calculate_length(s: &String) -> usize {
    s.len()
}"#.to_string(),
        ],
        learning_points: vec![
            "各値は「所有者」と呼ばれる変数を持ち、所有者は常に一つです。".to_string(),
            "所有者がスコープを抜けると、値は破棄（ドロップ）されます。".to_string(),
            "データ競合を防ぐため、可変の借用は同時に1つしか存在できません。".to_string(),
        ],
    }
}
