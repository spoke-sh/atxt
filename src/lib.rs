//! Foundational crate types for `atext`.
//!
//! The initial crate surface is intentionally small. It defines the
//! project's core concepts before format decoders and renderers land.

pub mod cli;
pub mod frame;
pub mod media;
pub mod render;
pub mod still_image;
pub mod terminal;

pub use cli::{CliError, run_cli};
pub use frame::{Rgba8, StillImageDecodeError, VisualFrame, VisualFrameError, decode_still_image};
pub use media::{
    AudioMetadata, MediaKind, MediaTiming, PixelDimensions, ProbeCompleteness, ProbeResult,
    probe_path,
};
pub use render::{OutputKind, PlanningReason, RenderIntent, RenderMode, RenderPlan, plan_render};
pub use still_image::{StillImageRenderError, render_still_image};
pub use terminal::{
    ColorSupport, Multiplexer, SessionMode, TerminalEnvironment, TerminalProfile, TerminalSize,
    detect_terminal_profile,
};
