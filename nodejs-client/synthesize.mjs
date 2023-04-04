import { PollyClient, SynthesizeSpeechCommand } from "@aws-sdk/client-polly";
import fs from 'fs/promises'
import { createWriteStream } from 'fs';
import child_process from 'child_process';

const pollyClient = new PollyClient({ region: 'ap-northeast-1' })

const main = async () => {

  const lines = [];
  for await (const chunk of process.stdin) {
    lines.push(chunk)
  };
  const text = Buffer.concat(lines).toString('utf-8');

  const command = new SynthesizeSpeechCommand({
    Text: text,
    VoiceId: 'Takumi',
    OutputFormat: 'mp3',
  })
  const { AudioStream: audioStream } = await pollyClient.send(command);

  const tmpdir = await fs.mkdtemp('/tmp/chatgpi-cli-sample-node-')
  const file_path = `${tmpdir}/test.mp3`

  await new Promise(resolve => audioStream
    .pipe(createWriteStream(file_path))
    .on('close', () => resolve(true))
  )

  await new Promise(resolve =>
    child_process.exec(`afplay ${file_path}`, () => resolve(true))
  )
  await fs.unlink(file_path)
}
main()
