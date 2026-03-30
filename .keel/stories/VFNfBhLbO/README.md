---
# system-managed
id: VFNfBhLbO
status: done
created_at: 2026-03-30T12:29:27
updated_at: 2026-03-30T14:25:46
# authored
title: Add Downstream Library Proof Example
type: feat
operator-signal:
scope: VFNe2qYHS/VFNeJVpOI
index: 2
started_at: 2026-03-30T14:24:32
completed_at: 2026-03-30T14:25:46
---

# Add Downstream Library Proof Example

## Summary

Add a Cargo example program (`examples/render_demo.rs`) that depends on `atext` as a library without the `cli` feature, constructs a synthetic terminal profile, and renders canonical still image and audio fixtures to stdout using the `render_to_text` entry point. This proves the library surface works end-to-end from a downstream consumer's perspective.

## Acceptance Criteria

- [x] [SRS-04/AC-01] An example program exists in `examples/` that compiles with `cargo build --example render_demo --no-default-features` <!-- verify: cargo build --example render_demo --no-default-features, SRS-04:start:end -->
- [x] [SRS-04/AC-02] The example renders at least one still image and one audio fixture to stdout using a synthetic terminal profile <!-- verify: cargo run --example render_demo --no-default-features, SRS-04:start:end -->
- [x] [SRS-NFR-02/AC-03] The example compiles and runs in CI without an interactive terminal or ffprobe on PATH <!-- verify: cargo run --example render_demo --no-default-features, SRS-NFR-02:start:end -->
