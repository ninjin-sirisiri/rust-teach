use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "関数と式".to_string(),
        content: r#"Rustは「式（Expression）」ベースの言語です。これが他の言語と少し違う面白いところです。

### 式 (Expression) vs 文 (Statement)

```text
       [文 (Statement)]             [式 (Expression)]
    ・アクションを起こす           ・値を計算して返す
    ・値を返さない               ・必ず値を返す
    ・末尾に `;` がある            ・末尾に `;` がない
           |                            |
    let x = 5;                   x + 1
```

### 関数の定義イメージ
Rustの関数は「数学の関数」に近いイメージです。

```text
      引数 (Input)          戻り値 (Output)
       |                      ^
       |                      |
    +--v----------------------+--+
    | fn add(a: i32, b: i32) -> i32 |
    |                             |
    |      a + b (式)             |
    +-----------------------------+
```

- **文（Statement）:** 行動を行い、値を返さない（末尾に `;` がつく）。
- **式（Expression）:** 値を評価して返す。関数の最後の行で `;` を付けないと、それが戻り値になります。

**Concept: 命令型 vs 表明型**
「何をするか（文）」ではなく「何であるか（式）」で考えることで、コードの意図が明確になります。Rustの関数は、数学的な関数に近い「結果を算出する」という性質を強く持っています。"#.to_string(),
        code_examples: vec![
            r#"fn main() {
    let result = add(10, 20);
    println!("計算結果: {}", result);
}

// 引数と戻り値の型を明記する
fn add(a: i32, b: i32) -> i32 {
    // セミコロンがないので、これが戻り値になる（return a + b; と同じ）
    a + b 
}

// セミコロンをつけると「文」になり、値を返さない（エラーになる場合がある）
// fn wrong_add(a: i32, b: i32) -> i32 {
//     a + b; // エラー！戻り値が () unit型になってしまう
// }"#.to_string(),
        ],
        learning_points: vec![
            "関数名はスネークケース（snake_case）で記述します。".to_string(),
            "関数の引数には必ず型を指定する必要があります。".to_string(),
            "文末にセミコロンを付けない式は、関数の戻り値となります。".to_string(),
        ],
    }
}
