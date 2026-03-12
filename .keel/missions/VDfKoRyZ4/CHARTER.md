# First Static Image Rendering Vertical Slice - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Deliver the first end-to-end static image rendering slice for atxt so a still image can be probed, decoded into a canonical visual frame, rendered through txtplot-backed braille/ASCII output, and inspected from a terminal-facing CLI with deterministic fixtures and explicit fallbacks. | board: VDfKtP4Yp |

## Constraints
- Rust-first implementation; use txtplot as the braille rendering backend for the still-image slice; preserve one canonical probe-to-render pipeline; keep fallback behavior truthful in tmux, ssh, and captured sessions; avoid format-specific bypasses around shared probing, terminal detection, and render planning; keep the first slice limited to static images, not GIF/video/audio.
## Halting Rules
- Do not halt while MG-01 has unfinished epic, voyage, or story work for static-image decoding, normalized visual-frame modeling, txtplot-backed rendering, CLI exposure, or fixture verification. Halt when the mission-linked board work is complete and only manual sign-off remains. Yield to human when progress is blocked by a required txtplot capability that does not exist yet or by a product decision about whether the first slice must ship color output or a CLI.
