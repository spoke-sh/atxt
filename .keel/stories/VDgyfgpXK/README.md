---
id: VDgyfgpXK
title: Implement Wav Decode And Waveform Spectrogram Transforms
type: feat
status: done
created_at: 2026-03-12T14:48:15
updated_at: 2026-03-13T09:22:29
operator-signal:
scope: VDgyaiPm5/VDgycpXSh
index: 2
started_at: 2026-03-13T09:15:52
completed_at: 2026-03-13T09:22:29
---

# Implement Wav Decode And Waveform Spectrogram Transforms

## Summary

Implement the first bounded WAV decode path plus waveform and spectrogram transforms, and connect those summaries to shared render planning so audio output stays honest across terminal capability levels.

## Acceptance Criteria

- [x] [SRS-03/AC-01] The crate can decode bounded WAV PCM input and derive waveform and spectrogram-oriented summary data from a shared audio decode boundary <!-- verify: cargo test, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-04/AC-02] Shared render planning can route audio summaries to waveform or spectrogram output without audio-local terminal heuristics <!-- verify: cargo test, SRS-04:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-03/AC-03] The first audio slice stays bounded by explicit sample, window, and bin budgets and does not require playback support <!-- verify: cargo test, SRS-NFR-03:start:end, proof: ac-3.log-->
