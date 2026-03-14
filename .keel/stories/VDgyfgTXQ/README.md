---
id: VDgyfgTXQ
title: Add Audio CLI Path And Mission Proofs
type: feat
status: done
created_at: 2026-03-12T14:48:15
updated_at: 2026-03-13T14:25:29
operator-signal:
scope: VDgyaiPm5/VDgycpXSh
index: 1
started_at: 2026-03-13T14:00:00
completed_at: 2026-03-13T14:25:29
---

# Add Audio CLI Path And Mission Proofs

## Summary

Expose the audio summary pipeline through the CLI and publish the canonical mission proof so operators can inspect the same audio fixture through direct and degraded terminal paths from `just mission`.

## Acceptance Criteria

- [x] [SRS-05/AC-01] The CLI can render an audio input from a filesystem path through the shared probe, planning, decode, transform, and audio render pipeline without mandatory audio-specific flags <!-- verify: cargo test, SRS-05:start:end -->
- [x] [SRS-06/AC-02] The repo includes a representative WAV fixture and reviewable proofs for both direct and degraded audio summary output paths <!-- verify: just screen, SRS-06:start:end -->
- [x] [SRS-NFR-01/AC-03] `just screen` exposes the audio operator signal from the primary human entrypoint without introducing audio-local terminal heuristics <!-- verify: just screen, SRS-NFR-01:start:end -->
