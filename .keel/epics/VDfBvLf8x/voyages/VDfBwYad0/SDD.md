# Probe and Capability Foundations - Software Design Description

> Define the first stable media-probe model and terminal capability profile so render planning can auto-detect inputs, choose safe defaults, and degrade explicitly under tmux, ssh, and low-capability terminals.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage establishes the first stable planning substrate for atext. It splits the problem into three layers: media probing, terminal capability detection, and render planning. Media and terminal specifics are normalized early so later renderers can consume one shared contract instead of growing separate heuristics.

## Context & Boundaries

In scope for this voyage are the public data models, the first detection entry points, and the conservative fallback rules that translate uncertain signals into explicit render plans. Out of scope are deeper codec backends, full renderer implementations, and terminal-protocol-specific inline image support.

```
┌─────────────────────────────────────────┐
│  Probe + Capability Foundations         │
│                                         │
│  ┌────────────┐  ┌──────────────┐      │
│  │ ProbeResult│  │TerminalProfile│     │
│  └─────┬──────┘  └──────┬───────┘      │
│        └──────┬─────────┘              │
│               v                        │
│         Render Planning                │
└─────────────────────────────────────────┘
        ↑               ↑
   Input bytes      Env / TTY signals
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Rust standard library | runtime | environment inspection, type modeling, and deterministic tests | stable |
| Terminal environment variables | runtime signal | derive coarse capability hints such as color, tmux, and ssh presence | `TERM`, `COLORTERM`, `NO_COLOR`, `TMUX`, `SSH_CONNECTION` |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Shared media model | Use one canonical `ProbeResult` surface for early media classification. | Keeps probe logic independent from downstream renderers and future decoder backends. |
| Shared terminal model | Use one canonical `TerminalProfile` surface for environment classification. | Prevents renderer-local terminal probing from drifting into incompatible heuristics. |
| Conservative planning | Prefer explicit low-capability fallbacks when signals are ambiguous. | atext is optimized for truthful terminal inspection, not optimistic feature detection. |

## Architecture

The implementation should land as a narrow seam around the existing bootstrap modules:

- `media` owns probe-facing asset identity and metadata.
- `terminal` owns capability profiling and environment-derived signals.
- `render` owns planning choices and fallback selection.

The detection entry points should feed these shared modules rather than encoding policy inside format-specific adapters or future renderers.

## Components

`ProbeResult`

- Purpose: represent the minimum media facts needed for planning.
- Interface: media family, geometry, timing/audio fields, and explicit unknown states.
- Behavior: tolerate partial metadata and unsupported inputs without collapsing into renderer-specific assumptions.

`TerminalProfile`

- Purpose: represent the minimum terminal facts needed for safe renderer selection.
- Interface: color support, Unicode reliability, animation viability, multiplexer, remoteness, and dimensions.
- Behavior: prefer conservative defaults when signals are absent or contradictory.

`Render Planning`

- Purpose: translate `ProbeResult` and `TerminalProfile` into explicit mode choices and fallbacks.
- Interface: pure planning helpers that consume the shared models and emit a render intent.
- Behavior: keep degradation rules centralized and testable.

## Interfaces

The planned entry points for this slice are:

- a media probing function that returns `ProbeResult`
- a terminal capability detection function that returns `TerminalProfile`
- a render planning function that consumes both and emits a `RenderIntent`

These interfaces should stay pure and deterministic enough for direct unit testing.

## Data Flow

1. Probe the input asset into `ProbeResult`.
2. Detect the terminal session into `TerminalProfile`.
3. Combine both surfaces in render planning.
4. Emit an explicit render mode or fallback decision for downstream rendering.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unknown or unsupported media kind | Probe cannot classify the asset confidently | Return `MediaKind::Unknown` with partial metadata when available | Planning falls back to metadata-oriented rendering or other conservative defaults |
| Missing or contradictory terminal signals | Capability detection sees absent or conflicting env hints | Mark the uncertain fields conservatively and disable optimistic modes | Planning selects the lowest safe renderer family for the session |
| Partial metadata from probe backends | Some dimensions, timing, or audio fields are unavailable | Preserve the known fields and keep the rest `None` | Planning uses whatever is known and avoids renderer choices that require missing facts |
