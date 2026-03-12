---
id: VDgyfgTXQ
title: Add Audio CLI Path And Mission Proofs
type: feat
status: backlog
created_at: 2026-03-12T14:48:15
updated_at: 2026-03-12T14:52:14
operator-signal:
scope: VDgyaiPm5/VDgycpXSh
index: 1
---

# Add Audio CLI Path And Mission Proofs

## Summary

Expose the audio summary pipeline through the CLI and publish the canonical mission proof so operators can inspect the same audio fixture through direct and degraded terminal paths from `just mission`.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] The CLI can render an audio input from a filesystem path through the shared probe, planning, decode, transform, and audio render pipeline without mandatory audio-specific flags <!-- verify: cargo test / manual, SRS-05:start:end -->
- [ ] [SRS-06/AC-02] The repo includes a representative WAV fixture and reviewable proofs for both direct and degraded audio summary output paths <!-- verify: manual / llm-judge, SRS-06:start:end -->
- [ ] [SRS-NFR-01/AC-03] `just mission` exposes the audio operator signal from the primary human entrypoint without introducing audio-local terminal heuristics <!-- verify: manual, SRS-NFR-01:start:end -->
