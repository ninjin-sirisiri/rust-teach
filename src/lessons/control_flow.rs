use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "制御フロー".to_string(),
        content: r#"Rustの制御構文は多くが「式」として扱えるのが特徴です。つまり、制御構文自体が値を返すことができます。

### 1. if 式 (分岐して値を返す)
他の言語では `if` はただの「命令」ですが、Rustでは結果を変数に入れることができます。

```text
       [条件: condition]
          /      \
      (true)    (false)
        |          |
      [ 5 ]      [ 6 ]
        \        /
         v      v
      [変数 number に代入]
       let number = ...
```

### 2. ループの比較

- **loop**: 無限ループ。サーバーの待受など「意図的に止めない」処理に使います。
- **while**: 条件付きループ。
- **for**: 安全な繰り返し。イテレータを使うので、インデックス間違いが起きません。

```text
[For Loop のイメージ]
コレクション: [10, 20, 30]
              ^   ^   ^
              |   |   |
   (イテレータが一つずつ安全に取り出す)
              |
      element in ... 
```

**Concept: forループの安全性**
Rustの `for` ループは、配列のインデックスを直接操作するのではなく、イテレータを使用します。これにより、「範囲外アクセス」というバグがコンパイル時に不可能な構造になっています。"#.to_string(),
        code_examples: vec![
            r#"fn main() {
    // 1. if は「式」なので値を返せる
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);

    // 2. loop から値を返す
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // ループを抜けつつ値を返す
        }
    };
    println!("ループの結果: {}", result);

    // 3. 安全な for ループ
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("値は: {}", element);
    }
}"#.to_string(),
        ],
        learning_points: vec![
            "if式は値を返すことができるため、変数への代入に使用できます。".to_string(),
            "loopは無限ループを作成し、breakで値を返すことも可能です。".to_string(),
            "forループはイテレータを使ってコレクションを安全に走査します。".to_string(),
        ],
    }
}
