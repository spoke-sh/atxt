---
# system-managed
id: VFNfBamXi
status: done
created_at: 2026-03-30T12:29:27
updated_at: 2026-03-30T14:24:22
# authored
title: Implement High-Level Render To Text Entry Point
type: feat
operator-signal:
scope: VFNe2qYHS/VFNeJVpOI
index: 1
started_at: 2026-03-30T14:21:42
completed_at: 2026-03-30T14:24:22
---

# Implement High-Level Render To Text Entry Point

## Summary

Add a `render_to_text` convenience function to the crate root that chains probe → plan → decode → render for a given filesystem path and terminal profile. Introduce a unified `RenderError` enum that wraps probe, decode, and render failures. The function must cover still images, animated images, and audio without introducing a parallel decision path.

## Acceptance Criteria

- [x] [SRS-01/AC-01] A `render_to_text(path, profile)` function exists at the crate root and returns `Result<String, RenderError>` <!-- verify: cargo test, SRS-01:start:end -->
- [x] [SRS-02/AC-02] The function successfully renders still images, animated images (GIF), and audio files through the existing pipeline <!-- verify: cargo test, SRS-02:start:end -->
- [x] [SRS-03/AC-03] Renderer selection is derived from `plan_render` and the shared terminal profile, not a parallel decision path <!-- verify: code review, SRS-03:start:end -->
- [x] [SRS-05/AC-04] The `RenderError` enum preserves source error context for probe, decode, and render failures <!-- verify: cargo test, SRS-05:start:end -->
