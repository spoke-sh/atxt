---
id: VDfBzGvhS
title: Implement Input Probing and Terminal Environment Detection
type: feat
status: done
created_at: 2026-03-12T07:28:36
updated_at: 2026-03-12T07:47:31
operator-signal: 
scope: VDfBvLf8x/VDfBwYad0
index: 2
started_at: 2026-03-12T07:43:25
completed_at: 2026-03-12T07:47:31
---

# Implement Input Probing and Terminal Environment Detection

## Summary

Implement the first probing and capability detection entry points so atext can classify both the input asset and the terminal session before render planning chooses defaults and fallbacks.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Input probing returns canonical probe data for the initial media families needed by the planning slice <!-- verify: cargo test, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Terminal capability detection derives a shared profile from environment and terminal signals for local, tmux, ssh-like, and low-capability sessions <!-- verify: cargo test, SRS-03:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-03] Ambiguous or hostile environment signals force conservative fallback decisions instead of optimistic renderer selection <!-- verify: cargo test, SRS-NFR-01:start:end, proof: ac-3.log-->
