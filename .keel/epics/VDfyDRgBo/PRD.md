# Timed Visual Sequence Summaries - Product Requirements

## Problem Statement

atxt can currently verify only static images. GIF and video inputs have no normalized timed-sequence contract, no bounded summary transform, and no operator-visible terminal proof that survives captured, degraded, or remote sessions. We need the first verification-first timed media slice before full playback or audio work.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make GIF and short video inputs inspectable end-to-end through a verification-friendly CLI summary. | A representative timed fixture can be rendered from file path to a truthful poster frame or contact sheet without manual renderer tuning. | The CLI and library surfaces cover at least one GIF or short video path end-to-end. |
| GOAL-02 | Establish one canonical timed-sequence contract and bounded decode path that future richer playback work can reuse. | Timed media lands on a reusable sequence model instead of renderer-local frame extraction. | The first timed slice routes through a shared sequence model and decode boundary. |
| GOAL-03 | Keep timed-sequence output deterministic, bounded, and truthful across direct and degraded terminals. | The same timed fixture remains reviewable through both direct and degraded terminal summaries. | Mission proofs cover direct and degraded timed-sequence paths from the primary `just mission` entrypoint. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Agent Operator | An agent or developer reviewing recorded media artifacts over ssh, tmux, or CI logs. | A stable terminal summary of short clips without depending on inline video support. |
| Terminal-Native Developer | A human debugging moving-image regressions from ordinary terminals. | A quick, truthful sense of temporal change without leaving the shell. |
| Library Integrator | A Rust consumer embedding atxt into another verification or automation pipeline. | A narrow timed-media decode and summary API that composes with the existing still-image renderer path. |

## Scope

### In Scope

- [SCOPE-01] A canonical normalized timed-sequence surface and probe metadata for GIF and short video inputs.
- [SCOPE-02] A bounded timed-sequence decode path that extracts a representative frame set for the first slice.
- [SCOPE-03] A poster-frame or contact-sheet transform that converts the sequence summary into a shared still-image frame for rendering.
- [SCOPE-04] A thin CLI path that renders a timed visual input through the shared pipeline with sensible defaults.
- [SCOPE-05] Deterministic fixtures and operator-visible proofs for direct and degraded timed-sequence output.

### Out of Scope

- [SCOPE-06] Full terminal animation or frame streaming playback.
- [SCOPE-07] Audio waveform/spectrogram work, document rendering, or richer multimodal summaries.
- [SCOPE-08] Inline image/video terminal protocols or renderer-specific terminal heuristics outside the shared planning path.
- [SCOPE-09] Unbounded transcoding or performance optimization beyond what is needed for a truthful first verification slice.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The crate must define a canonical timed-sequence surface that captures geometry, duration, sampling metadata, and representative frames without leaking decode-backend types into the public media model. | GOAL-02 | must | Keeps timed media reusable for later work and prevents renderer-local frame contracts. |
| FR-02 | The probe path must classify GIF and video inputs with enough timed metadata to drive bounded sampling and summary planning. | GOAL-01 GOAL-02 | must | Timed media needs explicit duration/rate context before decode and rendering decisions can be made honestly. |
| FR-03 | The crate must decode a bounded representative frame set for the first supported timed visual formats. | GOAL-01 GOAL-02 | must | Provides real timed-media input instead of pretending a single still frame is sufficient everywhere. |
| FR-04 | The crate must transform the representative frame set into a verification-friendly poster frame or contact sheet that routes through the shared still-image renderer path. | GOAL-01 GOAL-03 | must | Reuses the existing renderer investment and keeps terminal output stable and reviewable. |
| FR-05 | The project must expose a thin CLI path that renders a timed visual input from a filesystem path using the shared probe, detection, planning, decode, transform, and render pipeline. | GOAL-01 | must | Proves the product slice end-to-end for operators. |
| FR-06 | The repo must include representative timed-media fixtures and reviewable proofs for both direct and degraded output paths. | GOAL-01 GOAL-03 | must | Keeps the timed summary contract reviewable in CI and protects it from silent drift. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Timed-sequence summary rendering must continue to derive renderer choice from the shared `RenderPlan` and `TerminalProfile` surfaces rather than introducing sequence-local terminal heuristics. | GOAL-03 | must | Preserves the architecture and keeps fallback policy centralized. |
| NFR-02 | The first timed visual slice must remain deterministic and testable without requiring a live interactive terminal or full playback support. | GOAL-01 GOAL-03 | must | CI and agent workflows need reproducible proofs. |
| NFR-03 | Timed-media decode and summary work must stay bounded by a small sample budget and explicit fixture sizes. | GOAL-01 GOAL-03 | must | Prevents the first timed slice from becoming a general transcoder or hanging on large assets. |
| NFR-04 | Any external decode backend used for timed media must stay behind a narrow adapter boundary with explicit failure modes. | GOAL-02 | must | Keeps the public media model independent from the chosen decode mechanism. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Timed-sequence model and probe metadata | Rust unit tests | Story-level `cargo test` evidence linked to voyage SRS requirements |
| Timed decode and summary transform | Rust unit tests plus representative fixture assertions | Story-level `cargo test` evidence over GIF/video fixtures and summary outputs |
| CLI usability and operator signal | Mission proof output and targeted CLI proof logs | `just mission` output plus story-level direct/degraded proof artifacts |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A poster frame or contact sheet is sufficient for the first verification-first timed slice; full playback can wait. | Operators may not get enough temporal information from the first summary mode. | Validate the canonical fixture and mission proof with human review during execution. |
| The existing still-image renderer can be reused once timed media is summarized into a shared frame. | The mission may require a separate timed renderer earlier than planned. | Keep the summary transform honest and confirm it flows through current render contracts. |
| A narrow ffmpeg-backed or equivalent external decode adapter is acceptable for the first video slice. | Timed video support may stall on backend or tooling issues. | Validate the adapter and failure behavior during voyage implementation. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should the default summary be a poster frame for very short clips and a contact sheet for longer ones, or should the first slice pick one canonical mode only? | Epic owner | Open |
| Which fixture mix best captures useful temporal change without making the contact sheet unreadably dense? | Epic owner | Open |
| How much timed metadata can `probe_path` surface synchronously before deeper decode becomes necessary? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A GIF or short video can be rendered end-to-end from a filesystem path through atxt's CLI and library surfaces into a truthful timed summary without manual renderer selection.
- [ ] The timed-media slice routes through one shared sequence contract, one shared summary transform, and the existing still-image render path rather than a parallel pipeline.
- [ ] Reviewable proofs exist for both a direct terminal summary and a degraded fallback summary from the primary `just mission` entrypoint.
<!-- END SUCCESS_CRITERIA -->
