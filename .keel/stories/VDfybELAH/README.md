---
id: VDfybELAH
title: Add Timed Sequence CLI Path And Mission Proofs
type: feat
status: backlog
created_at: 2026-03-12T10:41:42
updated_at: 2026-03-12T10:46:28
operator-signal: 
scope: VDfyDRgBo/VDfyYRyVs
index: 2
---

# Add Timed Sequence CLI Path And Mission Proofs

## Summary

Expose the timed summary pipeline through the CLI and publish the canonical mission proof so operators can inspect the same timed fixture through direct and degraded terminal paths from `just mission`.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] The CLI can render a timed visual input from a filesystem path through the shared probe, planning, decode, summary, and still-image render pipeline without mandatory sequence-specific flags <!-- verify: cargo test / manual, SRS-05:start:end -->
- [ ] [SRS-06/AC-02] The repo includes a representative timed-media fixture and reviewable proofs for both direct and degraded timed summary output paths <!-- verify: manual / llm-judge, SRS-06:start:end -->
- [ ] [SRS-NFR-01/AC-03] `just mission` exposes the timed-sequence operator signal from the primary human entrypoint without introducing sequence-local terminal heuristics <!-- verify: manual, SRS-NFR-01:start:end -->
