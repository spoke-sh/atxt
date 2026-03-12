---
id: VDfM40eXl
title: Implement Txtplot Braille Renderer and ASCII Fallback
type: feat
status: backlog
created_at: 2026-03-12T08:08:38
updated_at: 2026-03-12T08:09:10
operator-signal: 
scope: VDfKtP4Yp/VDfLYKNEX
index: 1
---

# Implement Txtplot Braille Renderer and ASCII Fallback

## Summary

Implement the first still-image renderers by adapting the shared frame model into txtplot-backed braille output for direct paths and an internal ASCII fallback for degraded paths, all selected through the shared render-planning contract.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] A txtplot-backed braille renderer can turn a shared still-image frame into terminal output when shared planning selects the direct image path <!-- verify: cargo test, SRS-03:start:end -->
- [ ] [SRS-04/AC-02] A deterministic ASCII fallback renderer exists for still-image frames when planning degrades away from braille output <!-- verify: cargo test, SRS-04:start:end -->
- [ ] [SRS-NFR-01/AC-03] Renderer selection continues to derive from shared `RenderPlan` and `TerminalProfile` surfaces rather than backend-local terminal heuristics <!-- verify: cargo test, SRS-NFR-01:start:end -->
- [ ] [SRS-NFR-03/AC-04] txtplot integration stays behind a narrow adapter boundary instead of redefining the still-image media contract <!-- verify: cargo test, SRS-NFR-03:start:end -->
