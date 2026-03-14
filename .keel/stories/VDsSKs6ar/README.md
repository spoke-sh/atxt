---
id: VDsSKs6ar
title: Extend Media Model and Probing for Multimodal Video
type: feat
status: done
created_at: 2026-03-14T13:54:59
updated_at: 2026-03-14T14:01:48
operator-signal:
scope: VDsSA7qTy/VDsSGo91D
index: 1
started_at: 2026-03-14T14:01:48
completed_at: 2026-03-14T14:01:48
---

# Extend Media Model and Probing for Multimodal Video

## Summary

Update `ProbeResult` and `probe_path` to handle video containers and extract both visual and audio metadata.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `probe_path` identifies `.mp4`, `.mkv`, `.avi`, `.mov` as `MediaKind::Video`. <!-- verify: cargo test, SRS-01:start:end -->
- [x] [SRS-02/AC-02] `ProbeResult` simultaneously exposes `PixelDimensions` and `AudioMetadata` for video files. <!-- verify: cargo test, SRS-02:start:end -->
