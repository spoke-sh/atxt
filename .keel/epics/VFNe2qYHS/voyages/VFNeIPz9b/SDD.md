# Feature-Gate CLI and External Dependencies - Software Design Description

> Separate CLI-only modules and ffprobe/ffmpeg dependency behind Cargo feature gates so the default library surface is clean for downstream consumers.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces Cargo feature gates that partition the crate into a minimal library surface (probe, decode, normalize, plan, render for images and audio) and opt-in extensions for CLI tooling and video support. The existing binary target enables all features by default so current behavior is unchanged.

## Context & Boundaries

```
┌──────────────────────────────────────────────────────────────┐
│                     atext crate                              │
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Core library (always compiled)                         │ │
│  │  media · terminal · render · frame · sequence           │ │
│  │  still_image · audio · audio_render                     │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                              │
│  ┌─────────────────────┐  ┌──────────────────────────────┐  │
│  │  feature = "cli"    │  │  feature = "video"           │  │
│  │  cli · globe · stats│  │  video module                │  │
│  │  + crossterm dep    │  │  + ffprobe path in media.rs  │  │
│  └─────────────────────┘  │  + ffmpeg in video.rs        │  │
│                            └──────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `crossterm` | Rust crate (optional) | Terminal input for interactive globe; gated behind `cli` feature | 0.29.0 |
| `colored` | Rust crate | Color output used by `globe`; may need gating behind `cli` | 2 |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Feature granularity | Two features: `cli` and `video`, not one monolithic `full` feature. | CLI tooling and video support are orthogonal concerns; a consumer may want video without CLI or vice versa. |
| Default features | `default = ["cli", "video"]` in Cargo.toml. | Preserves existing behavior for the binary target and anyone depending on the crate without overrides. |
| Gate level | `#[cfg(feature)]` on `mod` declarations in `lib.rs`. | Keeps gates at module boundaries per NFR-01; avoids scattered cfg within functions. |
| `media.rs` video probing | Gate the `ffprobe` code path within `media.rs` behind `cfg(feature = "video")`, returning `MediaKind::Unknown` or a clear error for video inputs when disabled. | `media.rs` is a core module that must compile without the video feature; only the ffprobe call path is gated. |

## Architecture

Changes are limited to `Cargo.toml` and `src/lib.rs`, with a small cfg block in `src/media.rs`:

1. **Cargo.toml**: Add `[features]` section with `cli` and `video` features. Make `crossterm` and `colored` optional dependencies activated by `cli`. Add `default = ["cli", "video"]`.
2. **src/lib.rs**: Wrap `pub mod cli`, `pub mod globe`, `pub mod stats` and their re-exports in `#[cfg(feature = "cli")]`. Wrap `pub mod video` and its re-exports in `#[cfg(feature = "video")]`.
3. **src/media.rs**: Wrap the `ffprobe` invocation in `probe_path` behind `#[cfg(feature = "video")]`, with a fallback that classifies video-like files as `MediaKind::Unknown` or returns a probe error when the feature is off.

## Components

| Component | Purpose | Interface | Behavior |
|-----------|---------|-----------|----------|
| Cargo.toml features | Define feature gates | `[features]` section | `cli` enables cli/globe/stats + crossterm; `video` enables video + ffprobe path |
| lib.rs conditional exports | Gate module visibility | `#[cfg(feature = "...")]` on mod/use | Modules and re-exports only compiled when feature active |
| media.rs video probe gate | Gate ffprobe process | `#[cfg(feature = "video")]` within `probe_path` | Without video feature, video-like inputs get `MediaKind::Unknown` |

## Interfaces

No new public API surfaces. Existing interfaces are preserved when features are active, and absent when features are disabled.

## Data Flow

Unchanged from existing pipeline. The only difference is that certain modules and code paths are not compiled when their feature is disabled.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Video file probed without `video` feature | `probe_path` encounters a video-like extension with feature disabled | Return `MediaKind::Unknown` or error indicating video support not compiled | Consumer enables `video` feature or handles unknown media kind |
| CLI module used without `cli` feature | Compile-time error — module does not exist | Compilation fails with clear missing module error | Consumer enables `cli` feature |
