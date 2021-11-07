# NJSLYRust

- Rust で書かれた忍殺語変換プログラム（の予定）
- kotlin版NJSLYRConvertの移植
- 忍殺 NinjaSlayer とはなにか
    - Twitterの公式アカウント @njslyr
    - 公式facebook https://www.facebook.com/ninjaslayer.jp
    - https://diehardtales.com

- ビルドに必要
    - `cargo build`
    - TablePlusなどのsqliteエディタ

- 実行方法
    - `./target/debug/NJSLYRust [FLAGS]`
    - `-h, --help`            Prints help information
    - `-o, --oldtext`         show old text 変換前文字列（解析後）の表示
    - `-p, --partofspeech`    show part of speech or not 品詞表示
    - `-V, --version`         Prints version information
    - 標準入力を１行ずつ読んで、標準出力に出力

- 製作
    - いまのところ形態素解析くらいしかできていません
