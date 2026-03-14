---
id: VDsj0I9KK
title: Implement Non-Blocking TTY Input and ANSI Refresh
type: feat
status: in-progress
created_at: 2026-03-14T15:00:58
updated_at: 2026-03-14T15:03:17
operator-signal:
scope: VDsisINGM/VDsiveHYR
index: 1
started_at: 2026-03-14T15:03:17
---

# Implement Non-Blocking TTY Input and ANSI Refresh

## Summary

Implement the foundational interactive loop that puts the terminal in raw mode and refreshes the globe frame using cursor escape codes.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] The CLI can capture 'q' and arrow keys without requiring Enter. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-02/AC-02] Frames are refreshed by overwriting the existing terminal area rather than appending. <!-- verify: manual, SRS-02:start:end -->
