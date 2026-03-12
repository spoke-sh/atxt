# Timed Sequence Decode And Contact Sheet Rendering - Software Design Description

> Define the canonical timed-sequence model, add the first bounded GIF/video decode path, transform timed media into a verification-friendly summary frame, and expose that path through the CLI with deterministic proofs.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage delivers the first timed visual slice for atxt. It extends the pipeline from still-image rendering to bounded timed-sequence summaries by adding timed probe metadata, a shared sequence model, a bounded decode adapter, and a summary transform that turns timed media back into a still `VisualFrame` for the existing Braille/ASCII renderer path.

The key boundary is that atxt owns probe metadata, sample-budget policy, summary layout, degradation policy, and CLI behavior. A decode backend such as `ffmpeg` is only responsible for bounded representative frame extraction.

## Context & Boundaries

In scope are GIF and short video inputs, canonical timed-sequence normalization, bounded representative sampling, poster-frame/contact-sheet summarization, and deterministic proofs. Out of scope are full terminal animation, audio, inline graphics transports, and generalized transcoding.

```
┌────────────────────────────────────────────────────────────────────────────┐
│                             This Voyage                                   │
│                                                                            │
│  probe_path -> terminal detect -> plan_render -> decode sequence ->       │
│                                                summarize -> render frame   │
│                                                          │                │
│                                                          ├─ braille       │
│                                                          │  via txtplot   │
│                                                          └─ ASCII fallback│
└────────────────────────────────────────────────────────────────────────────┘
                ↑                                                ↑
         gif / short video inputs                          terminal stdout
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `ffmpeg` CLI or equivalent backend | External tool | Decode a bounded representative frame set for GIF/video fixtures and supported user inputs. | System CLI available in the repo shell |
| Existing `media`, `terminal`, and `render` modules | Internal modules | Provide probe classification, capability detection, and renderer planning. | Current crate API |
| Existing `frame` and `still_image` modules | Internal modules | Provide the shared `VisualFrame` surface and the current text renderer path. | Current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Timed rendering posture | Summary-first, not playback-first. | Verification needs a truthful bounded summary before it needs full animation. |
| Sequence model | Introduce one shared timed-sequence contract owned by atxt. | Keeps timed media reusable and prevents decode backend details from leaking outward. |
| First decode backend | Use a narrow external decode adapter for representative frame extraction. | This is the fastest way to support both GIF and video without overcommitting to a heavy internal codec stack. |
| Summary output | Produce a poster frame or contact sheet that converts back into a shared `VisualFrame`. | Reuses the still-image renderer investment and keeps terminal output deterministic. |
| Fallback policy | Continue to rely on the shared `RenderPlan` and still-image fallback renderers. | Timed media should not create a new terminal capability policy. |
| Sample budget | Keep a conservative fixed sample count for the first slice. | Prevents the first timed-media path from becoming unbounded or flaky in CI. |

## Architecture

The voyage adds one new timed branch that rejoins the existing still-image path before final rendering:

1. `media` continues to classify input paths, but now timed media gains duration/rate metadata.
2. `terminal` continues to derive `TerminalProfile`.
3. `render` continues to produce `RenderPlan`.
4. A new timed-sequence decode layer extracts a bounded representative frame set into a shared sequence object.
5. A new summary transform produces one shared `VisualFrame`:
   - poster frame for the simplest sequence mode
   - contact sheet for the default verification summary mode
6. The existing still-image renderer path turns that `VisualFrame` into Braille or ASCII text.
7. The CLI writes the rendered string to stdout.

## Components

| Component | Purpose | Interface | Behavior |
|-----------|---------|-----------|----------|
| Timed-sequence model | Canonical normalized contract for representative timed visual data. | Rust struct(s) exposed from the crate root when appropriate. | Holds duration, dimensions, sample timestamps, and sampled frames without backend-specific types. |
| Timed probe metadata | Extend coarse asset classification for GIF/video inputs. | `ProbeResult` additions or timed metadata sub-structs. | Surfaces duration, nominal frame rate, frame count when available, and whether summary decode is supported. |
| Timed decode adapter | Extract a bounded representative frame set from GIF/video inputs. | Path-based decode function returning a sequence or decode error. | Uses an external backend behind a narrow boundary and explicit failure messages. |
| Summary transform | Convert a bounded timed sequence into one still `VisualFrame`. | Pure transform helper. | Lays out sampled frames as a poster frame or contact sheet sized to the render plan. |
| CLI command | End-to-end user-facing timed entrypoint. | `atxt render <path>` on a timed visual input. | Runs probe, detect, plan, decode, summarize, and still-image render without extra mandatory flags. |

## Interfaces

The core library path for this voyage should remain simple:

- `probe_path(&Path) -> ProbeResult`
- `detect_terminal_profile(&TerminalEnvironment) -> TerminalProfile`
- `plan_render(&ProbeResult, &TerminalProfile) -> RenderPlan`
- `decode_timed_sequence(&Path, &ProbeResult) -> Result<TimedSequence, DecodeError>`
- `summarize_timed_sequence(&TimedSequence, &RenderPlan) -> Result<VisualFrame, SummaryError>`
- `render_still_image(&VisualFrame, &RenderPlan) -> Result<String, RenderError>`

The CLI should remain a thin wrapper over these interfaces rather than creating a separate timed-media implementation path.

## Data Flow

1. The CLI or caller receives a filesystem path.
2. `probe_path` classifies the asset as a timed visual input and captures coarse metadata.
3. Terminal capability detection derives a `TerminalProfile`.
4. `plan_render` selects a still-image render mode appropriate for the eventual summary frame.
5. The timed decode adapter extracts a bounded representative frame set.
6. The summary transform converts the sampled sequence into one `VisualFrame`.
7. The existing still-image renderer turns the summary frame into terminal text:
   - braille path for direct Unicode-capable sessions
   - ASCII fallback for degraded sessions
8. The CLI writes the rendered string to stdout.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unsupported timed format | Probe/decode cannot support the input family for this slice. | Return a typed decode/render error with path context. | Expand support in later work; do not silently pretend the asset is a still image. |
| Missing decode backend | External timed decode adapter is not available in the environment. | Fail clearly with backend guidance rather than panicking or hanging. | Install the backend or use a supported environment such as the repo dev shell. |
| Corrupt or unreadable media file | Probe or decode cannot extract representative frames. | Surface a clear decode error. | User supplies a valid file or upstream generation fixes the artifact. |
| Large or long input exceeds summary budget | Input duration or size would exceed the bounded first-slice budget. | Sample conservatively or refuse with an explicit budget error. | User trims the input or later missions widen the budget. |
| Low-capability session | `RenderPlan` degrades away from Braille output. | Route the summary frame through the existing ASCII fallback. | Output remains reviewable even when degraded. |
