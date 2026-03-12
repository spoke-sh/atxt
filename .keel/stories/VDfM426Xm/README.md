---
id: VDfM426Xm
title: Add Still Image CLI and Verification Fixtures
type: feat
status: backlog
created_at: 2026-03-12T08:08:38
updated_at: 2026-03-12T08:09:10
operator-signal: 
scope: VDfKtP4Yp/VDfLYKNEX
index: 2
---

# Add Still Image CLI and Verification Fixtures

## Summary

Add the first user-facing still-image render command and the fixture-backed proof set that demonstrates both direct braille output and degraded ASCII output from the shared pipeline.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] A thin CLI command renders a still image from a filesystem path using the shared probe, detect, plan, decode, and render pipeline <!-- verify: cargo test, SRS-05:start:end -->
- [ ] [SRS-06/AC-02] Representative fixtures and reviewable proofs exist for both direct braille output and degraded ASCII fallback output <!-- verify: vhs, SRS-06:start:end -->
- [ ] [SRS-NFR-04/AC-03] The default CLI path works without mandatory renderer-specific flags in ordinary terminals and captured sessions <!-- verify: cargo test, SRS-NFR-04:start:end -->
