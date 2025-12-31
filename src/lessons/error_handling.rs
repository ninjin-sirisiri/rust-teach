use super::Lesson;

pub fn get_lesson() -> Lesson {
    Lesson {
        title: "エラー処理の哲学".to_string(),
        content: "Rustは例外（Exception）を持ちません。代わりに型システムを使ってエラーを管理します。

- **回復可能なエラー (`Result<T, E>`):** ユーザーの入力ミスやファイルが見つからない場合など。上位にエラーを伝える `?` 演算子が非常に便利です。
- **回復不能なエラー (`panic!`):** バグの発見など、プログラムを継続できない致命的な状況。
- **unwrapとexpect:** 成功を確信している場合や、プロトタイプ作成時に手軽に値を取り出せますが、失敗するとパニックします。

**Concept: エラーを「値」として扱う**
エラーは「特別な状態」ではなく、関数の戻り値という「単なる値」です。これにより、開発者はエラーが起こりうる場所を型から確実に把握し、適切に対処することができます。".to_string(),
        code_examples: vec![
            "use std::fs::File;\nuse std::io::{self, Read};\n\nfn read_username() -> Result<String, io::Error> {\n    let mut f = File::open(\"hello.txt\")?;\n    let mut s = String::new();\n    f.read_to_string(&mut s)?;\n    Ok(s)\n}".to_string(),
        ],
        learning_points: vec![
            "Rustには例外（Exception）がなく、型システムでエラーを扱います。".to_string(),
            "回復可能なエラーには `Result<T, E>` を使用します。".to_string(),
            "致命的なエラーが発生した場合は `panic!` でプログラムを停止します。".to_string(),
        ],
    }
}
