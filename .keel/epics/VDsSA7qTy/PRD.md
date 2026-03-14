# Multimodal Video Verification - Product Requirements

## Problem Statement

Currently, `atext` can render visual sequences (GIFs) and audio clips (WAV/MP3) independently. However, video files (MP4, MKV, etc.) contain interleaved visual and audio streams. There is no unified way to verify a video file's combined health, synchronization, or content without rendering them separately.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Support probing of video containers to identify visual and audio streams simultaneously. | `probe_path` correctly reports dimensions and audio metadata for MP4/MKV. | Unified probing covering major containers. |
| GOAL-02 | Extract a synchronized summary of a video: a contact sheet of visual frames and a matching audio waveform. | A multimodal model exists that combines frames and audio bins. | One canonical multimodal summary contract. |
| GOAL-03 | Render a unified multimodal text representation in the terminal. | `atext render` on a video file displays both visual and audio summaries. | Truthful terminal-native video inspection. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Media Operator | A human verifying video ingestion over a CLI. | Quick verification of visual and audio presence/sync. |
| Verification Agent | An AI reviewing video artifacts in a CI pipeline. | Deterministic text representation of video content. |

## Scope

### In Scope

- [SCOPE-01] Probing support for MP4, MKV, AVI, MOV containers.
- [SCOPE-02] Extraction of 4 keyframes via FFmpeg.
- [SCOPE-03] Extraction of corresponding mono audio waveform via Symphonia.
- [SCOPE-04] Combined multimodal renderer stacking visual above audio.

### Out of Scope

- [SCOPE-05] Real-time video playback.
- [SCOPE-06] In-engine video decoding (preferring FFmpeg for now).
- [SCOPE-07] Multi-track audio selection (target first track).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `probe_path` must return both visual and audio metadata for video files. | GOAL-01 | must | Enables multimodal planning. |
| FR-02 | The decoder must extract synchronized visual frames and audio samples. | GOAL-02 | must | Ensures the summary is representative. |
| FR-03 | The renderer must display both visual and audio components in a single output. | GOAL-03 | must | Provides the unified verification view. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Video summaries must respect terminal-aware render planning. | GOAL-03 | must | Preserves architectural consistency. |
| NFR-02 | Extraction must remain bounded by sample and frame budgets. | GOAL-02 | must | Prevents resource exhaustion. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Multimodal Probing | Rust unit tests | `cargo test` with MP4/MKV fixtures. |
| Sync Extraction | Rust unit tests | `cargo test` verifying sample/frame alignment. |
| Unified Rendering | CLI Verification | `just screen` showing the Navigation Chart. |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| FFmpeg is reliable for frame extraction in this environment. | Mission stalled. | Confirmed `ffmpeg -version` success. |
| Synchronizing by timestamp is sufficient for a coarse summary. | Drift in visualization. | Review proofs manually. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should we support multiple audio tracks in the first slice? | Epic Owner | Decided: No. |
| Will some containers hide audio stream metadata from Symphonia? | Epic Owner | Open. |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `atext` identifies `.mp4`, `.mkv`, and other common video containers.
- [ ] Probing reports both visual dimensions/timing and audio stream properties.
- [ ] A single `atext render` command on a video file produces both a contact sheet and a waveform.
- [ ] The rendering remains truthful and terminal-native (Braille/ASCII).
<!-- END SUCCESS_CRITERIA -->
