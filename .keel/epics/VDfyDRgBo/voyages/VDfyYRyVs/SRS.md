# Timed Sequence Decode And Contact Sheet Rendering - SRS

## Summary

Epic: VDfyDRgBo
Goal: Define the canonical timed-sequence model, add the first bounded GIF/video decode path, transform timed media into a verification-friendly summary frame, and expose that path through the CLI with direct and degraded mission proofs.

## Scope

### In Scope

- [SCOPE-01] Define a shared timed-sequence model and the probe metadata required for bounded sampling.
- [SCOPE-02] Decode a bounded representative frame set for the first supported GIF and short video inputs.
- [SCOPE-03] Transform the bounded frame set into a poster frame or contact sheet summary that can reuse the still-image renderer path.
- [SCOPE-04] Expose a thin CLI render path for timed visual inputs using the shared probe, planning, decode, transform, and render pipeline.
- [SCOPE-05] Add deterministic fixtures and operator-visible proofs for direct and degraded timed-sequence summaries.

### Out of Scope

- [SCOPE-06] Full animation playback, frame streaming, or interactive transport control.
- [SCOPE-07] Audio, document rendering, or non-visual modalities.
- [SCOPE-08] Inline image/video terminal protocols and any renderer-specific terminal capability probing outside shared planning.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The crate must define a canonical timed-sequence surface that captures dimensions, duration, and representative sampled frames without exposing decode-backend types in the public media model. | SCOPE-01 | FR-01 | cargo test |
| SRS-02 | `probe_path` must classify GIF and video inputs with enough timed metadata to drive bounded sequence sampling and summary planning. | SCOPE-01 | FR-02 | cargo test |
| SRS-03 | The crate must decode a bounded representative frame set for the first supported timed visual formats and transform it into a verification-friendly poster frame or contact sheet summary. | SCOPE-02 SCOPE-03 | FR-03 | cargo test |
| SRS-04 | The timed summary must route through the shared still-image renderer path selected by `RenderPlan` rather than introducing a parallel timed-media terminal renderer. | SCOPE-03 SCOPE-04 | FR-04 | cargo test |
| SRS-05 | The CLI must render a timed visual input from a filesystem path through the shared probe, terminal detection, planning, decode, transform, and render pipeline without mandatory sequence-specific flags. | SCOPE-04 | FR-05 | cargo test / manual |
| SRS-06 | The repo must include a representative timed-media fixture and reviewable proofs for both direct and degraded timed summary output paths. | SCOPE-05 | FR-06 | cargo test / manual / llm-judge |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Timed summary renderer choice must continue to derive from the shared `RenderPlan` and `TerminalProfile` surfaces instead of sequence-local terminal heuristics. | SCOPE-03 SCOPE-04 | NFR-01 | cargo test |
| SRS-NFR-02 | The timed visual slice must remain deterministic and testable without requiring a live interactive terminal session. | SCOPE-01 SCOPE-02 SCOPE-03 SCOPE-04 SCOPE-05 | NFR-02 | cargo test / manual |
| SRS-NFR-03 | The first timed-media slice must stay bounded by a small sample budget and must not require general animation playback. | SCOPE-02 SCOPE-03 SCOPE-04 SCOPE-05 | NFR-03 | cargo test |
| SRS-NFR-04 | Any external decode backend for timed media must remain behind a narrow adapter boundary with explicit failure surfaces. | SCOPE-02 | NFR-04 | cargo test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
