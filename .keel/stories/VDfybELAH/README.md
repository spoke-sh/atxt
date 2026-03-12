---
id: VDfybELAH
title: Add Timed Sequence CLI Path And Mission Proofs
type: feat
status: done
created_at: 2026-03-12T10:41:42
updated_at: 2026-03-12T11:14:18
operator-signal: 
scope: VDfyDRgBo/VDfyYRyVs
index: 2
started_at: 2026-03-12T11:05:03
completed_at: 2026-03-12T11:14:18
---

# Add Timed Sequence CLI Path And Mission Proofs

## Summary

Expose the timed summary pipeline through the CLI and publish the canonical mission proof so operators can inspect the same timed fixture through direct and degraded terminal paths from `just mission`.

## Acceptance Criteria

- [x] [SRS-05/AC-01] The CLI can render a timed visual input from a filesystem path through the shared probe, planning, decode, summary, and still-image render pipeline without mandatory sequence-specific flags <!-- verify: cargo test cli::tests::cli_renders_timed_fixture -- --nocapture, SRS-05:start:end, proof: ac-1.log-->
- [x] [SRS-06/AC-02] The repo includes a representative timed-media fixture and reviewable proofs for both direct and degraded timed summary output paths <!-- verify: test -f /home/alex/workspace/spoke-sh/atext/src/testdata/half-swap.gif && test -f /home/alex/workspace/spoke-sh/atext/.keel/stories/VDfybELAH/EVIDENCE/direct.txt && test -f /home/alex/workspace/spoke-sh/atext/.keel/stories/VDfybELAH/EVIDENCE/ascii.txt && head -n 160 /home/alex/workspace/spoke-sh/atext/.keel/stories/VDfybELAH/EVIDENCE/mission.txt >/dev/null, SRS-06:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-03] `just mission` exposes the timed-sequence operator signal from the primary human entrypoint without introducing sequence-local terminal heuristics <!-- verify: cd /home/alex/workspace/spoke-sh/atext && just mission >/dev/null, SRS-NFR-01:start:end, proof: ac-3.log-->
