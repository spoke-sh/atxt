---
# system-managed
id: VFDjQiMcl
status: backlog
created_at: 2026-03-28T19:43:35
updated_at: 2026-03-28T19:44:37
# authored
title: Align CLI Contract And Wire Stats Surface
type: feat
operator-signal:
scope: VFDYN9LOV/VFDj0ZOQj
index: 3
---

# Align CLI Contract And Wire Stats Surface

## Summary

Make the top-level CLI contract truthful by aligning the usage string with the dispatcher and resolving the current `stats` mismatch in the same change slice.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] The top-level CLI usage text advertises only commands that the dispatcher actually supports, with regression coverage for invalid-argument and top-level command handling <!-- verify: cargo test cli::tests::cli_reports_usage_for_invalid_arguments -- --nocapture, SRS-01:start:end -->
- [ ] [SRS-02/AC-02] The existing stats surface is either wired into the CLI as a working top-level command or removed from the public contract in the same slice <!-- verify: cargo test, SRS-02:start:end -->
- [ ] [SRS-NFR-03/AC-03] The contract-alignment change lands within the current CLI shape without redesigning the parser or changing the existing `render`, `screen`, or `globe` interaction model <!-- verify: cargo test, SRS-NFR-03:start:end -->
