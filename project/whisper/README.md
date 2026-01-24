
# Transcoding m4b to wav

```bash
ffmpeg -i *.m4b -ar 16000 -ac 1 -c:a pcm_s16le <output>.wav
```
