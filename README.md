# chatgpt-cli

`chatgpt-cli` は、 ChatGPT をラップしたシンプルなコマンドラインインターフェースです。

## ビルド方法

### Cargo を使ったビルド

1. まず、[Rust](https://www.rust-lang.org/tools/install) をインストールしてください。
2. 次に、リポジトリをクローンします。

  ```shell
  $ git clone https://github.com/your_username/chatgpt-cli.git
  ```

3. ディレクトリに移動して、Cargo を使ってビルドします。
　
  ```shell
  $ cd chatgpt-cli
  $ cargo build --release
  ```

ビルドが完了すると、実行可能なバイナリが `target/release` ディレクトリに生成されますので適切なディレクトリに移動して使用してください。

### バイナリのダウンロード

[GitHub のリリースページ](https://github.com/kamataryo/chatgpt-cli/releases) から、最新バージョンのバイナリをダウンロードできます。  
利用可能なプラットフォームに応じて、適切なバイナリを選択してください。

ダウンロードしたバイナリを実行可能にし、適切なディレクトリに移動して使用してください。

## 使い方

### API キーの入力

環境変数、または `.chatgpt-cli.yaml` というファイルを使って OpenAI の認証情報を設定します。
詳細は `.envrc.sample` または `chatgpt-cli.yaml.sample` というファイルを確認してください。

### プログラムの実行

標準入力から対話を入力します。 

```shell
$ echo "こんにちは。
お元気ですか？" | chatgpt-cli
```
