---
# system-managed
id: VFOMhRs12
status: backlog
created_at: 2026-03-30T15:22:16
updated_at: 2026-03-30T15:30:08
# authored
title: Auto-Scale Media To Terminal Dimensions
type: feat
operator-signal:
scope: VFOMXUMN2/VFOMcaRRV
index: 3
---

# Auto-Scale Media To Terminal Dimensions

## Summary

This story enables `atxt` to automatically adjust the output dimensions of rendered media to fit within the terminal's viewport (columns and rows), preventing layout breakage.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] System retrieves rows/cols from `TerminalEnvironment` before rendering. <!-- verify: automated, SRS-03:start:end -->
- [ ] [SRS-03/AC-02] Render dimensions are calculated to maximize fit without exceeding terminal bounds while preserving aspect ratio. <!-- verify: automated, SRS-03:start:end -->
