---
id: VDfybFOA6
title: Define Timed Sequence Model And Probe Metadata
type: feat
status: done
created_at: 2026-03-12T10:41:42
updated_at: 2026-03-12T10:57:46
operator-signal: 
scope: VDfyDRgBo/VDfyYRyVs
index: 3
started_at: 2026-03-12T10:53:36
completed_at: 2026-03-12T10:57:46
---

# Define Timed Sequence Model And Probe Metadata

## Summary

Define the shared timed-sequence model for representative sampled frames and extend probe metadata so GIF and video inputs carry the timing and geometry information needed for bounded summary planning.

## Acceptance Criteria

- [x] [SRS-01/AC-01] A shared timed-sequence model exists for normalized representative frame data without exposing decode-backend types in the public media contract <!-- verify: cargo test, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-02] The probe path classifies the first timed visual input families with enough metadata to drive bounded sequence sampling and summary planning <!-- verify: cargo test, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-03] The model and probe metadata path remain deterministic and testable without requiring a live interactive terminal session <!-- verify: cargo test, SRS-NFR-02:start:end, proof: ac-3.log-->
