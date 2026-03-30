# Library Rendering API and Downstream Proof - SRS

## Summary

Epic: VFNe2qYHS
Goal: Deliver a high-level rendering entry point and a proof program demonstrating downstream library usage without CLI features or ffprobe.

## Scope

### In Scope

- [SCOPE-01] A high-level `render_to_text` function that chains probe → plan → decode → render for a given path and terminal profile.
- [SCOPE-02] Support for still images, animated images, and audio in the high-level entry point.
- [SCOPE-03] A proof example program that depends on `atext` as a library (no `cli` feature) and renders canonical fixtures.

### Out of Scope

- [SCOPE-04] Video rendering in the high-level entry point (requires `video` feature and external tools).
- [SCOPE-05] Streaming or progressive rendering APIs.
- [SCOPE-06] Changes to existing renderer implementations.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The crate must expose a high-level rendering function that accepts a filesystem path and terminal profile and returns the rendered text output. | SCOPE-01 | FR-04 | cargo test |
| SRS-02 | The high-level function must handle still images, animated images (GIF), and audio files by routing through the existing probe, plan, decode, and render pipeline. | SCOPE-02 | FR-05 | cargo test |
| SRS-03 | The high-level function must derive renderer choice from `plan_render` and the shared terminal profile, not introduce a parallel decision path. | SCOPE-01 | NFR-03 | code review / cargo test |
| SRS-04 | The repo must include a proof example that depends on `atext` without the `cli` feature, renders a canonical still image and audio fixture, and compiles without ffprobe. | SCOPE-03 | FR-06 | cargo build --example |
| SRS-05 | The high-level function must return a typed error that surfaces probe, decode, and render failures without losing context. | SCOPE-01 | FR-04 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The high-level function must use the existing pipeline functions, not reimplement any step. | SCOPE-01 | NFR-03 | code review |
| SRS-NFR-02 | The proof example must compile and run in CI without interactive terminal or ffprobe. | SCOPE-03 | FR-06 | CI proof log |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
