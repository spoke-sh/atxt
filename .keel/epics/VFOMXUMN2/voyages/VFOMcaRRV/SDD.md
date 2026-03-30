# Smooth Playback Engine - Software Design Description

> Implement frame-aware delays and adaptive scaling for cinematic terminal playback.

**SRS:** [SRS.md](SRS.md)

## Overview

The Smooth Playback Engine enhances the media rendering experience by ensuring that animated content plays at the correct speed and fits perfectly within the terminal's viewport. It also introduces data optimization via ANSI delta-encoding to prevent "tearing" and flickering caused by excessive terminal updates.

## Context & Boundaries

The engine sits between the media decoding layer (e.g., GIF/video frames) and the terminal output layer. It interacts with `TerminalEnvironment` to determine available space.

```
┌─────────────────────────────────────────┐
│              Smooth Playback Engine     │
│                                         │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │ Scaling │→ │ Timing  │→ │ Delta   │ │
│  │ Logic   │  │ Control │  │ Encoder │ │
│  └─────────┘  └─────────┘  └─────────┘ │
└─────────────────────────────────────────┘
        ↑               ↑             ↓
   [Media Source]  [Clock/Timer]   [Terminal]
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `image`    | Crate | GIF decoding and metadata extraction | |
| `tokio`    | Crate | Async sleep/scheduling for timing | |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Delta Encoding | Cell-level comparison | Minimizes bandwidth usage and reduces flickering on slower terminal connections. |

## Architecture

1. **Scaler:** Responsible for calculating target dimensions based on `TerminalEnvironment`.
2. **Scheduler:** Uses `tokio::time::sleep` (or equivalent) to wait between frame emissions based on the frame's delay.
3. **Delta Encoder:** Buffers the previous frame and only generates ANSI sequences for cells that have changed color or character.

## Components

### Scaling Engine
Automatically calculates the largest possible dimensions that fit within terminal rows/cols while maintaining the original media's aspect ratio.

### Playback Loop
A loop that iterates through decoded frames, applying the delay specified in the frame metadata before proceeding to the next.

### ANSI Optimizer
A stateful renderer that tracks the last cell state (color/character) and only emits the minimal set of ANSI escape sequences to reach the next state.

## Data Flow

1. Input: Sequence of `(Frame, Delay)`.
2. Apply `Scaling Engine` to `Frame`.
3. Feed `Scaled Frame` into `ANSI Optimizer`.
4. `ANSI Optimizer` emits optimized stream to stdout.
5. `Playback Loop` sleeps for `Delay` duration.
6. Repeat for next frame.
