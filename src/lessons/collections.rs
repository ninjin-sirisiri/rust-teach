use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "基本コレクション".to_string(),
        content: "標準ライブラリが提供する、ヒープに保存されるデータ構造です。

- **Vec<T>:** 連続したメモリに値を並べる「ベクタ」。可変長配列です。
- **String:** UTF-8エンコードされた可変長の文字列。
- **HashMap<K, V>:** キーと値のペアを保存するマップ。

**Concept: UTF-8の安全性**
Rustの `String` は常に有効なUTF-8であることを保証します。不正なバイト列によるバグを防ぎ、世界中の文字を安全に扱うことができます。".to_string(),
        code_examples: vec![
            "// ベクタ\nlet mut v = vec![1, 2, 3];\nv.push(4);\n\n// 文字列\nlet mut s = String::from(\"foo\");\ns.push_str(\"bar\");\n\n// ハッシュマップ\nuse std::collections::HashMap;\nlet mut scores = HashMap::new();\nscores.insert(String::from(\"Blue\"), 10);".to_string(),
        ],
        learning_points: vec![
            "Vec<T> はサイズを変更可能な配列のようなコレクションです。".to_string(),
            "String はUTF-8でエンコードされたテキストを所有します。".to_string(),
            "HashMap<K, V> はキーと値のペアを効率的に検索できます。".to_string(),
        ],
    }
}
