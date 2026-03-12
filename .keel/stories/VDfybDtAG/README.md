---
id: VDfybDtAG
title: Implement Bounded Sequence Decode And Contact Sheet Transform
type: feat
status: backlog
created_at: 2026-03-12T10:41:42
updated_at: 2026-03-12T10:46:28
operator-signal: 
scope: VDfyDRgBo/VDfyYRyVs
index: 1
---

# Implement Bounded Sequence Decode And Contact Sheet Transform

## Summary

Implement the first bounded timed-sequence decode adapter and summary transform so representative GIF/video frames can be converted into one shared `VisualFrame` for the existing still-image renderer path.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] The crate can decode a bounded representative frame set for the first timed visual formats and transform it into a verification-friendly poster frame or contact sheet summary <!-- verify: cargo test, SRS-03:start:end -->
- [ ] [SRS-04/AC-02] The summary frame routes through the existing still-image renderer path selected by shared planning rather than a new timed-media terminal backend <!-- verify: cargo test, SRS-04:start:end -->
- [ ] [SRS-NFR-03/AC-03] The first timed-media slice stays bounded by an explicit sample budget and does not require general animation playback <!-- verify: cargo test, SRS-NFR-03:start:end -->
