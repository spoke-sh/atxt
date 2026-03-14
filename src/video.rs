use std::error::Error;
use std::fmt;
use std::path::Path;
use std::process::Command;

use crate::audio::{AudioSummary, AudioDecodeError, decode_audio_summary};
use crate::frame::{VisualFrame, VisualFrameError, Rgba8};
use crate::media::{ProbeResult, PixelDimensions};

/// Decode failures for multimodal video.
#[derive(Debug)]
pub enum VideoDecodeError {
    Audio(AudioDecodeError),
    Visual(VisualFrameError),
    FFmpeg(String),
    UnsupportedMediaKind,
}

impl fmt::Display for VideoDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Audio(e) => write!(f, "audio decode error: {e}"),
            Self::Visual(e) => write!(f, "visual decode error: {e}"),
            Self::FFmpeg(e) => write!(f, "ffmpeg error: {e}"),
            Self::UnsupportedMediaKind => write!(f, "unsupported media kind for video decode"),
        }
    }
}

impl Error for VideoDecodeError {}

use crate::audio_render::render_audio_summary;
use crate::render::{RenderPlan, RenderMode};
use crate::sequence::summarize_timed_sequence;
use crate::still_image::render_still_image;

/// Unified multimodal video summary.
pub struct VideoSummary {
    pub frames: Vec<VisualFrame>,
    pub audio: AudioSummary,
}

/// Render failures for video output.
#[derive(Debug)]
pub enum VideoRenderError {
    Visual(crate::still_image::StillImageRenderError),
    Audio(crate::audio_render::AudioRenderError),
    Summary(crate::sequence::TimedSequenceSummaryError),
}

impl fmt::Display for VideoRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visual(e) => write!(f, "visual render error: {e}"),
            Self::Audio(e) => write!(f, "audio render error: {e}"),
            Self::Summary(e) => write!(f, "summary error: {e}"),
        }
    }
}

impl Error for VideoRenderError {}

/// Render a multimodal video summary into terminal-safe text.
pub fn render_video_summary(
    summary: &VideoSummary,
    plan: &RenderPlan,
) -> Result<String, VideoRenderError> {
    let mut output = String::new();

    // 1. Render Visual Contact Sheet
    let samples: Vec<crate::sequence::TimedFrameSample> = summary.frames.iter().map(|f| {
        crate::sequence::TimedFrameSample {
            timestamp_ms: 0, // Placeholder
            frame: f.clone(),
        }
    }).collect();

    let sequence = crate::sequence::TimedVisualSequence::new(
        crate::media::MediaKind::Video,
        crate::media::MediaTiming::default(),
        samples,
    ).map_err(|_| VideoRenderError::Summary(crate::sequence::TimedSequenceSummaryError::LayoutOverflow))?;
    
    let contact_sheet = summarize_timed_sequence(&sequence).map_err(VideoRenderError::Summary)?;
    let visual_text = render_still_image(&contact_sheet, plan).map_err(VideoRenderError::Visual)?;
    
    output.push_str("\x1b[1mVisual Summary (Contact Sheet)\x1b[0m\n");
    output.push_str(&visual_text);
    output.push('\n');

    // 2. Render Audio Waveform
    let mut audio_plan = plan.clone();
    audio_plan.output = crate::render::OutputKind::AudioVisualization;
    // Ensure we have a valid audio mode if the plan was visual-oriented
    if audio_plan.intent.mode != RenderMode::Waveform && audio_plan.intent.mode != RenderMode::Spectrogram {
         audio_plan.intent.mode = RenderMode::Waveform;
    }

    let audio_text = render_audio_summary(&summary.audio, &audio_plan).map_err(VideoRenderError::Audio)?;
    
    output.push_str("\x1b[1mAudio Summary (Waveform)\x1b[0m\n");
    output.push_str(&audio_text);

    Ok(output)
}

/// Decode a synchronized multimodal summary from a video file.
pub fn decode_video_summary(
    path: &Path,
    probe: &ProbeResult,
) -> Result<VideoSummary, VideoDecodeError> {
    if !probe.kind.is_timed_visual() {
        return Err(VideoDecodeError::UnsupportedMediaKind);
    }

    // 1. Extract Visual Frames via FFmpeg
    let frames = extract_video_frames(path, probe)?;

    // 2. Extract Audio via existing Symphonia logic
    let audio = decode_audio_summary(path, probe).map_err(VideoDecodeError::Audio)?;

    Ok(VideoSummary { frames, audio })
}

fn extract_video_frames(
    path: &Path,
    probe: &ProbeResult,
) -> Result<Vec<VisualFrame>, VideoDecodeError> {
    let duration_ms = probe.timing.and_then(|t| t.duration_ms).unwrap_or(1000);
    let num_frames = 4;
    let mut visual_frames = Vec::with_capacity(num_frames);

    for i in 0..num_frames {
        let timestamp_ms = (i as u64 * duration_ms) / num_frames as u64;
        let timestamp_secs = timestamp_ms as f64 / 1000.0;
        
        let output = Command::new("ffmpeg")
            .args([
                "-ss", &timestamp_secs.to_string(),
                "-i", path.to_str().unwrap_or_default(),
                "-frames:v", "1",
                "-f", "image2pipe",
                "-vcodec", "png",
                "-",
            ])
            .output()
            .map_err(|e| VideoDecodeError::FFmpeg(e.to_string()))?;

        if !output.status.success() {
            return Err(VideoDecodeError::FFmpeg(String::from_utf8_lossy(&output.stderr).to_string()));
        }

        let img = image::load_from_memory(&output.stdout)
            .map_err(|e| VideoDecodeError::FFmpeg(format!("image decode error: {e}")))?
            .to_rgba8();
        
        let dimensions = PixelDimensions::new(img.width(), img.height());
        let pixels = img.pixels().map(|p| Rgba8::new(p.0[0], p.0[1], p.0[2], p.0[3])).collect();
        
        let frame = VisualFrame::new(dimensions, pixels).map_err(VideoDecodeError::Visual)?;
        visual_frames.push(frame);
    }

    Ok(visual_frames)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::probe_path;

    #[test]
    fn decode_video_summary_extracts_frames_and_audio() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut path = root.to_path_buf();
        path.push("src/testdata/multimodal_test.mp4");

        if !path.exists() {
            return;
        }

        let probe = probe_path(&path);
        let summary = decode_video_summary(&path, &probe).expect("should decode video summary");

        assert_eq!(summary.frames.len(), 4);
        assert!(summary.audio.waveform().bin_count() > 0);
    }
}
