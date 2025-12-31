use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "構造体とメソッド".to_string(),
        content: r#"構造体 (struct) は、関連するデータを一つにまとめるカスタム型です。

### 1. 構造体のイメージ
ユーザーアカウントなど、複数の属性を持つオブジェクトを作るのに使います。

```text
struct User {
    username: String,
    email: String,
    active: bool,
}

    [User インスタンス]
   +--------------------------------+
   | username: "alice"              |
   | email:    "alice@example.com"  |
   | active:   true                 |
   +--------------------------------+
```

### 2. メソッド (impl)
データと振る舞い（メソッド）をセットにします。

```text
       [struct Rectangle]  <-- データ (設計図)
             |
             v
       [impl Rectangle]    <-- 振る舞い (メソッド)
      +------------------+
      | fn area(&self)   |  <-- 面積を計算
      | fn can_hold(...) |  <-- 他の四角形が入るか判定
      +------------------+
```

- **implブロック:** 構造体にメソッドを定義します。第1引数の `&self` はその構造体のインスタンス自身を指します。
- **関連関数:** `String::from` のように `self` を取らない関数。コンストラクタ（生成関数）としてよく使われます。

**Concept: データと振る舞いの分離**
Rustではデータ（struct）と振る舞い（impl）を明確に分けて記述します。これはオブジェクト指向言語のクラスに似ていますが、より柔軟な設計が可能です。"#.to_string(),
        code_examples: vec![
            r#"struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 1. メソッド: インスタンスの状態を使う (&self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 2. 関連関数: インスタンスを使わない (他言語のstatic method)
    // 通常、新しいインスタンスを作るために使います
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // 関連関数で正方形を作る
    let rect = Rectangle::square(10);
    
    // メソッドを使って面積を計算
    println!("面積は: {}", rect.area());
}"#.to_string(),
        ],
        learning_points: vec![
            "構造体は関連するデータをまとめて名前をつけたものです。".to_string(),
            "implブロックを使うことで、構造体に関連するメソッドや関数を定義できます。".to_string(),
            "タプル構造体はフィールド名を持たず、型だけを定義します。".to_string(),
        ],
    }
}
