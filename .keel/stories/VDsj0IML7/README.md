---
id: VDsj0IML7
title: Wire Live 3D Rotation to Navigation Chart
type: feat
status: done
created_at: 2026-03-14T15:00:58
updated_at: 2026-03-14T16:59:58
operator-signal:
scope: VDsisINGM/VDsiveHYR
index: 2
started_at: 2026-03-14T15:03:17
submitted_at: 2026-03-14T15:39:34
completed_at: 2026-03-14T16:59:58
---

# Wire Live 3D Rotation to Navigation Chart

## Summary

Connect arrow key input to the `angle_x` and `angle_y` parameters of the `render_drift_globe` function to enable interactive navigation.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Pressing Left/Right rotates the globe around the Y-axis. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Pressing Up/Down rotates the globe around the X-axis. <!-- verify: manual, SRS-03:start:end, proof: ac-2.log-->
