
# Transcoding m4b to wav

```bash
ffmpeg -i *.m4b -ar 16000 -ac 1 -c:a pcm_s16le <output>.wav
```



# Alignment

docker run -it -v /tmp/test:/data mmcauliffe/montreal-forced-aligner:latest


ffmpeg -i bookworm.wav -c:a libopus -b:a 32k -strict -map_metadata 0 -y bookworm.webm
