# Static Image Rendering With Txtplot - Product Requirements

## Problem Statement

atxt has probing and render planning, but no end-to-end still-image decoder, normalized visual frame, or txtplot-backed terminal renderer. We need a first vertical slice that turns a static image into truthful braille or ASCII terminal output with deterministic fallbacks and reviewable fixtures.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make still images inspectable end-to-end in hostile terminal environments through the atxt library and CLI. | A representative still image can be rendered from file path to reviewable terminal output without manual renderer tuning. | End-to-end CLI and library proofs cover at least one still-image path. |
| GOAL-02 | Establish one canonical normalized visual-frame contract and decoder boundary that future timed media can reuse. | The still-image slice lands on a reusable visual-frame surface instead of renderer-local raster handling. | The first still-image implementation routes through a shared visual-frame model. |
| GOAL-03 | Keep renderer selection and degradation truthful under captured, low-capability, or non-Unicode sessions. | Fallback behavior is deterministic and reviewable for low-capability terminal profiles. | Snapshot or CLI proofs cover direct braille and degraded ASCII paths. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Agent Operator | An agent or developer reviewing media artifacts over ssh, tmux, or CI logs. | A truthful textual rendering of a still image without leaving the terminal workflow. |
| Terminal-Native Developer | A human debugging image-heavy systems from ordinary terminals. | A reliable way to inspect image contents and regressions without inline image protocol support. |
| Library Integrator | A Rust consumer embedding atxt into another verification or automation pipeline. | A stable decode-to-render API that does not depend on ad hoc terminal heuristics. |

## Scope

### In Scope

- [SCOPE-01] A canonical normalized visual-frame surface and decoder boundary for static raster images.
- [SCOPE-02] Static-image decoding for the first common raster families needed by the still-image slice.
- [SCOPE-03] A txtplot-backed braille renderer plus ASCII fallback for still images using the shared render-planning contract.
- [SCOPE-04] A thin CLI entrypoint that renders a still image from a filesystem path to stdout.
- [SCOPE-05] Deterministic fixtures and verification artifacts for direct and degraded rendering paths.

### Out of Scope

- [SCOPE-06] GIF, video, timed-sequence rendering, or contact-sheet rendering work.
- [SCOPE-07] Audio waveform or spectrogram output.
- [SCOPE-08] Inline image protocols, terminal-specific graphics transports, or non-txtplot visual backends.
- [SCOPE-09] Full color-fidelity goals such as advanced dithering, palette management, or perceptual optimization beyond a truthful first slice.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The crate must define a canonical normalized visual-frame surface for still-image rendering that is independent from txtplot and other backend-specific details. | GOAL-02 | must | Keeps the decode path reusable for later timed media and prevents renderer-local raster models. |
| FR-02 | The crate must decode the first supported still-image families from a filesystem path into the shared visual-frame contract. | GOAL-01 GOAL-02 | must | Provides the first usable media-to-frame path for real assets instead of probe-only classification. |
| FR-03 | The crate must render a shared still-image frame through a txtplot-backed braille path when the terminal profile allows it. | GOAL-01 GOAL-03 | must | Establishes the first high-density truthful renderer using the chosen backend. |
| FR-04 | The crate must provide an explicit ASCII fallback for still images when Unicode reliability or other terminal constraints make braille unsuitable. | GOAL-01 GOAL-03 | must | Ensures useful output under hostile or captured sessions instead of optimistic failure. |
| FR-05 | The project must expose a thin CLI path that renders a still image from a filesystem path using the shared probe, detection, planning, decode, and render pipeline. | GOAL-01 | must | Proves the product slice end-to-end and gives operators a runnable surface. |
| FR-06 | The repo must include representative fixtures and reviewable render proofs for direct and degraded still-image output paths. | GOAL-01 GOAL-03 | must | Keeps output reviewable in CI and protects the renderer contract against silent drift. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Static-image rendering must continue to derive renderer choice from the shared terminal profile rather than ad hoc backend checks. | GOAL-03 | must | Preserves the architecture and keeps fallback policy centralized. |
| NFR-02 | The first still-image slice must remain deterministic and testable without requiring a live interactive terminal. | GOAL-01 GOAL-03 | must | CI and agent workflows need reproducible proofs without terminal-specific lab conditions. |
| NFR-03 | The txtplot dependency must remain behind a narrow adapter boundary so future renderer backends or internal changes do not leak through the public atxt model. | GOAL-02 | must | Prevents backend choice from becoming the canonical media contract. |
| NFR-04 | The default path must remain auto-configured for ordinary terminals without mandatory renderer-specific flags. | GOAL-01 GOAL-03 | must | atxt is explicitly trying to avoid fragile manual terminal configuration. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Shared visual-frame model and decode path | Rust unit tests | Story-level `cargo test` evidence linked to voyage SRS requirements |
| CLI still-image rendering | VHS terminal recordings and targeted CLI proof logs | Story-level `vhs` artifacts or manual CLI proof logs for representative fixtures |
| Output usefulness and fallback readability | LLM judge or curated snapshot review for terminal output | Story-level `llm-judge` or equivalent reviewed fixture evidence |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| txtplot's existing braille canvas is sufficient for a first still-image renderer when wrapped by a small adapter layer. | atxt may be blocked on upstream txtplot additions before rendering can land. | Validate the planned adapter against txtplot's current public API during voyage design. |
| The first still-image slice can be successful without solving advanced color-dithering or photorealistic mapping. | The output may be judged too lossy for verification workflows. | Use representative fixtures and fallback review during execution. |
| A thin CLI is enough for the first user-facing slice; richer command UX can wait. | The first vertical slice may feel incomplete for operators. | Keep the command narrow and revisit after the first usable path lands. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Does txtplot need a public helper for raster blitting, or is a small atxt-side adapter enough for the first slice? | Epic owner | Open |
| Should the first CLI expose width controls immediately, or rely on terminal size and current render planning defaults? | Epic owner | Open |
| Which fixture set best captures both recognizability and degradation pressure for still images? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A still image can be rendered end-to-end from a filesystem path through atxt's CLI and library surfaces without manual renderer selection.
- [ ] The still-image slice routes through one shared visual-frame contract and one shared terminal-planning path rather than renderer-local heuristics.
- [ ] Reviewable proofs exist for both a direct braille render and a degraded ASCII fallback.
<!-- END SUCCESS_CRITERIA -->
