# VOYAGE REPORT: Feature-Gate CLI and External Dependencies

## Voyage Metadata
- **ID:** VFNeIPz9b
- **Epic:** VFNe2qYHS
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Add Cargo Feature Gates for CLI and Video Modules
- **ID:** VFNeyEfLY
- **Status:** done

#### Summary
Add `cli` and `video` Cargo features to Cargo.toml and gate the `cli`, `globe`, `stats`, and `video` module declarations in `lib.rs` behind their respective features. Make `crossterm` and `colored` optional dependencies activated by `cli`. Set `default = ["cli", "video"]` so the binary target preserves all existing behavior.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `cargo check --lib --no-default-features` compiles without the `cli`, `globe`, `stats`, or `video` modules <!-- verify: cargo check, SRS-01:start:end -->
- [x] [SRS-02/AC-02] `crossterm` and `colored` are only required when the `cli` feature is enabled <!-- verify: cargo check, SRS-02:start:end -->
- [x] [SRS-04/AC-03] `cargo test` with default features passes all existing tests unchanged <!-- verify: cargo test, SRS-04:start:end -->
- [x] [SRS-05/AC-04] Feature gates are placed on `mod` declarations in `lib.rs`, not scattered within functions <!-- verify: code review, SRS-05:start:end -->

### Gate Video Probe Path in Media Module
- **ID:** VFNeyTFSl
- **Status:** done

#### Summary
Gate the `ffprobe` process invocation in `media.rs` behind `#[cfg(feature = "video")]` so that `probe_path` compiles and works for image and audio inputs without the video feature. Video-like inputs return `MediaKind::Unknown` when the feature is disabled.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `cargo test --lib --no-default-features` passes for still image and audio probing paths without ffprobe available <!-- verify: cargo test, SRS-03:start:end -->
- [x] [SRS-03/AC-02] Video-like inputs probed without the `video` feature return `MediaKind::Unknown` rather than panicking or attempting to spawn ffprobe <!-- verify: cargo test, SRS-03:start:end -->

### Verify No-Default-Features Library Compilation
- **ID:** VFNeyhrWR
- **Status:** done

#### Summary
Add a CI-friendly verification step that confirms `cargo check --lib --no-default-features` succeeds and that existing tests pass with default features. This story closes voyage 1 by proving the feature gates work end-to-end.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `cargo check --lib --no-default-features` succeeds with no compilation errors <!-- verify: cargo check, SRS-01:start:end -->
- [x] [SRS-NFR-02/AC-02] `cargo test` with default features remains green with no regressions <!-- verify: cargo test, SRS-NFR-02:start:end -->


