# Multimodal Video Verification Slice - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Support probing of video containers for both visual and audio metadata. | board:VDsSKs6ar |
| MG-02 | Extract a synchronized summary (contact sheet + waveform) from a video file. | board:VDsSLvO21 |
| MG-03 | Render the multimodal summary in the terminal via `atext render`. | board:VDsSLva2o |

## Strategy

1. Extend `ProbeResult` to hold both visual and audio metadata.
2. Use `symphonia` for container probing and audio decode.
3. Use `ffmpeg` for frame extraction.
4. Integrate into the CLI and Navigation Chart.

## Constraints

- Use FFmpeg for frame extraction to maintain a thin bootstrap implementation.
- Preserve deterministic mono mixdown for the audio component.
- The multimodal summary must fit within standard terminal width without wrapping.

## Halting Rules

- STOP if Symphonia cannot handle common video containers (MP4/MKV).
- STOP if FFmpeg is unavailable in the environment (it is confirmed available).
- YIELD to human for review of the first multimodal navigation proof.
