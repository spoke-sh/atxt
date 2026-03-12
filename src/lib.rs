//! Foundational crate types for `atext`.
//!
//! The initial crate surface is intentionally small. It defines the
//! project's core concepts before format decoders and renderers land.

pub mod media;
pub mod render;
pub mod terminal;

pub use media::{MediaKind, ProbeResult};
pub use render::{OutputKind, RenderIntent, RenderMode};
pub use terminal::{ColorSupport, Multiplexer, TerminalProfile};
