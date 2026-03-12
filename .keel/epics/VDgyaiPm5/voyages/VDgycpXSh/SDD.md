# Audio Decode And Summary Rendering - Software Design Description

> Deliver the first bounded audio summary pipeline so a filesystem audio clip can be probed, decoded, transformed into waveform and spectrogram-oriented summaries, rendered through shared planning, and reviewed from the CLI and just mission.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage delivers the first audio slice for atxt. It extends the pipeline from visual-only verification to bounded audio summaries by adding richer audio probe metadata, a shared audio-summary model, a bounded WAV decode adapter, waveform and spectrogram transforms, and an audio renderer path selected by the existing planning surfaces.

The key boundary is that atxt owns probe metadata, mono mixdown policy, sample and FFT budgets, renderer selection, degradation policy, and CLI behavior. The decode backend is only responsible for bounded PCM extraction from the first supported input format.

## Context & Boundaries

In scope are WAV inputs, canonical audio-summary normalization, bounded sample extraction, waveform/spectrogram transforms, shared planning-based renderer selection, and deterministic proofs. Out of scope are playback, streaming, compressed-codec parity, audio semantics, and terminal-local heuristics outside shared planning.

```
┌────────────────────────────────────────────────────────────────────────────┐
│                             This Voyage                                   │
│                                                                            │
│  probe_path -> terminal detect -> plan_render -> decode audio ->          │
│                                                summarize -> render audio   │
│                                                           │               │
│                                                           ├─ spectrogram  │
│                                                           ├─ waveform     │
│                                                           └─ ASCII fallback│
└────────────────────────────────────────────────────────────────────────────┘
                ↑                                                ↑
            wav input                                       terminal stdout
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `hound` or equivalent WAV decoder | Rust crate | Decode PCM WAV headers and samples for the first bounded audio slice. | Crate dependency selected during implementation |
| `rustfft` or equivalent FFT helper | Rust crate | Compute bounded spectrogram windows for the first direct audio renderer path. | Crate dependency selected during implementation |
| Existing `media`, `terminal`, and `render` modules | Internal modules | Provide probe classification, capability detection, and renderer planning. | Current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| First decode format | Require WAV support for the first decoded audio path. | WAV is deterministic, simple to fixture, and avoids overexpanding the first mission. |
| Shared model | Introduce one audio-summary contract owned by atxt. | Keeps audio reusable and prevents renderer-local PCM handling. |
| Channel policy | Deterministic mono mixdown for the first summary slice. | Reduces renderer complexity while keeping the first proof truthful enough for verification. |
| Direct renderer posture | Prefer spectrogram output when terminal density allows, with waveform fallback when it does not. | Aligns with the architecture's higher-information direct path while preserving legibility. |
| Degraded renderer posture | Fall back to low-density waveform or ASCII output through shared planning. | Keeps terminal degradation centralized and honest. |
| Budget policy | Use fixed sample, window, and bin budgets for the first slice. | Prevents the first audio mission from becoming unbounded or flaky in CI. |

## Architecture

The voyage adds one new audio branch while preserving the existing planning boundary:

1. `media` continues to classify input paths, but audio probing gains richer WAV metadata.
2. `terminal` continues to derive `TerminalProfile`.
3. `render` continues to produce `RenderPlan`, extended to choose waveform or spectrogram modes for audio.
4. A new audio decode layer extracts bounded PCM samples into a shared audio-summary object.
5. A new transform layer derives:
   - a low-density waveform summary
   - a higher-information spectrogram summary
6. A new audio renderer maps the shared summary into terminal-safe text:
   - spectrogram for direct, sufficiently capable terminals
   - waveform for moderate terminals
   - ASCII waveform fallback for degraded sessions
7. The CLI writes the rendered string to stdout.

## Components

| Component | Purpose | Interface | Behavior |
|-----------|---------|-----------|----------|
| Audio-summary model | Canonical normalized contract for representative audio summary data. | Rust struct(s) exposed from the crate root when appropriate. | Holds duration, sample metadata, waveform bins, and spectrogram bins without backend-specific types. |
| Audio probe metadata | Extend coarse asset classification for the first decoded audio format. | `ProbeResult` additions or richer `AudioMetadata`. | Surfaces sample rate, channels, duration, and decode readiness when available. |
| WAV decode adapter | Extract bounded PCM samples from WAV inputs. | Path-based decode function returning an audio summary or decode error. | Uses a narrow adapter boundary with explicit failure messages and deterministic mono mixdown. |
| Audio transforms | Convert bounded PCM into waveform and spectrogram summaries. | Pure transform helpers over normalized PCM buffers. | Apply conservative fixed budgets for samples, windows, and spectral bins. |
| Audio renderer | Turn the shared audio summary into text output chosen by `RenderPlan`. | `render_audio_summary(&AudioSummary, &RenderPlan)`. | Produces spectrogram, waveform, or ASCII fallback output without terminal-local heuristics. |
| CLI command | End-to-end user-facing audio entrypoint. | `atxt render <path>` on an audio input. | Runs probe, detect, plan, decode, summarize, and audio render without extra mandatory flags. |

## Interfaces

The core library path for this voyage should remain simple:

- `probe_path(&Path) -> ProbeResult`
- `detect_terminal_profile(&TerminalEnvironment) -> TerminalProfile`
- `plan_render(&ProbeResult, &TerminalProfile) -> RenderPlan`
- `decode_audio_summary(&Path, &ProbeResult) -> Result<AudioSummary, AudioDecodeError>`
- `render_audio_summary(&AudioSummary, &RenderPlan) -> Result<String, AudioRenderError>`

The CLI should remain a thin wrapper over these interfaces rather than creating an audio-specific implementation path outside the shared plan.

## Data Flow

1. The CLI or caller receives a filesystem path.
2. `probe_path` classifies the asset as audio and captures coarse metadata.
3. Terminal capability detection derives a `TerminalProfile`.
4. `plan_render` selects waveform, spectrogram, or ASCII fallback output for audio.
5. The audio decode adapter extracts bounded PCM samples from the supported audio input.
6. Transform helpers build a shared audio summary containing waveform and spectral views.
7. The audio renderer turns the summary into terminal text:
   - spectrogram for direct, sufficiently capable terminals
   - waveform when density or capability is lower
   - ASCII waveform fallback for degraded sessions
8. The CLI writes the rendered string to stdout.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unsupported audio format | Probe/decode cannot support the input family for this slice. | Return a typed decode/render error with path context. | Expand support later; do not silently pretend unsupported audio is renderable. |
| Corrupt or unreadable WAV file | Probe or decode cannot extract deterministic PCM samples. | Surface a clear decode error. | User supplies a valid file or upstream generation fixes the artifact. |
| Large or long input exceeds summary budget | Input duration or size would exceed bounded first-slice budgets. | Sample conservatively or refuse with an explicit budget error. | User trims the input or later missions widen the budget. |
| Low-capability session | `RenderPlan` degrades away from direct spectrogram rendering. | Route through waveform or ASCII fallback. | Output remains reviewable even when degraded. |
| FFT or transform instability | Spectrogram windowing or normalization produces untruthful bins. | Fail tests and surface a deterministic transform error rather than guessing. | Adjust the bounded transform policy during implementation. |
