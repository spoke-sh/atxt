# Video Probing and Combined Summary Rendering - Software Design Description

## Architecture

We will extend the `symphonia` probing logic to identify multiple tracks.

### Probing
The `probe_path` logic will be updated to:
1. Use `symphonia` to probe the container.
2. Identify the first visual track and the first audio track.
3. Populate `ProbeResult` with both sets of metadata.

### Decoding
For the first slice, we will leverage **FFmpeg** for frame extraction and **Symphonia** for audio decoding.
- **Visual:** Use `ffmpeg` to extract specific timestamps as PNG frames.
- **Audio:** Use the existing `decode_audio_summary` logic which already uses Symphonia.

### Rendering
A new `render_multimodal_summary` function will:
1. Call `render_still_image` for the contact sheet.
2. Call `render_audio_summary` for the waveform.
3. Concatenate the strings with a separator.

## Technical Hazards

| Hazard | Risk | Mitigation |
|--------|------|------------|
| Codec Compatibility | FFmpeg might fail on some proprietary formats. | Target common formats (H.264, VP9) for the first slice. |
| Synchronization | Extracted audio might not perfectly align with frames. | Use consistent timestamp logic for both extractions. |
