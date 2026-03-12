# atext

Render anything as text.

`atext` is a Rust project for turning images, video, GIFs, audio, and eventually richer media formats into terminal-native textual renderings that survive `tmux`, `ssh`, CI logs, and agentic verification workflows.

The immediate goal is operational, not nostalgic: make arbitrary media inspectable inside the environments where agents and terminal-heavy developers already work. If the resulting representations become useful for multimodal training or world-model research later, that is a downstream question, not the product promise.

Status: bootstrap phase. This repository currently establishes the project contract, contributor workflow, and a minimal Rust crate skeleton.

## Project Navigation

This repository carries a small set of foundational documents:

- [CONSTITUTION.md](CONSTITUTION.md) - project principles, decision hierarchy, and product posture
- [ARCHITECTURE.md](ARCHITECTURE.md) - canonical pipeline, module boundaries, and extension seams
- [GUIDE.md](GUIDE.md) - practical contributor workflows
- [CONFIGURATION.md](CONFIGURATION.md) - development environment and configuration surfaces
- [RELEASE.md](RELEASE.md) - manual release checklist for the bootstrap phase

If you use Nix, `nix develop` provides the Rust toolchain plus `just`, `cargo-nextest`, `cargo-llvm-cov`, `ffmpeg`, `vhs`, `keel`, and `sift`.

## Why This Exists

Most terminal media tools break down in one of four ways:

- they depend on terminal features that disappear under `tmux`, `ssh`, or CI
- they require users to hand-tune codecs, color modes, or renderer settings
- they optimize for novelty over truthful inspection
- they treat each format as a separate product instead of normalizing them into one rendering pipeline

`atext` is intended to take the opposite approach:

- probe inputs automatically
- detect terminal capabilities automatically
- degrade gracefully when capabilities are limited
- keep one canonical media-to-text pipeline
- optimize for verification-first viewing, not pixel-perfect imitation

## Intended Input and Output Families

| Input family | Normalized form | Typical textual output |
| --- | --- | --- |
| Images | static visual frame | ASCII, blocks, Braille, dithered color |
| GIFs / video | timed visual sequence | sampled frame stream, poster frame, contact sheet |
| Audio | sampled waveform or spectral summary | waveform, spectrogram, level overview |
| Documents and other rich assets | page or structural summary | page preview, metadata summary, extracted regions |

The important product decision is that decoders are format-specific, but renderers should target a small number of shared normalized representations.

## Design Goals

- Terminal-first reliability. Work well in ordinary terminals before reaching for exotic transport features.
- Auto-configuration by default. Detect terminal constraints and input characteristics before asking the user for flags.
- Honest degradation. If fidelity is lost, the tool should make that obvious and choose the most useful fallback.
- Verification over ornament. The render should help a human or agent answer "what is this?" and "did this change?" quickly.
- Library and CLI together. The same rendering core should power an eventual CLI and downstream Rust integrations.

## Target UX

These commands are examples of the intended interface, not implemented behavior yet:

```bash
atxt inspect clip.mp4
atxt render hero.png
atxt render sample.gif --fps 6
atxt render speech.wav --mode waveform
atxt render dashboard.mov --mode contact-sheet --width 120
```

The target behavior is:

- sensible defaults with little or no configuration
- capability-aware output choices
- stable text output that works over remote shells
- explicit switches when users need deterministic overrides

## Local Setup

### With Nix

```bash
nix develop
```

## Verification Modes

- If you only run one command to validate the current verification surface, run `just mission`.
- `just mission` builds the repo binary, renders the canonical still-image fixture once through an interactive Braille path and once through a degraded ASCII fallback path, then prints current mission status.
- `just signal` prints the product-facing signal only: fixture identity plus the direct and degraded render outputs that a human should inspect.
- `just mission-status` prints the current or latest Keel mission state so the operator can see whether the current slice is still active, achieved, or ready for human verification.

### Common Commands

```bash
just
just mission
just signal
just mission-status
just fmt
just cargo-check
just clippy
just test
just check
just flake-check
```

## Early Roadmap

1. Establish the canonical intermediate representation for visual and audio media.
2. Implement terminal capability detection and degradation rules.
3. Add the first static-image renderer path.
4. Add timed-sequence rendering for GIF and video sampling.
5. Add audio waveform and spectrogram output.
6. Build regression fixtures that make terminal output reviewable in CI.

## Research Posture

`atext` may become a useful compression layer for machine perception experiments, but the repository should not over-claim that. The near-term bar is much simpler: can a person or agent inspect media meaningfully through text in hostile terminal environments?
