use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "列挙型とパターンマッチング".to_string(),
        content: r#"Rustの列挙型（Enum）は非常に強力で、データを持たせることができます。

### 1. Enum (変身できる型)
Enumは「どれか一つ」の状態をとります。

```text
enum Message {
    Quit,              // [Quit]
    Move { x, y },     // [Move | x:10, y:20]
    Write(String),     // [Write | "hello"]
}
```

### 2. Option型 (Nullの代わり)
Rustには `null` がありません。代わりに「箱に入っているか、空っぽか」を表す `Option` を使います。

```text
       Option<i32>
      /           \
  Some(5)         None
 [ 5 ]           [   ] (空っぽ)
```
中身を取り出すには、必ず「箱を開けて確認する」処理 (`match`など) が必要です。

### 3. match式 (最強のスイッチ)
全ての可能性を網羅（排他的にチェック）しないとコンパイルエラーになります。

```text
      [input]
         |
    +----v----+
    |  match  |
    +---------+
     /   |   \
 case A case B case C ... (漏れがあってはいけない)
```

**Concept: 代数的データ型**
RustのEnumは「和の種類」を表現する代数的データ型です。特に `Option` の存在により、「値がない」状態を型システムが強制的に処理させることで、多くの言語で発生する NullPointerException を撲滅しています。"#.to_string(),
        code_examples: vec![
            r#"enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn route(msg: Message) {
    match msg {
        Message::Quit => println!("終了"),
        Message::Write(s) => println!("メッセージ: {}", s),
        Message::Move { x, y } => println!("移動: x={}, y={}", x, y),
    }
}

fn main() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    if let Some(i) = some_number {
        println!("数字が入っています: {}", i);
    } else {
        println!("空っぽです");
    }
}"#.to_string(),
        ],
        learning_points: vec![
            "Enumは複数のバリアントの一つになりうる型を定義します。".to_string(),
            "Option<T> はRustにおける「値の不在（null）」を安全に扱う仕組みです。".to_string(),
            "match式を使うと、列挙型の全てのパターンを網羅的に処理できます。".to_string(),
        ],
    }
}
