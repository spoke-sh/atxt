# Audio Decode And Summary Rendering - SRS

## Summary

Epic: VDgyaiPm5
Goal: Deliver the first bounded audio summary pipeline so a filesystem audio clip can be probed, decoded, transformed into waveform and spectrogram-oriented summaries, rendered through shared planning, and reviewed from the CLI and just mission.

## Scope

### In Scope

- [SCOPE-01] Define a shared audio-summary model and the probe metadata required for bounded audio planning.
- [SCOPE-02] Decode a bounded representative WAV sample set with deterministic mono mixdown.
- [SCOPE-03] Transform the bounded audio samples into waveform and spectrogram summaries and render them through shared planning.
- [SCOPE-04] Expose a thin CLI render path for audio inputs using the shared probe, planning, decode, transform, and render pipeline.
- [SCOPE-05] Add deterministic fixtures and operator-visible proofs for direct and degraded audio summaries.

### Out of Scope

- [SCOPE-06] Playback, streaming, or any real-time audio transport.
- [SCOPE-07] Speech-to-text, semantic audio inference, or non-summary analysis.
- [SCOPE-08] Compressed audio decode support beyond what is necessary to keep the first slice honest and bounded.
- [SCOPE-09] Audio-local terminal capability probing or visualization polish outside shared planning.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The crate must define a canonical audio-summary surface that captures duration, audio metadata, waveform bins, and spectrogram bins without exposing decode-backend types in the public media model. | SCOPE-01 | FR-01 | cargo test |
| SRS-02 | `probe_path` must classify WAV inputs with enough audio metadata to drive bounded decode and audio render planning. | SCOPE-01 | FR-02 | cargo test |
| SRS-03 | The crate must decode a bounded WAV input and transform it into waveform and spectrogram-oriented summary data suitable for terminal rendering. | SCOPE-02 SCOPE-03 | FR-03 | cargo test |
| SRS-04 | Audio summaries must render through shared planning as waveform or spectrogram output with truthful degraded fallback behavior. | SCOPE-03 SCOPE-04 | FR-04 | cargo test |
| SRS-05 | The CLI must render an audio input from a filesystem path through the shared probe, terminal detection, planning, decode, transform, and render pipeline without mandatory audio-specific flags. | SCOPE-04 | FR-05 | cargo test / manual |
| SRS-06 | The repo must include a representative WAV fixture and reviewable proofs for both direct and degraded audio summary output paths. | SCOPE-05 | FR-06 | cargo test / manual / llm-judge |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Audio renderer choice must continue to derive from the shared `RenderPlan` and `TerminalProfile` surfaces instead of audio-local terminal heuristics. | SCOPE-03 SCOPE-04 | NFR-01 | cargo test |
| SRS-NFR-02 | The audio slice must remain deterministic and testable without requiring a live interactive terminal session. | SCOPE-01 SCOPE-02 SCOPE-03 SCOPE-04 SCOPE-05 | NFR-02 | cargo test / manual |
| SRS-NFR-03 | The first audio slice must stay bounded by explicit sample, window, and bin budgets and must not require playback support. | SCOPE-02 SCOPE-03 SCOPE-04 SCOPE-05 | NFR-03 | cargo test |
| SRS-NFR-04 | Any decode backend for audio must remain behind a narrow adapter boundary with explicit failure surfaces. | SCOPE-02 | NFR-04 | cargo test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
