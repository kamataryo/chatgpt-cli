# chatgpt-cli

`chatgpt-cli` は、 ChatGPT への API アクセスをラップしたシンプルなコマンドラインインターフェースです。

## ビルド方法

### Cargo を使ったビルド

1. まず、[Rust](https://www.rust-lang.org/tools/install) をインストールしてください
2. 次に、リポジトリをクローンします

  ```shell
  $ git clone https://github.com/kamataryo/chatgpt-cli.git
  ```

3. ディレクトリに移動して、Cargo を使ってビルドします
　
  ```shell
  $ cd chatgpt-cli
  $ cargo build --release
  ```

ビルドが完了すると、実行可能なバイナリが `target/release` ディレクトリに生成されますので適切なディレクトリに移動して使用してください。

### バイナリのダウンロード

[GitHub のリリースページ](https://github.com/kamataryo/chatgpt-cli/releases/latest) から、最新バージョンのバイナリをダウンロードできます。  
利用可能なプラットフォームに応じて、適切なバイナリを選択してください。

ダウンロードしたバイナリを実行可能にし、適切なディレクトリに移動して使用してください。

```shell
$ curl -sL https://github.com/kamataryo/chatgpt-cli/releases/download/v0.1.0/chatgpt-cli_x86_64-unknown-linux-gnu > ./chatgpt-cli
$ chmod +x ./chatgpt-cli
```

## 使い方

### API キーの入力

環境変数、または `.chatgpt-cli.yaml` というファイルを使って OpenAI の認証情報を設定します。
詳細は `.envrc.sample` または `chatgpt-cli.yaml.sample` というファイルを確認してください。

### プログラムの実行

標準入力から対話を入力します。 

```shell
$ echo "こんにちは。
お元気ですか？" | ./chatgpt-cli
```

### advanced

おもしろそうな使い方のメモです。

#### ChatGPT の対話結果でさらに質問する

```shell
$ echo "ChatGPTに対する質問を1行で作ってください" | ./chatgpt-cli | ./chatgpt-cli
```

#### Amazon polly で結果を読み上げる

Amazon Polly の SynthesizeSpeech を実行できるポリシーが必要です。
また、実行環境に afplay コマンドが必要です。

```shell
$ echo 'hello, how are you?' | ./chatgpt-cli | node ./nodejs-client/synthesize.mjs
```

#### Amazon Transcribe を使って音声入力で質問する

AWS S3 バケツと Amazon Transcribe StartTranscriptJob 及び GetTranscriptJob のポリシーが必要です。
以下を参考にバケツを作成してください。

```shell
$ aws s3 mb s3://<YOUR_BUCKET_NAME>
$ aws s3api put-bucket-policy \
  --bucket <YOUR_BUCKET_NAME> \
  --policy '{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": {
                "Service": "transcribe.amazonaws.com"
            },
            "Action": "s3:GetObject",
            "Resource": "arn:aws:s3:::<YOUR_BUCKET_NAME>/*"
        }
    ]
}'
```

また、実行可能なパスに [SoX](https://sox.sourceforge.net/) が必要です。Mac の場合は以下のコマンドでインストールできます。

```shell
$ brew install sox
```

```shell
$ export AMAZON_TRANSCRIBE_MP3_BUCKET=<YOUR_BUCKET_NAME>
# 5秒録音ののちテキスト起こし -> ChatGPT に質問
$ node ./nodejs-client/transcribe.mjs 5 | ./chatgpt-cli
```
