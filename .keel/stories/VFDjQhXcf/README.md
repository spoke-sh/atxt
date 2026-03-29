---
# system-managed
id: VFDjQhXcf
status: backlog
created_at: 2026-03-28T19:43:35
updated_at: 2026-03-28T19:44:37
# authored
title: Publish Inspection Proofs And Align Surface Docs
type: docs
operator-signal:
scope: VFDYN9LOV/VFDj0ZOQj
index: 1
---

# Publish Inspection Proofs And Align Surface Docs

## Summary

Publish deterministic operator-visible proofs for the inspection-oriented CLI surface and update user-facing docs so they match the shipped command contract.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] The repo includes reviewable direct and degraded proof captures for the inspection-oriented CLI surface, covering the new command contract and the chosen truth of `stats` <!-- verify: manual, SRS-05:start:end -->
- [ ] [SRS-NFR-01/AC-02] The proof path remains deterministic and readable in captured terminal environments rather than relying on a live interactive shell <!-- verify: manual, SRS-NFR-01:start:end -->
- [ ] [SRS-05/AC-03] README and adjacent operator docs describe the implemented top-level CLI surface and inspection flow without advertising commands that the binary does not dispatch <!-- verify: manual, SRS-05:start:end -->
