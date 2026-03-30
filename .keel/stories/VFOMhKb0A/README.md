---
# system-managed
id: VFOMhKb0A
status: done
created_at: 2026-03-30T15:22:16
updated_at: 2026-03-30T15:36:15
# authored
title: Implement ANSI Stream Delta-Encoding
type: feat
operator-signal:
scope: VFOMXUMN2/VFOMcaRRV
index: 2
started_at: 2026-03-30T15:35:22
completed_at: 2026-03-30T15:36:15
---

# Implement ANSI Stream Delta-Encoding

## Summary

This story implements an ANSI stream optimizer that calculates the difference (delta) between consecutive frames. By only emitting ANSI escape sequences for cells that have changed color or character, the total characters sent per frame is drastically reduced, preventing flickering and terminal update overhead.

## Acceptance Criteria

- [x] [SRS-02/AC-01] State tracker buffers the cell state of the previous frame. <!-- verify: cargo test, SRS-02:start:end -->
- [x] [SRS-02/AC-02] Renderer only emits characters for cells that differ from the buffer. <!-- verify: cargo test, SRS-02:start:end -->
- [x] [SRS-NFR-01/AC-03] ANSI stream compression achieves a >30% reduction in total characters for standard animations. <!-- verify: cargo test, SRS-NFR-01:start:end -->
