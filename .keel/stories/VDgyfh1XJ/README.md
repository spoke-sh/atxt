---
id: VDgyfh1XJ
title: Define Audio Summary Model And Probe Metadata
type: feat
status: backlog
created_at: 2026-03-12T14:48:15
updated_at: 2026-03-12T14:52:14
operator-signal:
scope: VDgyaiPm5/VDgycpXSh
index: 3
---

# Define Audio Summary Model And Probe Metadata

## Summary

Define the shared audio-summary contract for waveform and spectrogram-oriented data, then extend probing so the first decoded audio format carries enough metadata to drive bounded planning and rendering.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] A shared audio-summary model exists for normalized waveform and spectrogram bins without exposing decode-backend types in the public media contract <!-- verify: cargo test, SRS-01:start:end -->
- [ ] [SRS-02/AC-02] `probe_path` classifies WAV inputs with enough audio metadata to drive bounded decode and audio render planning <!-- verify: cargo test, SRS-02:start:end -->
- [ ] [SRS-NFR-02/AC-03] The audio-summary model and probe metadata remain deterministic and testable without requiring a live interactive terminal session <!-- verify: cargo test, SRS-NFR-02:start:end -->
