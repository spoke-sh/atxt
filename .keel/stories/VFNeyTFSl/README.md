---
# system-managed
id: VFNeyTFSl
status: done
created_at: 2026-03-30T12:28:36
updated_at: 2026-03-30T14:21:32
# authored
title: Gate Video Probe Path in Media Module
type: refactor
operator-signal:
scope: VFNe2qYHS/VFNeIPz9b
index: 2
started_at: 2026-03-30T14:19:43
completed_at: 2026-03-30T14:21:32
---

# Gate Video Probe Path in Media Module

## Summary

Gate the `ffprobe` process invocation in `media.rs` behind `#[cfg(feature = "video")]` so that `probe_path` compiles and works for image and audio inputs without the video feature. Video-like inputs return `MediaKind::Unknown` when the feature is disabled.

## Acceptance Criteria

- [x] [SRS-03/AC-01] `cargo test --lib --no-default-features` passes for still image and audio probing paths without ffprobe available <!-- verify: cargo test, SRS-03:start:end -->
- [x] [SRS-03/AC-02] Video-like inputs probed without the `video` feature return `MediaKind::Unknown` rather than panicking or attempting to spawn ffprobe <!-- verify: cargo test, SRS-03:start:end -->
