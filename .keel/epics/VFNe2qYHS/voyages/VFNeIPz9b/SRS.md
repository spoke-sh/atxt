# Feature-Gate CLI and External Dependencies - SRS

## Summary

Epic: VFNe2qYHS
Goal: Separate CLI-only modules and ffprobe/ffmpeg dependency behind Cargo feature gates so the default library surface is clean for downstream consumers.

## Scope

### In Scope

- [SCOPE-01] Add a `cli` Cargo feature that gates the `cli`, `globe`, and `stats` modules and the `crossterm` dependency.
- [SCOPE-02] Add a `video` Cargo feature that gates the `video` module and `ffprobe`/`ffmpeg` code paths in `media.rs`.
- [SCOPE-03] Configure the binary target to enable `cli` and `video` features by default.
- [SCOPE-04] Update `lib.rs` exports to conditionally compile gated modules.

### Out of Scope

- [SCOPE-05] Changes to rendering logic, pipeline architecture, or normalized media model.
- [SCOPE-06] New API surfaces or convenience functions (covered by voyage 2).
- [SCOPE-07] Crate publication or versioning.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The crate must compile as a library without the `cli`, `globe`, and `stats` modules when the `cli` feature is disabled. | SCOPE-01 | FR-01 | cargo check --lib --no-default-features |
| SRS-02 | The `crossterm` dependency must only be required when the `cli` feature is enabled. | SCOPE-01 | FR-01 | cargo check --lib --no-default-features |
| SRS-03 | The crate must compile and pass image/audio tests when the `video` feature is disabled, gating `ffprobe` and `ffmpeg` process spawning. | SCOPE-02 | FR-02 | cargo test --lib --no-default-features |
| SRS-04 | The binary target must enable both `cli` and `video` features by default so all existing CLI behavior is preserved. | SCOPE-03 | FR-03 | cargo test (default features) |
| SRS-05 | `lib.rs` must conditionally export gated modules with `#[cfg(feature = "...")]` on `mod` declarations, not scattered within functions. | SCOPE-04 | FR-01 | code review / cargo check |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Feature-gate boundaries must align with module boundaries for clean conditional compilation. | SCOPE-01 SCOPE-02 SCOPE-04 | NFR-01 | code review |
| SRS-NFR-02 | All existing tests must pass with default features enabled. | SCOPE-03 | NFR-02 | cargo test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
