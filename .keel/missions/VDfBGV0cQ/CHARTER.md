# Input Probing and Terminal Capability Detection - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Deliver input probing and terminal capability detection for atext so media assets and terminal sessions can be classified automatically and routed to the right text renderer with explicit fallbacks. | board: VDfBvLf8x |

## Constraints
- Rust-first implementation; terminal-first reliability over tmux and ssh; auto-detect input and terminal capabilities before requiring flags; keep one canonical media-to-text pipeline and explicit degraded fallbacks; avoid format-specific bypasses around shared probing and capability logic.
## Halting Rules
- Do not halt while MG-01 has unfinished epic, voyage, or story work for probing or terminal capability detection; halt when MG-01 board verification is satisfied and only manual follow-up remains; yield to human when progress is blocked by an external decoder/backend choice or a product decision about the canonical probing/render boundary.
