# Library-First Crate Surface - Product Requirements

## Problem Statement

Downstream crates cannot depend on atext as a library without pulling in CLI-only modules, and the public surface lacks a high-level rendering entry point, forcing consumers to manually chain the probe-plan-decode-render pipeline.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make `atext` usable as a direct Rust dependency for downstream crates that only need the media-to-text rendering pipeline. | A downstream crate can compile with `atext` as a dependency without CLI modules or `ffprobe` on `$PATH`. | A proof program demonstrates the library surface end-to-end. |
| GOAL-02 | Provide a single-call rendering entry point so library consumers do not need to understand pipeline internals. | A consumer can render a media file to text with one function call and a terminal profile. | The entry point covers still image, animated image, and audio paths. |
| GOAL-03 | Keep the existing CLI and binary target fully functional with no behavior changes. | All existing tests and CLI proofs continue to pass after feature-gating. | `cargo test` green, `atext render` and `atext screen` unchanged. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Library Integrator | A Rust crate (e.g. keel) embedding atxt for terminal rendering without an external binary. | A clean library dependency with no CLI or external tool baggage. |
| CLI Operator | Existing users of the `atext` binary. | No regressions in CLI behavior after the refactor. |

## Scope

### In Scope

- [SCOPE-01] Cargo feature gates that separate CLI-only modules (`cli`, `globe`, `stats`) from the core rendering pipeline.
- [SCOPE-02] An opt-in feature gate for `ffprobe`-dependent video probing so library consumers without `ffprobe` can still compile and use image/audio paths.
- [SCOPE-03] A high-level `render_to_text` (or equivalent) convenience function that chains probe → plan → decode → render for library consumers.
- [SCOPE-04] A proof program that depends on `atext` as a library and renders canonical fixtures without the `cli` feature or `ffprobe`.

### Out of Scope

- [SCOPE-05] Crate registry publication or versioning for public distribution.
- [SCOPE-06] Splitting into multiple crates (e.g. `atxt-core`).
- [SCOPE-07] Changes to the normalized media model, pipeline architecture, or renderer implementations.
- [SCOPE-08] New media format support or rendering capabilities.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The crate must compile as a library without the `cli`, `globe`, and `stats` modules when the `cli` feature is not enabled. | GOAL-01 | must | Prevents downstream consumers from pulling in CLI-specific dependencies and code. |
| FR-02 | The crate must compile and pass tests for still image and audio rendering paths when `ffprobe` is not available, gated behind an opt-in `video` feature. | GOAL-01 | must | Removes the external binary dependency for consumers that do not need video support. |
| FR-03 | The binary target must enable the `cli` and `video` features by default so existing CLI behavior is unchanged. | GOAL-03 | must | Prevents regressions for current CLI users. |
| FR-04 | The crate must expose a high-level rendering function that accepts a filesystem path and terminal profile and returns rendered text, without requiring the consumer to manually orchestrate the pipeline. | GOAL-02 | must | The current surface requires chaining 4+ function calls with intermediate types; a single entry point makes library integration practical. |
| FR-05 | The high-level rendering function must support still images, animated images, and audio files. | GOAL-02 | must | These are the media paths available without `ffprobe`, covering the primary downstream use case. |
| FR-06 | The repo must include a proof example that depends on `atext` as a library (without `cli` feature), renders a canonical fixture, and compiles without `ffprobe`. | GOAL-01 GOAL-02 | must | Proves the library surface works end-to-end from a consumer's perspective. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The feature-gate boundaries must align with module boundaries so conditional compilation is clean `cfg(feature)` on `mod` declarations, not scattered `cfg` within functions. | GOAL-01 | must | Keeps the feature-gate maintainable and prevents accidental coupling. |
| NFR-02 | All existing tests must pass with default features (CLI + video) enabled. | GOAL-03 | must | No regressions in existing behavior. |
| NFR-03 | The high-level rendering function must derive renderer choice from the shared terminal profile and render-planning contract, not introduce a parallel decision path. | GOAL-02 | must | Preserves architecture and keeps fallback policy centralized. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Feature-gate compilation | `cargo check --lib --no-default-features` succeeds | CI or local proof log |
| CLI regression | Full `cargo test` with default features | Test suite green |
| Library entry point | Proof example compiles and produces expected output | Example program output artifact |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Module-level feature gates are sufficient; no intra-module conditional compilation is needed. | May need finer-grained `cfg` attributes within modules that mix CLI and core logic. | Audit module imports during voyage design. |
| `ffprobe` usage is isolated to `media.rs` video probing and `video.rs`. | Other modules may transitively depend on video metadata. | Grep for `ffprobe` and `Command::new` usage. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Does `crossterm` (used by `globe`) pull in dependencies that library consumers should avoid? | Epic owner | Open |
| Should the high-level API return a `String` or accept a `Write` sink for streaming? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `cargo check --lib --no-default-features` compiles without CLI modules or ffprobe dependency.
- [ ] A proof example renders a still image and audio file to text using only the library surface.
- [ ] All existing tests pass with default features enabled.
<!-- END SUCCESS_CRITERIA -->
