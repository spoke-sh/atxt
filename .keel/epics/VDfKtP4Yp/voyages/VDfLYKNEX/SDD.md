# Still Image Decode and Txtplot Rendering - Software Design Description

> Define the canonical still-image frame model, add a static-image decode path, and deliver a txtplot-backed braille plus ASCII fallback renderer with a thin CLI and deterministic proofs.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage delivers the first end-to-end visual slice for atxt. It adds a canonical still-image frame model, a static-image decode path, a txtplot-backed braille renderer, an internal ASCII fallback, and a thin CLI wrapper that exercises the existing probe, terminal detection, and render-planning flow.

The key boundary is that atxt owns media decoding, normalization, scale/fit policy, fallback policy, and CLI behavior. txtplot is used only as a braille raster target once atxt has already converted the image into a renderer-agnostic frame and selected a direct braille render path.

## Context & Boundaries

In scope are still-image files, canonical frame normalization, a txtplot adapter for braille rendering, an internal ASCII fallback, and deterministic proofs. Out of scope are timed media, audio, inline image protocols, and advanced color-fidelity work.

```
┌─────────────────────────────────────────────────────────────────────┐
│                              This Voyage                           │
│                                                                     │
│  probe_path -> terminal detect -> plan_render -> decode -> render  │
│                                                │          │         │
│                                                │          ├─ braille│
│                                                │          │  via    │
│                                                │          │ txtplot │
│                                                │          └─ ASCII  │
│                                                │             fallback│
└─────────────────────────────────────────────────────────────────────┘
                ↑                                         ↑
        static image files                          terminal stdout
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `image` crate | Rust library | Decode common static raster formats and perform resize/thumbnail operations for the first still-image slice. | Rust crate API; version chosen during implementation |
| `txtplot` crate | Rust library | Provide the braille canvas and ANSI/string rendering path for direct still-image output. | Public `BrailleCanvas` API |
| Existing `media`, `terminal`, and `render` modules | Internal modules | Provide probe, capability detection, and renderer planning surfaces already established by the previous mission. | Current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Canonical image model | Introduce one shared still-image frame surface owned by atxt. | Keeps raster data reusable for future timed media and prevents txtplot types from leaking into the media contract. |
| Braille backend | Use `txtplot::BrailleCanvas` as a narrow adapter target, not as the canonical frame model. | txtplot already provides efficient braille raster output, but it does not own image decode, fit, or fallback policy. |
| Coordinate mode | Use `set_pixel_screen` semantics for raster mapping. | Still-image decoding naturally produces top-left screen-ordered rows. |
| Fallback renderer | Keep ASCII fallback inside atxt. | ASCII degradation is part of atxt's terminal-truthfulness policy, not txtplot's concern. |
| First rendering posture | Optimize for truthful luma/shape recognition first; do not block the slice on advanced color fidelity. | Keeps the first vertical slice achievable while still proving the product path end-to-end. |
| CLI posture | Add a thin `render` command over the shared library path. | Proves the product slice without committing to a large CLI surface prematurely. |

## Architecture

The voyage adds one new normalization layer between probe/planning and rendering:

1. `media` continues to classify input paths into `ProbeResult`.
2. `terminal` continues to derive `TerminalProfile`.
3. `render` continues to produce `RenderPlan`.
4. A new still-image decode layer turns supported image files into a shared frame object.
5. Renderer adapters consume the shared frame object:
   - txtplot-backed braille renderer for direct Unicode-capable paths
   - internal ASCII renderer for degraded paths
6. A thin CLI command invokes the shared library path and writes the final text to stdout.

## Components

| Component | Purpose | Interface | Behavior |
|-----------|---------|-----------|----------|
| Still-image frame model | Canonical normalized raster contract for still images. | Rust struct(s) exposed from the crate root when appropriate. | Holds dimensions plus renderer-agnostic raster data needed by braille and ASCII renderers. |
| Static-image decoder | Decode supported image files into the shared frame model. | Path-based decode function returning a frame or decode error. | Uses `image` crate facilities and preserves enough geometry to support fit/scaling decisions. |
| Txtplot adapter | Convert a shared frame into braille output through `BrailleCanvas`. | Pure rendering helper that returns a `String` or writes into a buffer. | Maps resized/thresholded pixels into `set_pixel_screen` calls and emits txtplot render output. |
| ASCII renderer | Provide a low-capability fallback for still images. | Pure rendering helper that returns a `String`. | Converts normalized image intensity into deterministic ASCII rows sized to the plan/terminal width. |
| CLI command | End-to-end user-facing entrypoint. | `atxt render <path>` and any tightly coupled still-image-only options. | Runs probe, detect, plan, decode, render, and stdout emission with explicit failures for unsupported/decode-error cases. |

## Interfaces

The core library path for this voyage should remain simple:

- `probe_path(&Path) -> ProbeResult`
- `TerminalProfile::detect()` or `detect_terminal_profile(&TerminalEnvironment)`
- `plan_render(&ProbeResult, &TerminalProfile) -> RenderPlan`
- `decode_still_image(&Path, &RenderPlan) -> Result<VisualFrame, DecodeError>`
- `render_still_image(&VisualFrame, &RenderPlan, &TerminalProfile) -> Result<String, RenderError>`

The CLI should be a thin wrapper over these interfaces, not a separate implementation path.

## Data Flow

1. The CLI or caller receives a filesystem path.
2. `probe_path` classifies the asset as a still image and collects coarse metadata.
3. Terminal capability detection derives a `TerminalProfile`.
4. `plan_render` selects a direct braille or degraded ASCII still-image plan.
5. The static-image decoder opens the asset, scales/fits it to the intended output density, and normalizes it into a shared frame.
6. The selected renderer turns the shared frame into terminal text:
   - braille path: map frame pixels into `txtplot::BrailleCanvas`, then call `render_no_color` or `render_to`
   - ASCII path: map normalized luminance into a deterministic character ramp
7. The CLI writes the rendered string to stdout.

When terminal size is unknown, the renderer should use a conservative default width rather than failing or requiring manual flags.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unsupported still-image format | Decoder cannot open the file as a supported static image family. | Return a typed decode/render error to the caller or CLI. | Expand supported formats in a later slice; do not silently guess. |
| Corrupt or unreadable image file | `image` crate decode error or filesystem read failure. | Surface a clear error with path context. | User supplies a valid file or upstream workflow fixes artifact generation. |
| Missing terminal size | `TerminalProfile.size` is `None`. | Use a conservative default render width/height policy. | Rendering still succeeds without manual configuration. |
| Low-capability or non-Unicode session | `RenderPlan` degrades away from braille. | Route to the ASCII fallback rather than attempting braille anyway. | Output remains reviewable, though degraded. |
| txtplot adapter mismatch | Frame-to-braille mapping exposes a backend limitation. | Fail the render path explicitly and capture it in tests, or fall back if the plan allows. | Rework the adapter in atxt or request a narrow upstream txtplot addition if truly needed. |
