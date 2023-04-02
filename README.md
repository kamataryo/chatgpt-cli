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

```shell
$ aws polly synthesize-speech \
  --text "$(echo 'Hello!' | ./chatgpt-cli)" \
  --voice-id Joanna \
  --output-format mp3 \
  ./test.mp3 && \
  afplay ./test.mp3 && \
  rm ./test.mp3
```

#### Amazon Transcribe を使って音声入力で質問する

```shell
# 準備
$ aws s3 mb s3://kamataryo-sandbox-amazon-transcribe-mp3-bucket
$ aws s3api put-bucket-policy \
  --bucket kamataryo-sandbox-amazon-transcribe-mp3-bucket \
  --policy '{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": {
                "Service": "transcribe.amazonaws.com"
            },
            "Action": "s3:GetObject",
            "Resource": "arn:aws:s3:::kamataryo-sandbox-amazon-transcribe-mp3-bucket/*"
        }
    ]
}'
# 録音 & アップロード
$ ffmpeg -f avfoundation -i ":2" -t 3 test.mp3
$ aws s3 cp ./test.mp3 s3://kamataryo-sandbox-amazon-transcribe-mp3-bucket/test.mp3
$ rm ./test.mp3
# 文字起こし
$ aws transcribe start-transcription-job \
  --transcription-job-name 20230403_kamataryo_test \
  --media-format mp3 \
  --language-code ja-JP \
  --media MediaFileUri=https://s3.amazonaws.com/kamataryo-sandbox-amazon-transcribe-mp3-bucket/test.mp3
# 結果の取得
$ TRANSCRIPT=$(
    curl -sL $(
      aws transcribe get-transcription-job \
        --transcription-job-name 20230403_kamataryo_test |
      jq -r '.TranscriptionJob.Transcript.TranscriptFileUri'
  ) | jq -r '.results.transcripts[0].transcript'
)
# chatGPT に投げる
$ echo $TRANSCRIPT | ./chatgpt-cli
```
