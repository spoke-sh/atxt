# CLI Contract Wiring And Inspection Output - Software Design Description

> Deliver a truthful inspection-first CLI slice by aligning the public command contract, exposing shared probe and render-plan metadata, and publishing deterministic direct/degraded proofs for operator workflows.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage keeps the current top-level `atext` CLI shape but makes it truthful and inspectable. The work stays inside the existing pipeline:

1. Align the top-level usage string with the command matcher in `src/cli.rs`.
2. Make `stats` truthful by wiring the existing `render_stats()` surface into the CLI or removing it from the advertised contract in the same slice.
3. Add `inspect <path>` as a human-readable explanation path that runs `probe_path`, `detect_terminal_profile`, and `plan_render` without requiring a full media decode.
4. Add deterministic direct/degraded proofs and update docs so the public CLI contract matches the shipped implementation.

## Context & Boundaries

In scope are the public CLI contract in `src/cli.rs`, the stats renderer in `src/stats.rs`, the shared probe/terminal/render-planning contracts, and the docs/proof artifacts that explain and verify the top-level command surface.

Out of scope are new media decoders, new renderer families, a new CLI framework, and any broad redesign of the interaction model beyond this first inspection-first slice.

```
┌─────────────────────────────────────────────────────────────┐
│                         This Voyage                         │
│                                                             │
│  ┌──────────────┐   ┌──────────────────┐   ┌─────────────┐ │
│  │ CLI dispatch │→→│ inspect formatter │→→│ proofs/docs │ │
│  └──────┬───────┘   └────────┬─────────┘   └─────────────┘ │
│         │                    │                               │
│         ↓                    ↓                               │
│  ┌──────────────┐   ┌──────────────────┐                    │
│  │ stats bridge │   │ shared pipeline  │                    │
│  │ render_stats │   │ probe/terminal/  │                    │
│  │              │   │ plan surfaces    │                    │
│  └──────────────┘   └──────────────────┘                    │
└─────────────────────────────────────────────────────────────┘
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `src/cli.rs` | internal module | Existing top-level command matcher and user-facing error surface | crate-local |
| `src/media.rs` | internal module | Provides `probe_path` and `ProbeResult` for inspection output | crate-local |
| `src/terminal.rs` | internal module | Provides `detect_terminal_profile` and `TerminalProfile` | crate-local |
| `src/render.rs` | internal module | Provides `plan_render` and `RenderPlan` | crate-local |
| `src/stats.rs` | internal module | Existing stats renderer that is currently not wired into the CLI | crate-local |
| `README.md` and proof artifacts | project docs | Public contract and operator-visible evidence | repository-local |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| CLI shape | Keep the current top-level verb model | Resolves contract drift without broad CLI redesign. |
| Inspection source of truth | Reuse shared probe, terminal, and planning surfaces | Keeps the explanation path faithful to actual renderer selection. |
| Stats decision | Either wire the existing stats output or remove it from the advertised surface in the same slice | A partially advertised operator command is worse than no command. |
| Proof posture | Capture direct and degraded inspection output as deterministic text artifacts | Keeps the new command reviewable in CI, ssh, and agent workflows. |

## Architecture

The design centers on a thin operator-inspection layer on top of the existing pipeline:

- `run_cli` remains the single top-level command entrypoint.
- A new `inspect` branch collects `ProbeResult`, `TerminalProfile`, and `RenderPlan` and passes them to a formatter that emits stable human-readable text.
- `stats` becomes an explicit CLI branch if retained.
- Existing render paths remain unchanged except for command-contract alignment work.

## Components

### CLI Dispatch

Purpose: keep the public command surface truthful and route top-level verbs to the correct implementation.

Behavior:
- match `render`, `screen`, `globe`, retained/removed `stats`, and new `inspect`
- keep usage text aligned with dispatch
- preserve current error semantics for usage and runtime failures

### Inspection Formatter

Purpose: turn shared pipeline state into an operator-readable explanation.

Behavior:
- display media kind, probe completeness, and available metadata
- display terminal-profile facts that influence degradation
- display selected render mode, output kind, degraded state, and planning reason
- avoid full decode/render work unless a later slice explicitly needs it

### Stats Bridge

Purpose: make the truth of the stats surface explicit.

Behavior:
- if retained, call `render_stats()` and return its text through the existing CLI error boundary
- if removed, clean the usage/docs surface in the same slice

### Proof And Doc Alignment

Purpose: keep the operator contract reviewable.

Behavior:
- add CLI tests for command-contract alignment and inspect output
- add direct/degraded captures for representative inputs
- update README and related docs so examples match the implemented command set

## Interfaces

Top-level CLI additions remain narrow:

- `atext stats` if retained
- `atext inspect <path>`

Expected `inspect` sections:

- asset path
- media classification and probe completeness
- probe metadata
- terminal profile summary
- render plan summary

The interface is human-readable text first; no new stable machine contract is introduced in this voyage.

## Data Flow

`inspect <path>`:

1. CLI captures the terminal environment.
2. `probe_path(path)` classifies the asset and collects metadata.
3. `detect_terminal_profile(env)` derives the session constraints.
4. `plan_render(probe, terminal)` computes the chosen output strategy.
5. The formatter emits one stable text summary combining those shared surfaces.

`stats` if retained:

1. CLI routes `stats`.
2. `render_stats()` reads the existing board-history artifact.
3. Output returns through the normal CLI success/error surface.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Public command mismatch | CLI tests or usage assertions fail | Treat as a contract bug and fix usage/dispatch in the same slice | Align advertised commands with the matcher |
| Inspection lacks enough metadata to explain a fallback | Missing probe or terminal fields in formatter output | Emit explicit `unknown` or partial values rather than inventing detail | Extend shared metadata only if current contracts are insufficient |
| `stats` output is unusable or obsolete | Tests or manual review show it does not represent a stable operator need | Remove it from public usage/docs rather than shipping a misleading command | Revisit in a later mission if demand proves real |
| Captured-session output drifts from expectations | Direct/degraded proof captures or tests fail | Treat as a verification regression | Update implementation or expectations intentionally |
