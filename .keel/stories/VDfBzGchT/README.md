---
id: VDfBzGchT
title: Define Probe and Terminal Capability Models
type: feat
status: done
created_at: 2026-03-12T07:28:36
updated_at: 2026-03-12T07:42:14
operator-signal: 
scope: VDfBvLf8x/VDfBwYad0
index: 1
started_at: 2026-03-12T07:39:59
completed_at: 2026-03-12T07:42:14
---

# Define Probe and Terminal Capability Models

## Summary

Define the shared `ProbeResult` and `TerminalProfile` surfaces so later probing, planning, and rendering work uses one canonical contract instead of format-specific or renderer-local heuristics.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `ProbeResult` captures the media family, geometry, timing, audio metadata, and explicit unknown-partial states needed for renderer planning <!-- verify: cargo test, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-02] `TerminalProfile` captures color, Unicode, animation, multiplexer, remote, and size signals required for planning decisions <!-- verify: cargo test, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-03] The model surfaces are documented or tested in a way that does not require a live interactive terminal session to validate them <!-- verify: cargo test, SRS-NFR-02:start:end, proof: ac-3.log-->
