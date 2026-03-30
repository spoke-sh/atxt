//! Foundational crate types for `atxt`.
//!
//! The initial crate surface is intentionally small. It defines the
//! project's core concepts before format decoders and renderers land.

pub mod audio;
pub mod audio_render;
pub mod frame;
pub mod media;
pub mod render;
pub mod sequence;
pub mod still_image;
pub mod terminal;

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "cli")]
pub mod globe;
#[cfg(feature = "cli")]
pub mod stats;

#[cfg(feature = "video")]
pub mod video;

pub use audio::{
    AudioDecodeError, AudioSummary, AudioSummaryError, SpectrogramSummary, WaveformBin,
    WaveformSummary, decode_audio_summary,
};
pub use audio_render::{AudioRenderError, render_audio_summary};
pub use frame::{Rgba8, StillImageDecodeError, VisualFrame, VisualFrameError, decode_still_image};
pub use media::{
    AudioMetadata, MediaKind, MediaTiming, PixelDimensions, ProbeCompleteness, ProbeResult,
    probe_path,
};
pub use render::{OutputKind, PlanningReason, RenderIntent, RenderMode, RenderPlan, plan_render};
pub use sequence::{
    TimedFrameSample, TimedSequenceDecodeError, TimedSequenceSummaryError, TimedVisualSequence,
    TimedVisualSequenceError, decode_timed_sequence, summarize_timed_sequence,
};
pub use still_image::{StillImageRenderError, render_still_image};
pub use terminal::{
    ColorSupport, Multiplexer, SessionMode, TerminalEnvironment, TerminalProfile, TerminalSize,
    detect_terminal_profile,
};

#[cfg(feature = "cli")]
pub use cli::{CliError, run_cli};
#[cfg(feature = "cli")]
pub use stats::render_stats;
#[cfg(feature = "cli")]
pub use globe::render_drift_globe;

#[cfg(feature = "video")]
pub use video::{VideoDecodeError, VideoRenderError, VideoSummary, decode_video_summary, render_video_summary};

use std::fmt;
use std::path::Path;

/// Unified error type for the high-level [`render_to_text`] entry point.
#[derive(Debug)]
pub enum RenderError {
    StillDecode(StillImageDecodeError),
    TimedDecode(TimedSequenceDecodeError),
    TimedSummary(TimedSequenceSummaryError),
    AudioDecode(AudioDecodeError),
    StillRender(StillImageRenderError),
    AudioRender(AudioRenderError),
    #[cfg(feature = "video")]
    VideoDecode(video::VideoDecodeError),
    #[cfg(feature = "video")]
    VideoRender(video::VideoRenderError),
    /// The probed media kind has no rendering path available.
    UnsupportedMedia(MediaKind),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StillDecode(e) => write!(f, "{e}"),
            Self::TimedDecode(e) => write!(f, "{e}"),
            Self::TimedSummary(e) => write!(f, "{e}"),
            Self::AudioDecode(e) => write!(f, "{e}"),
            Self::StillRender(e) => write!(f, "{e}"),
            Self::AudioRender(e) => write!(f, "{e}"),
            #[cfg(feature = "video")]
            Self::VideoDecode(e) => write!(f, "{e}"),
            #[cfg(feature = "video")]
            Self::VideoRender(e) => write!(f, "{e}"),
            Self::UnsupportedMedia(kind) => write!(f, "unsupported media kind: {kind:?}"),
        }
    }
}

impl std::error::Error for RenderError {}

/// Render a media file to terminal text in one call.
///
/// Probes the file, selects a renderer based on the terminal profile,
/// decodes the media, and returns the rendered text output.
pub fn render_to_text(path: &Path, profile: &TerminalProfile) -> Result<String, RenderError> {
    let probe = probe_path(path);
    let plan = plan_render(&probe, profile);

    #[cfg(feature = "video")]
    if probe.kind == MediaKind::Video {
        let summary = video::decode_video_summary(path, &probe).map_err(RenderError::VideoDecode)?;
        return video::render_video_summary(&summary, &plan).map_err(RenderError::VideoRender);
    }

    match probe.kind {
        MediaKind::Image => {
            let frame = decode_still_image(path).map_err(RenderError::StillDecode)?;
            render_still_image(&frame, &plan).map_err(RenderError::StillRender)
        }
        MediaKind::AnimatedImage => {
            let sequence = decode_timed_sequence(path, &probe).map_err(RenderError::TimedDecode)?;
            let frame = summarize_timed_sequence(&sequence).map_err(RenderError::TimedSummary)?;
            render_still_image(&frame, &plan).map_err(RenderError::StillRender)
        }
        MediaKind::Audio => {
            let summary = decode_audio_summary(path, &probe).map_err(RenderError::AudioDecode)?;
            render_audio_summary(&summary, &plan).map_err(RenderError::AudioRender)
        }
        #[cfg(feature = "video")]
        MediaKind::Video => unreachable!("handled above"),
        _ => Err(RenderError::UnsupportedMedia(probe.kind)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    use std::process;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_png() -> PathBuf {
        let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut path = env::temp_dir();
        path.push(format!("atxt-lib-test-{}-{}.png", process::id(), id));

        let mut img = image::RgbaImage::new(4, 4);
        for y in 0..4 {
            for x in 0..4 {
                let v = if x < 2 { 0 } else { 255 };
                img.put_pixel(x, y, image::Rgba([v, v, v, 255]));
            }
        }
        img.save(&path).expect("test png should save");
        path
    }

    fn captured_profile() -> TerminalProfile {
        detect_terminal_profile(&TerminalEnvironment {
            term: Some("xterm-256color".into()),
            colorterm: None,
            no_color: false,
            tmux: false,
            ssh_connection: false,
            columns: Some(80),
            rows: Some(24),
            stdout_is_tty: false,
        })
    }

    #[test]
    fn render_to_text_renders_still_image() {
        let path = temp_png();
        let profile = captured_profile();
        let result = render_to_text(&path, &profile);
        std::fs::remove_file(&path).ok();
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn render_to_text_returns_error_for_unsupported_media() {
        let result = render_to_text(Path::new("unknown.xyz"), &captured_profile());
        assert!(matches!(result, Err(RenderError::UnsupportedMedia(MediaKind::Unknown))));
    }
}
