---
# system-managed
id: VFDjQhzce
status: backlog
created_at: 2026-03-28T19:43:35
updated_at: 2026-03-28T19:44:37
# authored
title: Add Shared Inspect Command For Media Planning
type: feat
operator-signal:
scope: VFDYN9LOV/VFDj0ZOQj
index: 2
---

# Add Shared Inspect Command For Media Planning

## Summary

Add a first-class `inspect <path>` command that explains what `atext` detected about an asset and why shared planning chose the current render strategy.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] `atext inspect <path>` reports media kind, probe completeness, and relevant metadata for representative supported inputs without requiring source inspection <!-- verify: cargo test, SRS-03:start:end -->
- [ ] [SRS-04/AC-02] The inspect output includes terminal-profile and render-plan context, including session mode, Unicode reliability, chosen render mode, degraded state, output kind, and planning reason <!-- verify: cargo test, SRS-04:start:end -->
- [ ] [SRS-NFR-02/AC-03] The inspect command derives its explanation data from the shared `ProbeResult`, `TerminalProfile`, and `RenderPlan` contracts rather than duplicating renderer-local logic in the CLI <!-- verify: cargo test, SRS-NFR-02:start:end -->
