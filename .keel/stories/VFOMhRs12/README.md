---
# system-managed
id: VFOMhRs12
status: done
created_at: 2026-03-30T15:22:16
updated_at: 2026-03-30T15:36:16
# authored
title: Auto-Scale Media To Terminal Dimensions
type: feat
operator-signal:
scope: VFOMXUMN2/VFOMcaRRV
index: 3
started_at: 2026-03-30T15:35:22
completed_at: 2026-03-30T15:36:16
---

# Auto-Scale Media To Terminal Dimensions

## Summary

This story enables `atxt` to automatically adjust the output dimensions of rendered media to fit within the terminal's viewport (columns and rows), preventing layout breakage.

## Acceptance Criteria

- [x] [SRS-03/AC-01] System retrieves rows/cols from `TerminalEnvironment` before rendering. <!-- verify: cargo test, SRS-03:start:end -->
- [x] [SRS-03/AC-02] Render dimensions are calculated to maximize fit without exceeding terminal bounds while preserving aspect ratio. <!-- verify: cargo test, SRS-03:start:end -->
