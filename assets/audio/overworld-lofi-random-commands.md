# Overworld Lofi Random - Processing Commands

## Final Processing Chain

The final file was created in multiple steps:

### Step 1: Apply Lofi Effects to Original
Applied all the lofi effects to the original file at original speed:

```bash
ffmpeg -y -i overworld-original.aif \
  -ar 22050 \
  -acodec pcm_s16le \
  -filter:a "volume=0.4,lowpass=f=2500,highpass=f=200,acompressor=threshold=0.089:ratio=9:attack=200:release=1000,agate=level_in=1:mode=downward:range=0.11:threshold=0.003:ratio=2:attack=0.1:release=50" \
  -sample_fmt s16 \
  overworld-lofi.aif
```

**Effects applied:**
- Sample rate reduction: 44100 Hz → 22050 Hz
- Volume: 40% (0.4)
- Low-pass filter: 2500 Hz (dampens higher notes)
- High-pass filter: 200 Hz (strips low-end richness)
- Compressor: threshold=0.089, ratio=9, attack=200ms, release=1000ms
- Gate: reduces reverb tail (threshold=0.003, ratio=2, attack=0.1ms, release=50ms)

### Step 2: Slow Down by 10%
Adjusted tempo to align with 4/4 timing:

```bash
ffmpeg -y -i overworld-lofi.aif \
  -filter:a "atempo=0.9" \
  overworld-lofi-temp.aif && \
mv overworld-lofi-temp.aif overworld-lofi.aif
```

**Effect:**
- Tempo: 90% speed (10% slower)

### Step 3: Add Randomness
Added tremolo effect for volume variation:

```bash
ffmpeg -y -i overworld-lofi.aif \
  -filter:a "tremolo=f=2.5:d=0.3" \
  overworld-lofi-random.aif
```

**Effect:**
- Tremolo: frequency=2.5 Hz, depth=0.3 (30% volume variation)

### Step 4: Convert to MP3 for Web Use
Converted the AIFF file to MP3 format for web optimization:

```bash
ffmpeg -y -i overworld-lofi-random.aif \
  -codec:a libmp3lame \
  -b:a 128k \
  overworld-lofi-random.mp3
```

**Result:**
- Format: MP3
- Bitrate: 128 kbps
- File size: ~1.4MB (reduced from 7.9MB AIFF)

### Step 5: Create Half-Speed Version
Created an experimental half-speed version:

```bash
ffmpeg -y -i overworld-lofi-random.mp3 \
  -filter:a "atempo=0.5" \
  overworld-lofi-random-halfspeed.mp3
```

**Effect:**
- Tempo: 50% speed (half speed)
- Duration: ~3:06 (doubled from ~1:33)

### Step 6: Convert to OGG for Game Use
Converted the MP3 to OGG format for use with bevy_kira_audio (which requires OGG format):

```bash
ffmpeg -y -i overworld-lofi-random-halfspeed.mp3 \
  -codec:a libvorbis \
  -qscale:a 5 \
  overworld-lofi-random-halfspeed.ogg
```

**Result:**
- Format: OGG Vorbis
- Quality: 5 (good quality, smaller file size)
- File size: ~1MB (reduced from 1.4MB MP3)
- **This is the final file used in the game**

## Single Command (Combined)

If you want to recreate the final file in one command:

```bash
ffmpeg -y -i overworld-original.aif \
  -ar 22050 \
  -acodec pcm_s16le \
  -filter:a "volume=0.4,lowpass=f=2500,highpass=f=200,acompressor=threshold=0.089:ratio=9:attack=200:release=1000,agate=level_in=1:mode=downward:range=0.11:threshold=0.003:ratio=2:attack=0.1:release=50,atempo=0.9,tremolo=f=2.5:d=0.3" \
  -sample_fmt s16 \
  overworld-lofi-random.aif
```

