//! Foundational crate types for `atext`.
//!
//! The initial crate surface is intentionally small. It defines the
//! project's core concepts before format decoders and renderers land.

pub mod media;
pub mod render;
pub mod terminal;

pub use media::{
    AudioMetadata, MediaKind, MediaTiming, PixelDimensions, ProbeCompleteness, ProbeResult,
    probe_path,
};
pub use render::{OutputKind, PlanningReason, RenderIntent, RenderMode, RenderPlan, plan_render};
pub use terminal::{
    ColorSupport, Multiplexer, SessionMode, TerminalEnvironment, TerminalProfile, TerminalSize,
    detect_terminal_profile,
};
