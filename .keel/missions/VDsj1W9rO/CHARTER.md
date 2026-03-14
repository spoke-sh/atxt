# Interactive Strategic Navigation Console - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Establish a live interactive TTY loop for the 3D Drift Globe. | board:VDsj0I9KK |
| MG-02 | Enable fluid 3D rotation controlled by arrow keys. | board:VDsj0IML7 |
| MG-03 | Show real-time POI metadata during navigation. | board:VDsj0IcMw |

## Strategy

1. Implement a `run_interactive` loop in `src/cli.rs`.
2. Use `crossterm` (if available) or raw ANSI escape codes for input and cursor control.
3. Hook the loop into the existing `render_drift_globe` function.

## Constraints

- Interaction must be flicker-free in standard terminal emulators.
- `atext` must gracefully fallback to static rendering if `stdout_is_tty` is false.
- The binary size should remain bounded (prefer raw ANSI if possible).

## Halting Rules

- STOP if interactive mode significantly increases binary startup latency.
- YIELD to human for initial evaluation of "feel" and rotation speed.
