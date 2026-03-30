---
# system-managed
id: VFNeyEfLY
status: backlog
created_at: 2026-03-30T12:28:35
updated_at: 2026-03-30T12:31:59
# authored
title: Add Cargo Feature Gates for CLI and Video Modules
type: refactor
operator-signal:
scope: VFNe2qYHS/VFNeIPz9b
index: 1
---

# Add Cargo Feature Gates for CLI and Video Modules

## Summary

Add `cli` and `video` Cargo features to Cargo.toml and gate the `cli`, `globe`, `stats`, and `video` module declarations in `lib.rs` behind their respective features. Make `crossterm` and `colored` optional dependencies activated by `cli`. Set `default = ["cli", "video"]` so the binary target preserves all existing behavior.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `cargo check --lib --no-default-features` compiles without the `cli`, `globe`, `stats`, or `video` modules <!-- verify: cargo check, SRS-01:start:end -->
- [ ] [SRS-02/AC-02] `crossterm` and `colored` are only required when the `cli` feature is enabled <!-- verify: cargo check, SRS-02:start:end -->
- [ ] [SRS-04/AC-03] `cargo test` with default features passes all existing tests unchanged <!-- verify: cargo test, SRS-04:start:end -->
- [ ] [SRS-05/AC-04] Feature gates are placed on `mod` declarations in `lib.rs`, not scattered within functions <!-- verify: code review, SRS-05:start:end -->
