# Audio Summary Rendering - Product Requirements

## Problem Statement

atxt now has verification-first slices for static images and timed visual summaries, but audio inputs still have no normalized decode, transform, or renderer path. `wav`, `mp3`, `flac`, and `ogg` files can be classified during probing, yet there is no truthful waveform or spectrogram output for CLI users or agent workflows. We need the first bounded audio verification slice before attempting richer multimodal or semantic audio work.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make a representative audio input inspectable end-to-end through a verification-friendly CLI summary. | A canonical audio fixture can be rendered from file path to a truthful waveform or spectrogram summary without manual renderer tuning. | The CLI and library surfaces cover at least one WAV path end-to-end. |
| GOAL-02 | Establish one canonical audio-summary contract and bounded decode/transform path that future richer audio work can reuse. | Audio lands on a reusable summary model instead of renderer-local PCM handling. | The first audio slice routes through a shared audio summary model and decode boundary. |
| GOAL-03 | Keep audio output deterministic, bounded, and truthful across direct and degraded terminals. | The same audio fixture remains reviewable through both direct and degraded terminal summaries. | Mission proofs cover direct and degraded audio paths from the primary `just mission` entrypoint. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Agent Operator | An agent or developer reviewing generated audio artifacts over ssh, tmux, or CI logs. | A stable terminal summary of audio content without leaving the shell or relying on playback. |
| Terminal-Native Developer | A human debugging audio regressions from ordinary terminals. | A quick, truthful sense of amplitude and spectral change without opening a media player. |
| Library Integrator | A Rust consumer embedding atxt into another verification or automation pipeline. | A narrow audio decode and summary API that composes with the shared probe and planning surfaces. |

## Scope

### In Scope

- [SCOPE-01] A canonical normalized audio-summary surface and probe metadata for the first supported decoded audio inputs.
- [SCOPE-02] A bounded WAV decode path with deterministic mono mixdown and summary-friendly sample extraction.
- [SCOPE-03] Waveform and spectrogram summary transforms plus shared renderer selection for audio summaries.
- [SCOPE-04] A thin CLI path that renders an audio input through the shared pipeline with sensible defaults.
- [SCOPE-05] Deterministic fixtures and operator-visible proofs for direct and degraded audio output.

### Out of Scope

- [SCOPE-06] Playback, streaming, or real-time terminal audio output.
- [SCOPE-07] Semantic audio analysis such as transcription, tagging, or speaker inference.
- [SCOPE-08] Decode support for compressed audio formats beyond what is needed to keep the first slice honest and bounded.
- [SCOPE-09] Renderer-local terminal heuristics, ANSI heatmaps, or advanced visualization polish that is not required for the first verification slice.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The crate must define a canonical audio-summary surface that captures duration, sample metadata, waveform-oriented bins, and spectrogram-oriented bins without leaking decode-backend types into the public media model. | GOAL-02 | must | Keeps audio reusable for later work and prevents renderer-local PCM contracts. |
| FR-02 | The probe path must classify the first decoded audio format with enough metadata to drive bounded decode and audio renderer planning. | GOAL-01 GOAL-02 | must | Audio work needs explicit duration and sample-rate context before decode and rendering decisions can be made honestly. |
| FR-03 | The crate must decode a bounded representative audio sample set for the first supported audio format and transform it into waveform and spectrogram summaries. | GOAL-01 GOAL-02 | must | Provides real audio input instead of treating audio as metadata-only. |
| FR-04 | The crate must render audio summaries through shared planning as waveform or spectrogram output with a truthful degraded fallback. | GOAL-01 GOAL-03 | must | Makes audio inspectable in the terminal while preserving centralized fallback policy. |
| FR-05 | The project must expose a thin CLI path that renders an audio input from a filesystem path using the shared probe, detection, planning, decode, transform, and render pipeline. | GOAL-01 | must | Proves the product slice end-to-end for operators. |
| FR-06 | The repo must include representative audio fixtures and reviewable proofs for both direct and degraded output paths. | GOAL-01 GOAL-03 | must | Keeps the audio summary contract reviewable in CI and protects it from silent drift. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Audio renderer choice must continue to derive from the shared `RenderPlan` and `TerminalProfile` surfaces rather than audio-local terminal heuristics. | GOAL-03 | must | Preserves the architecture and keeps fallback policy centralized. |
| NFR-02 | The first audio slice must remain deterministic and testable without requiring a live interactive terminal or playback support. | GOAL-01 GOAL-03 | must | CI and agent workflows need reproducible proofs. |
| NFR-03 | Audio decode and summary work must stay bounded by explicit sample, window, and bin budgets. | GOAL-01 GOAL-03 | must | Prevents the first audio slice from becoming an unbounded analyzer or hanging on long inputs. |
| NFR-04 | Any decode backend used for audio must stay behind a narrow adapter boundary with explicit failure modes. | GOAL-02 | must | Keeps the public audio model independent from the chosen decode mechanism. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Audio-summary model and probe metadata | Rust unit tests | Story-level `cargo test` evidence linked to voyage SRS requirements |
| Audio decode, transform, and render behavior | Rust unit tests plus representative fixture assertions | Story-level `cargo test` evidence over waveform/spectrogram summaries and renderer outputs |
| CLI usability and operator signal | Mission proof output, direct/degraded captures, and optional `vhs` recordings | `just mission` output plus story-level direct/degraded proof artifacts |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A first bounded audio slice can target WAV as the required decode format while leaving compressed formats for later work. | The mission may stall if operators need compressed audio immediately. | Validate fixture usefulness and scope boundaries during implementation. |
| Deterministic mono mixdown is sufficient for the first verification-oriented audio summary. | Stereo or spatial information may be more important than expected for some inputs. | Keep mixdown policy explicit and confirm the canonical fixture still tells the truth operators need. |
| A bounded spectrogram is useful enough to justify a small FFT dependency or equivalent transform path. | The mission may need to fall back to waveform-only output. | Validate direct proof output during execution before widening scope. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should the default direct audio renderer be spectrogram-only, or should waveform remain the canonical first output when terminal density is limited? | Epic owner | Open |
| What summary density keeps the spectrogram legible in ordinary terminals without overfitting to wide local terminals? | Epic owner | Open |
| Does the first mission need compressed audio decode fallback, or is honest WAV-only decode sufficient for the initial verification slice? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A WAV audio input can be rendered end-to-end from a filesystem path through atxt's CLI and library surfaces into a truthful waveform or spectrogram summary without manual renderer selection.
- [ ] The audio slice routes through one shared audio-summary contract, one bounded decode path, and shared render planning rather than a parallel audio-specific terminal pipeline.
- [ ] Reviewable proofs exist for both a direct terminal audio summary and a degraded fallback summary from the primary `just mission` entrypoint.
<!-- END SUCCESS_CRITERIA -->
