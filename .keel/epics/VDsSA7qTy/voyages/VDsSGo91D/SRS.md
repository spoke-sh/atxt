# Video Probing and Combined Summary Rendering - SRS

## Summary

Epic: VDsSA7qTy
Goal: Support video container probing and multimodal rendering (contact sheet + waveform).

## Scope

### In Scope

- [SCOPE-01] Detect video containers (MP4, MKV, AVI, MOV) during probing.
- [SCOPE-02] Extract metadata for both visual and audio streams in a single pass.
- [SCOPE-03] Extract a synchronized multimodal summary (frames and audio samples).
- [SCOPE-04] Render the combined summary in the terminal.

### Out of Scope

- [SCOPE-05] Real-time video playback or streaming.
- [SCOPE-06] Semantic video analysis (object detection, etc.).
- [SCOPE-07] In-engine native video codec support (delegating to FFmpeg for frames).

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `probe_path` must support `.mp4`, `.mkv`, `.avi`, `.mov` extensions and identify them as Video. | SCOPE-01 | FR-01 | cargo test |
| SRS-02 | `ProbeResult` must contain both `PixelDimensions` and `AudioMetadata` when multiple streams are present. | SCOPE-02 | FR-01 | cargo test |
| SRS-03 | The system must extract synchronized visual frames and audio waveform data for a representative video slice. | SCOPE-03 | FR-02 | cargo test |
| SRS-04 | The CLI `render` command must stack the visual contact sheet above the audio waveform for video inputs. | SCOPE-04 | FR-03 | just screen |
| SRS-05 | The repo must include a representative video fixture and mission proofs. | SCOPE-04 | FR-03 | just screen |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Video summaries must respect the established terminal-aware render planning. | SCOPE-04 | NFR-01 | just screen |
| SRS-NFR-02 | Multimodal extraction must remain bounded by sample and frame budgets. | SCOPE-03 | NFR-02 | cargo test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
