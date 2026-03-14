---
id: VDsSLvO21
title: Implement Multimodal Video Decode With FFmpeg And Symphonia
type: feat
status: done
created_at: 2026-03-14T13:55:09
updated_at: 2026-03-14T14:04:56
operator-signal:
scope: VDsSA7qTy/VDsSGo91D
index: 2
completed_at: 2026-03-14T14:04:56
---

# Implement Multimodal Video Decode With FFmpeg And Symphonia

## Summary

Implement the logic to extract synchronized frames (via FFmpeg) and audio waveform (via Symphonia) from a video file.

## Acceptance Criteria

- [x] [SRS-03/AC-01] A representative video slice is decoded into a `VisualFrame` (contact sheet) and an `AudioSummary` (waveform). <!-- verify: cargo test, SRS-03:start:end -->
- [x] [SRS-NFR-02/AC-02] Extraction stays within the 16,384 sample and 4-8 frame budget. <!-- verify: cargo test, SRS-NFR-02:start:end -->
