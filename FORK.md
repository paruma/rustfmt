# 空ブロックの改行を保持する rustfmt

公式の rustfmt を元に、書きかけの空ブロックの配置を保持する設定を追加した個人用フォークです。

## ビルドと実行

リポジトリ指定の nightly と開発用コンポーネントを使用します。

```sh
rustup toolchain install nightly-2026-11-02 --profile minimal --component rustc-dev,llvm-tools,rustfmt
cargo build --locked --release --bin rustfmt
./bin/rustfmt --config preserve_empty_block_layout=true path/to/main.rs
```

`bin/rustfmt` は rustc の共有ライブラリを読み込むためのラッパーです。
このフォークの設定を使うには、この実行ファイルを呼び出す必要があります。
通常の `cargo fmt` やエディターの設定は自動では変更しません。

## 設定

対象プロジェクトの `rustfmt.toml` に記載します。

```toml
preserve_empty_block_layout = true
```

既定値は `false` です。上流の `empty_item_single_line` の設定にかかわらず、対象の空ブロックではこの設定を優先します。

- 元が `{}` なら 1 行のままにします。
- 元が複数行なら、開き括弧と閉じ括弧を別の行にします。
- 内部に空行があれば、最大 1 行まで保持します。
- 空行のインデントや行末の空白は残しません。
- コメントや文を含むブロックは通常の整形に任せます。

関数、条件分岐、ループ、クロージャー、`async`、`const`、`unsafe`、単独・ラベル付きブロック、`match` とその腕、空の `impl`・`trait`・`struct`・`enum`・インラインモジュールを回帰テストで確認します。
文字列やマクロのトークン列を一律に置換する処理ではありません。

## 検証

```sh
cargo test --locked
cargo fmt --check
```

`tests/source/preserve-empty-block-layout.rs` が入力、同名の `tests/target` のファイルが期待結果です。
上流のテスト機構で期待結果との一致と再整形時の安定性も検証します。
