import AudioRecorder from 'node-audiorecorder'
import fs from 'fs/promises'
import { createWriteStream } from 'fs'
import { S3Client } from '@aws-sdk/client-s3'
import { Upload } from '@aws-sdk/lib-storage'
import { TranscribeClient, StartTranscriptionJobCommand, GetTranscriptionJobCommand } from '@aws-sdk/client-transcribe'


const sleep = (msec) => new Promise(resolve => setTimeout(() => resolve(true), msec))

const main = async () => {
  let [,,sec] = process.argv
  sec = parseFloat(sec)
  if(typeof sec !== 'number' || sec <= 0) {
    console.error('Invalid argv[2]. Argument should be a number.')
    process.exit(1)
  }

  const options = {
    program: `rec`, // Which program to use, either `arecord`, `rec`, or `sox`.
    device: null, // Recording device to use, e.g. `hw:1,0`

    bits: 16, // Sample size. (only for `rec` and `sox`)
    channels: 1, // Channel count.
    encoding: `signed-integer`, // Encoding type. (only for `rec` and `sox`)
    format: `S16_LE`, // Encoding type. (only for `arecord`)
    rate: 16000, // Sample rate.
    type: `mp3`, // Format type.

    // Following options only available when using `rec` or `sox`.
    silence: 0.5, // Duration of silence in seconds before it stops recording.
    thresholdStart: 0.1, // Silence threshold to start recording.
    thresholdStop: 0.1, // Silence threshold to stop recording.
    keepSilence: true, // Keep the silence in the recording.
  }

  const audioRecorder = new AudioRecorder(options)
  const writeStream = createWriteStream('./tmp.mp3', { encoding: 'binary' })

  audioRecorder.start().stream().pipe(writeStream)
  await Promise.all([
    sleep(sec * 1000),
    new Promise(resolve => {
      resolve(true)
    }),
    new Promise(resolve => writeStream.on('close', resolve(true))),
  ])


  const key = Date.now()
  const s3Client = new S3Client({ region: 'ap-northeast-1' })
  const upload = new Upload({
    client: s3Client,
    params: {
      Bucket: process.env.AMAZON_TRANSCRIBE_MP3_BUCKET,
      Key: `data/${key}.mp3`,
      Body: await fs.readFile('./tmp.mp3'),
      ContentType: 'audio/mpeg',
    },
  });

  await upload.done()
  await fs.unlink('./tmp.mp3')


  const transcribeClient = new TranscribeClient({ region: 'ap-northeast-1' });
  const startTranscriptionJobCommand = new StartTranscriptionJobCommand({
    TranscriptionJobName: `${key}_test`,
    MediaFormat: 'mp3',
    LanguageCode: 'ja-JP',
    Media: {
      MediaFileUri: `https://s3.amazonaws.com/${process.env.AMAZON_TRANSCRIBE_MP3_BUCKET}/data/${key}.mp3`,
    },
  })
  await transcribeClient.send(startTranscriptionJobCommand)

  const getTranscriptionJobCommand = new GetTranscriptionJobCommand({
    TranscriptionJobName: `${key}_test`,
  })

  let transcript_url
  let retry_remain = 50
  while (true) {
    await sleep(2000)
    const getTranscribeJobCommandOutput = await transcribeClient.send(getTranscriptionJobCommand);

    if (retry_remain === 0 || getTranscribeJobCommandOutput.TranscriptionJob.TranscriptionJobStatus === 'FAILED') {
      console.error('Unknown error.')
      console.error(getTranscribeJobCommandOutput)
      process.exit(2)
    } else if (getTranscribeJobCommandOutput.TranscriptionJob.TranscriptionJobStatus === 'IN_PROGRESS') {
      retry_remain--
      continue
    } else {
      transcript_url = getTranscribeJobCommandOutput.TranscriptionJob.Transcript.TranscriptFileUri
      break
    }
  }

  const transcription_result_resp = await fetch(transcript_url)
  const transcription_result = await transcription_result_resp.json()

  const text = transcription_result.results.transcripts
    .map(({ transcript }) => transcript)
    .join('\n')

  console.log(text);
}

main()
