# Library Rendering API and Downstream Proof - Software Design Description

> Deliver a high-level rendering entry point and a proof program demonstrating downstream library usage without CLI features or ffprobe.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds a single convenience function to the crate root that wraps the existing four-step pipeline (probe → plan → decode → render) into one call. It also adds a Cargo example program that exercises this function as a downstream consumer would, proving the library surface works without CLI modules or external tools.

## Context & Boundaries

```
┌────────────────────────────────────────────────────────────┐
│                      This Voyage                           │
│                                                            │
│  render_to_text(path, terminal_profile) -> Result<String>  │
│       │                                                    │
│       ├── probe_path                                       │
│       ├── plan_render                                      │
│       ├── decode (still_image | sequence | audio)          │
│       └── render (still_image | sequence | audio)          │
│                                                            │
│  examples/render_demo.rs  (proof program)                  │
└────────────────────────────────────────────────────────────┘
        ↑                                       ↑
   filesystem path                        terminal stdout
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Existing probe/plan/decode/render modules | Internal | All pipeline steps already exist | Current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Return type | `Result<String, RenderError>` with a new unified `RenderError` enum | Consumers need a single error type that covers probe, decode, and render failures. |
| Video handling | Return an error when `video` feature is disabled and input is video | Fail explicitly rather than silently degrading to metadata-only output. |
| Module placement | New function in a `render_api` module or directly in `lib.rs` | Keep it close to the crate root since it's the primary library entry point. |
| Proof program | Cargo example (`examples/render_demo.rs`) | Examples compile against the library naturally and are the idiomatic Rust proof pattern. |
| Terminal profile in proof | Construct a synthetic `TerminalProfile` rather than calling `detect_terminal_profile` | The proof must work in CI without a live terminal. |

## Architecture

The convenience function is a thin orchestrator:

```rust
pub fn render_to_text(path: &Path, profile: &TerminalProfile) -> Result<String, RenderError> {
    let probe = probe_path(path)?;
    let plan = plan_render(&probe, profile);
    match plan.output {
        OutputKind::SingleFrame => {
            let frame = decode_still_image(path, &plan)?;
            Ok(render_still_image(&frame, &plan, profile)?)
        }
        OutputKind::FrameSequence => {
            let seq = decode_timed_sequence(path)?;
            let summary = summarize_timed_sequence(&seq, &plan)?;
            // render contact sheet or summary
        }
        OutputKind::AudioVisualization => {
            let audio = decode_audio_summary(path, &plan)?;
            Ok(render_audio_summary(&audio, &plan, profile)?)
        }
        OutputKind::MetadataSummary => {
            // render metadata fallback
        }
    }
}
```

No new pipeline steps are introduced. The function is purely a dispatch wrapper.

## Components

| Component | Purpose | Interface | Behavior |
|-----------|---------|-----------|----------|
| `render_to_text` | Single-call rendering entry point | `fn render_to_text(path: &Path, profile: &TerminalProfile) -> Result<String, RenderError>` | Probes, plans, decodes, renders; returns text or error |
| `RenderError` | Unified error type for the high-level API | Enum wrapping probe, decode, and render errors | Preserves source error context for debugging |
| `examples/render_demo.rs` | Proof program | Standalone binary | Renders bundled fixtures to stdout using a synthetic terminal profile |

## Interfaces

The primary new interface:

```rust
/// Render a media file to terminal text in one call.
///
/// Probes the file, selects a renderer based on the terminal profile,
/// decodes the media, and returns the rendered text output.
pub fn render_to_text(path: &Path, profile: &TerminalProfile) -> Result<String, RenderError>;
```

## Data Flow

1. Caller provides a filesystem path and `TerminalProfile`.
2. `probe_path` classifies the file and extracts metadata.
3. `plan_render` selects renderer and output shape.
4. The appropriate decoder produces a normalized media model.
5. The appropriate renderer produces terminal text.
6. The text is returned to the caller.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unsupported media type | `probe_path` returns unknown kind | Return `RenderError::UnsupportedMedia` | Consumer handles gracefully or enables `video` feature |
| Decode failure | Decoder returns typed error | Wrap in `RenderError` and return | Consumer logs or retries with different input |
| Render failure | Renderer returns typed error | Wrap in `RenderError` and return | Consumer adjusts terminal profile or handles fallback |
| Video input without `video` feature | Probe returns `Unknown` for video files | Return clear error | Consumer enables `video` feature |
