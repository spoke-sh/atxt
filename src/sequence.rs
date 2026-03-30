use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use image::AnimationDecoder;

use crate::frame::{Rgba8, VisualFrame, VisualFrameError};
use crate::media::{MediaKind, MediaTiming, PixelDimensions, ProbeResult};

const DEFAULT_SEQUENCE_SAMPLE_BUDGET: usize = 4;

/// One representative frame sampled from a timed visual sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimedFrameSample {
    pub timestamp_ms: u64,
    pub frame: VisualFrame,
}

impl TimedFrameSample {
    pub fn new(timestamp_ms: u64, frame: VisualFrame) -> Self {
        Self {
            timestamp_ms,
            frame,
        }
    }
}

/// Validation failures for the canonical timed-sequence surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimedVisualSequenceError {
    UnsupportedMediaKind {
        kind: MediaKind,
    },
    EmptySamples,
    SampleCountExceedsFrameCount {
        frame_count: u64,
        sample_count: usize,
    },
    SampleAfterDuration {
        duration_ms: u64,
        timestamp_ms: u64,
        sample_index: usize,
    },
    FrameDimensionsMismatch {
        expected: PixelDimensions,
        actual: PixelDimensions,
        sample_index: usize,
    },
    NonMonotonicTimestamps {
        previous_timestamp_ms: u64,
        timestamp_ms: u64,
        sample_index: usize,
    },
}

impl fmt::Display for TimedVisualSequenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMediaKind { kind } => {
                write!(
                    f,
                    "timed visual sequences only support animated image or video kinds, got {:?}",
                    kind
                )
            }
            Self::EmptySamples => write!(f, "timed visual sequences require at least one sample"),
            Self::SampleCountExceedsFrameCount {
                frame_count,
                sample_count,
            } => write!(
                f,
                "sample count {} exceeds declared frame count {}",
                sample_count, frame_count
            ),
            Self::SampleAfterDuration {
                duration_ms,
                timestamp_ms,
                sample_index,
            } => write!(
                f,
                "sample {} occurs at {} ms, beyond declared duration {} ms",
                sample_index, timestamp_ms, duration_ms
            ),
            Self::FrameDimensionsMismatch {
                expected,
                actual,
                sample_index,
            } => write!(
                f,
                "sample {} dimensions {}x{} do not match sequence dimensions {}x{}",
                sample_index,
                actual.width_px,
                actual.height_px,
                expected.width_px,
                expected.height_px
            ),
            Self::NonMonotonicTimestamps {
                previous_timestamp_ms,
                timestamp_ms,
                sample_index,
            } => write!(
                f,
                "sample {} timestamp {} ms is earlier than previous sample at {} ms",
                sample_index, timestamp_ms, previous_timestamp_ms
            ),
        }
    }
}

impl Error for TimedVisualSequenceError {}

/// Decode failures for path-based timed-sequence loading.
#[derive(Debug)]
pub enum TimedSequenceDecodeError {
    UnsupportedMediaKind {
        path: PathBuf,
        kind: MediaKind,
    },
    DecodeBackendUnavailable {
        path: PathBuf,
        kind: MediaKind,
    },
    ReadFailed {
        path: PathBuf,
        source: std::io::Error,
    },
    DecodeFailed {
        path: PathBuf,
        source: image::ImageError,
    },
    InvalidFrame(VisualFrameError),
    InvalidSequence(TimedVisualSequenceError),
}

impl fmt::Display for TimedSequenceDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMediaKind { path, kind } => write!(
                f,
                "path '{}' is not a supported timed visual input (detected kind: {:?})",
                path.display(),
                kind
            ),
            Self::DecodeBackendUnavailable { path, kind } => write!(
                f,
                "timed decode backend is not implemented yet for '{}' (detected kind: {:?})",
                path.display(),
                kind
            ),
            Self::ReadFailed { path, source } => {
                write!(
                    f,
                    "failed to read timed visual input '{}': {}",
                    path.display(),
                    source
                )
            }
            Self::DecodeFailed { path, source } => write!(
                f,
                "failed to decode timed visual input '{}': {}",
                path.display(),
                source
            ),
            Self::InvalidFrame(error) => {
                write!(
                    f,
                    "decoded timed frame produced invalid frame data: {error}"
                )
            }
            Self::InvalidSequence(error) => {
                write!(f, "decoded timed sequence was invalid: {error}")
            }
        }
    }
}

impl Error for TimedSequenceDecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadFailed { source, .. } => Some(source),
            Self::DecodeFailed { source, .. } => Some(source),
            Self::InvalidFrame(source) => Some(source),
            Self::InvalidSequence(source) => Some(source),
            Self::UnsupportedMediaKind { .. } | Self::DecodeBackendUnavailable { .. } => None,
        }
    }
}

/// Summary transform failures for timed visual sequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimedSequenceSummaryError {
    LayoutOverflow,
    InvalidFrame(VisualFrameError),
}

impl fmt::Display for TimedSequenceSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LayoutOverflow => {
                write!(f, "timed summary layout overflowed frame dimensions")
            }
            Self::InvalidFrame(error) => {
                write!(f, "timed summary produced invalid frame data: {error}")
            }
        }
    }
}

impl Error for TimedSequenceSummaryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidFrame(source) => Some(source),
            Self::LayoutOverflow => None,
        }
    }
}

/// Canonical normalized timed visual sequence consumed by later summary transforms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimedVisualSequence {
    kind: MediaKind,
    timing: MediaTiming,
    dimensions: PixelDimensions,
    samples: Vec<TimedFrameSample>,
}

impl TimedVisualSequence {
    pub fn new(
        kind: MediaKind,
        timing: MediaTiming,
        samples: Vec<TimedFrameSample>,
    ) -> Result<Self, TimedVisualSequenceError> {
        if !kind.is_timed_visual() {
            return Err(TimedVisualSequenceError::UnsupportedMediaKind { kind });
        }

        let Some(first_sample) = samples.first() else {
            return Err(TimedVisualSequenceError::EmptySamples);
        };

        if let Some(frame_count) = timing.frame_count
            && samples.len() > usize::try_from(frame_count).unwrap_or(usize::MAX)
        {
            return Err(TimedVisualSequenceError::SampleCountExceedsFrameCount {
                frame_count,
                sample_count: samples.len(),
            });
        }

        let dimensions = first_sample.frame.dimensions();
        let mut previous_timestamp_ms = first_sample.timestamp_ms;

        if let Some(duration_ms) = timing.duration_ms
            && previous_timestamp_ms > duration_ms
        {
            return Err(TimedVisualSequenceError::SampleAfterDuration {
                duration_ms,
                timestamp_ms: previous_timestamp_ms,
                sample_index: 0,
            });
        }

        for (index, sample) in samples.iter().enumerate() {
            if sample.frame.dimensions() != dimensions {
                return Err(TimedVisualSequenceError::FrameDimensionsMismatch {
                    expected: dimensions,
                    actual: sample.frame.dimensions(),
                    sample_index: index,
                });
            }

            if index > 0 && sample.timestamp_ms < previous_timestamp_ms {
                return Err(TimedVisualSequenceError::NonMonotonicTimestamps {
                    previous_timestamp_ms,
                    timestamp_ms: sample.timestamp_ms,
                    sample_index: index,
                });
            }

            if let Some(duration_ms) = timing.duration_ms
                && sample.timestamp_ms > duration_ms
            {
                return Err(TimedVisualSequenceError::SampleAfterDuration {
                    duration_ms,
                    timestamp_ms: sample.timestamp_ms,
                    sample_index: index,
                });
            }

            previous_timestamp_ms = sample.timestamp_ms;
        }

        Ok(Self {
            kind,
            timing,
            dimensions,
            samples,
        })
    }

    pub const fn kind(&self) -> MediaKind {
        self.kind
    }

    pub const fn timing(&self) -> MediaTiming {
        self.timing
    }

    pub const fn dimensions(&self) -> PixelDimensions {
        self.dimensions
    }

    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    pub fn samples(&self) -> &[TimedFrameSample] {
        &self.samples
    }
}

/// Decode the first supported timed visual inputs into the canonical sequence surface.
pub fn decode_timed_sequence(
    path: &Path,
    probe: &ProbeResult,
) -> Result<TimedVisualSequence, TimedSequenceDecodeError> {
    match probe.kind {
        MediaKind::AnimatedImage => decode_gif_sequence(path, probe),
        MediaKind::Video => Err(TimedSequenceDecodeError::DecodeBackendUnavailable {
            path: path.to_path_buf(),
            kind: probe.kind,
        }),
        kind => Err(TimedSequenceDecodeError::UnsupportedMediaKind {
            path: path.to_path_buf(),
            kind,
        }),
    }
}

/// Convert a timed sequence into a still summary frame that can reuse the still-image render path.
pub fn summarize_timed_sequence(
    sequence: &TimedVisualSequence,
) -> Result<VisualFrame, TimedSequenceSummaryError> {
    if sequence.sample_count() == 1 {
        return Ok(sequence.samples()[0].frame.clone());
    }

    let sample_count = sequence.sample_count();
    let columns = contact_sheet_columns(sample_count);
    let sample_count_u32 =
        u32::try_from(sample_count).map_err(|_| TimedSequenceSummaryError::LayoutOverflow)?;
    let rows = sample_count_u32.div_ceil(columns);
    let tile_dimensions = sequence.dimensions();
    let width_px = tile_dimensions
        .width_px
        .checked_mul(columns)
        .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;
    let height_px = tile_dimensions
        .height_px
        .checked_mul(rows)
        .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;
    let summary_dimensions = PixelDimensions::new(width_px, height_px);
    let summary_pixel_count = usize::try_from(width_px)
        .ok()
        .and_then(|width| {
            usize::try_from(height_px)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;
    let mut pixels = vec![Rgba8::default(); summary_pixel_count];
    let summary_width =
        usize::try_from(width_px).map_err(|_| TimedSequenceSummaryError::LayoutOverflow)?;
    let tile_width = usize::try_from(tile_dimensions.width_px)
        .map_err(|_| TimedSequenceSummaryError::LayoutOverflow)?;

    for (index, sample) in sequence.samples().iter().enumerate() {
        let index_u32 =
            u32::try_from(index).map_err(|_| TimedSequenceSummaryError::LayoutOverflow)?;
        let column = index_u32 % columns;
        let row = index_u32 / columns;
        let x_offset = tile_dimensions
            .width_px
            .checked_mul(column)
            .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;
        let y_offset = tile_dimensions
            .height_px
            .checked_mul(row)
            .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;
        let x_offset =
            usize::try_from(x_offset).map_err(|_| TimedSequenceSummaryError::LayoutOverflow)?;
        let y_offset =
            usize::try_from(y_offset).map_err(|_| TimedSequenceSummaryError::LayoutOverflow)?;

        for y in 0..sample.frame.height_px() {
            let y = usize::try_from(y).map_err(|_| TimedSequenceSummaryError::LayoutOverflow)?;
            let dest_start = y_offset
                .checked_add(y)
                .and_then(|row_index| row_index.checked_mul(summary_width))
                .and_then(|row_index| row_index.checked_add(x_offset))
                .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;
            let src_start = y
                .checked_mul(tile_width)
                .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;
            let src_end = src_start
                .checked_add(tile_width)
                .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;
            let dest_end = dest_start
                .checked_add(tile_width)
                .ok_or(TimedSequenceSummaryError::LayoutOverflow)?;

            pixels[dest_start..dest_end]
                .copy_from_slice(&sample.frame.pixels()[src_start..src_end]);
        }
    }

    VisualFrame::new(summary_dimensions, pixels).map_err(TimedSequenceSummaryError::InvalidFrame)
}

fn decode_gif_sequence(
    path: &Path,
    probe: &ProbeResult,
) -> Result<TimedVisualSequence, TimedSequenceDecodeError> {
    let file = File::open(path).map_err(|source| TimedSequenceDecodeError::ReadFailed {
        path: path.to_path_buf(),
        source,
    })?;
    let reader = image::codecs::gif::GifDecoder::new(BufReader::new(file)).map_err(|source| {
        TimedSequenceDecodeError::DecodeFailed {
            path: path.to_path_buf(),
            source,
        }
    })?;
    let frames = reader.into_frames().collect_frames().map_err(|source| {
        TimedSequenceDecodeError::DecodeFailed {
            path: path.to_path_buf(),
            source,
        }
    })?;
    let decoded_frame_count = frames.len();
    let sample_indices = select_sample_indices(frames.len(), DEFAULT_SEQUENCE_SAMPLE_BUDGET);

    if sample_indices.is_empty() {
        return Err(TimedSequenceDecodeError::InvalidSequence(
            TimedVisualSequenceError::EmptySamples,
        ));
    }

    let mut next_selected = 0_usize;
    let mut elapsed_ms = 0_u64;
    let mut samples = Vec::with_capacity(sample_indices.len());

    for (index, frame) in frames.into_iter().enumerate() {
        let delay_ms = image_delay_ms(frame.delay());
        if next_selected < sample_indices.len() && index == sample_indices[next_selected] {
            let visual_frame = rgba_frame_to_visual(frame.into_buffer())
                .map_err(TimedSequenceDecodeError::InvalidFrame)?;
            samples.push(TimedFrameSample::new(elapsed_ms, visual_frame));
            next_selected += 1;
        }
        elapsed_ms = elapsed_ms.saturating_add(delay_ms);
    }

    let timing = merge_sequence_timing(probe.timing, Some(decoded_frame_count), elapsed_ms);

    TimedVisualSequence::new(MediaKind::AnimatedImage, timing, samples)
        .map_err(TimedSequenceDecodeError::InvalidSequence)
}

fn merge_sequence_timing(
    probe_timing: Option<MediaTiming>,
    actual_frame_count: Option<usize>,
    duration_ms: u64,
) -> MediaTiming {
    let actual_frame_count = actual_frame_count.and_then(|count| u64::try_from(count).ok());
    let actual_duration_ms = if duration_ms > 0 {
        Some(duration_ms)
    } else {
        None
    };
    let actual_nominal_frame_rate_milli_fps =
        actual_frame_count
            .zip(actual_duration_ms)
            .and_then(|(count, duration_ms)| {
                count
                    .checked_mul(1_000_000)
                    .and_then(|value| value.checked_div(duration_ms))
                    .and_then(|value| u32::try_from(value).ok())
            });
    let probe_timing = probe_timing.unwrap_or_default();

    MediaTiming {
        frame_count: probe_timing.frame_count.or(actual_frame_count),
        duration_ms: probe_timing.duration_ms.or(actual_duration_ms),
        nominal_frame_rate_milli_fps: probe_timing
            .nominal_frame_rate_milli_fps
            .or(actual_nominal_frame_rate_milli_fps),
    }
}

fn contact_sheet_columns(sample_count: usize) -> u32 {
    let mut columns = 1_u32;
    while usize::try_from(columns.saturating_mul(columns)).unwrap_or(usize::MAX) < sample_count {
        columns = columns.saturating_add(1);
    }
    columns
}

fn select_sample_indices(frame_count: usize, budget: usize) -> Vec<usize> {
    if frame_count == 0 || budget == 0 {
        return Vec::new();
    }

    if frame_count <= budget {
        return (0..frame_count).collect();
    }

    let last_index = frame_count - 1;
    (0..budget)
        .map(|slot| slot * last_index / (budget - 1))
        .collect()
}

fn image_delay_ms(delay: image::Delay) -> u64 {
    let (numer, denom) = delay.numer_denom_ms();
    let denom = denom.max(1);

    u64::from(numer).div_ceil(u64::from(denom))
}

fn rgba_frame_to_visual(buffer: image::RgbaImage) -> Result<VisualFrame, VisualFrameError> {
    let dimensions = PixelDimensions::new(buffer.width(), buffer.height());
    let pixels = buffer
        .pixels()
        .map(|pixel| {
            let [r, g, b, a] = pixel.0;
            Rgba8::new(r, g, b, a)
        })
        .collect();

    VisualFrame::new(dimensions, pixels)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::process;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use gif::{Encoder, Frame, Repeat};

    use crate::frame::{Rgba8, VisualFrame};
    use crate::media::{ProbeCompleteness, ProbeResult, probe_path};
    use crate::render::plan_render;
    use crate::still_image::render_still_image;
    use crate::terminal::TerminalEnvironment;

    use super::{
        TimedFrameSample, TimedSequenceDecodeError, TimedVisualSequence, TimedVisualSequenceError,
        decode_timed_sequence, summarize_timed_sequence,
    };
    use crate::media::{MediaKind, MediaTiming, PixelDimensions};

    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn sample_frame(color: u8) -> VisualFrame {
        VisualFrame::new(
            PixelDimensions::new(1, 1),
            vec![Rgba8::new(color, color, color, 255)],
        )
        .expect("sample frame should be valid")
    }

    struct TempGifPath {
        path: PathBuf,
    }

    impl TempGifPath {
        fn new() -> Self {
            let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = env::temp_dir();
            path.push(format!("atxt-sequence-test-{}-{}.gif", process::id(), id));
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempGifPath {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    fn write_grayscale_gif(path: &Path, colors: &[u8]) {
        let file = File::create(path).expect("gif fixture should be created");
        let mut encoder = Encoder::new(file, 1, 1, &[]).expect("gif encoder should open");
        encoder
            .set_repeat(Repeat::Infinite)
            .expect("gif repeat should be set");

        for color in colors {
            let pixels = vec![*color, *color, *color];
            let mut frame = Frame::from_rgb_speed(1, 1, &pixels, 10);
            frame.delay = 2;
            encoder
                .write_frame(&frame)
                .expect("gif frame should be written");
        }
    }

    #[test]
    fn timed_visual_sequence_validates_samples_and_exposes_metadata() {
        let sequence = TimedVisualSequence::new(
            MediaKind::Video,
            MediaTiming {
                frame_count: Some(4),
                duration_ms: Some(600),
                nominal_frame_rate_milli_fps: Some(6_666),
            },
            vec![
                TimedFrameSample::new(0, sample_frame(0)),
                TimedFrameSample::new(500, sample_frame(255)),
            ],
        )
        .expect("timed sequence should validate");

        assert_eq!(sequence.kind(), MediaKind::Video);
        assert_eq!(sequence.dimensions(), PixelDimensions::new(1, 1));
        assert_eq!(sequence.sample_count(), 2);
        assert_eq!(
            sequence.timing(),
            MediaTiming {
                frame_count: Some(4),
                duration_ms: Some(600),
                nominal_frame_rate_milli_fps: Some(6_666),
            }
        );
        assert_eq!(sequence.samples()[1].timestamp_ms, 500);
    }

    #[test]
    fn timed_visual_sequence_rejects_non_timed_kinds() {
        let error = TimedVisualSequence::new(
            MediaKind::Image,
            MediaTiming::default(),
            vec![TimedFrameSample::new(0, sample_frame(0))],
        )
        .unwrap_err();

        assert_eq!(
            error,
            TimedVisualSequenceError::UnsupportedMediaKind {
                kind: MediaKind::Image,
            }
        );
    }

    #[test]
    fn timed_visual_sequence_rejects_invalid_sample_layout() {
        let mismatched_frame = VisualFrame::new(
            PixelDimensions::new(2, 1),
            vec![Rgba8::new(0, 0, 0, 255), Rgba8::new(255, 255, 255, 255)],
        )
        .expect("mismatched frame should be valid");

        let error = TimedVisualSequence::new(
            MediaKind::AnimatedImage,
            MediaTiming {
                frame_count: Some(1),
                duration_ms: Some(100),
                nominal_frame_rate_milli_fps: Some(10_000),
            },
            vec![
                TimedFrameSample::new(0, sample_frame(0)),
                TimedFrameSample::new(50, mismatched_frame),
            ],
        )
        .unwrap_err();

        assert_eq!(
            error,
            TimedVisualSequenceError::SampleCountExceedsFrameCount {
                frame_count: 1,
                sample_count: 2,
            }
        );

        let error = TimedVisualSequence::new(
            MediaKind::AnimatedImage,
            MediaTiming {
                frame_count: Some(2),
                duration_ms: Some(100),
                nominal_frame_rate_milli_fps: Some(20_000),
            },
            vec![
                TimedFrameSample::new(60, sample_frame(0)),
                TimedFrameSample::new(50, sample_frame(255)),
            ],
        )
        .unwrap_err();

        assert_eq!(
            error,
            TimedVisualSequenceError::NonMonotonicTimestamps {
                previous_timestamp_ms: 60,
                timestamp_ms: 50,
                sample_index: 1,
            }
        );
    }

    #[test]
    fn decode_timed_sequence_extracts_bounded_gif_samples() {
        let gif = TempGifPath::new();
        write_grayscale_gif(gif.path(), &[0, 32, 96, 160, 224, 255]);
        let probe = probe_path(gif.path());

        let sequence =
            decode_timed_sequence(gif.path(), &probe).expect("gif sequence should decode");

        assert_eq!(probe.completeness, ProbeCompleteness::Complete);
        assert_eq!(sequence.kind(), MediaKind::AnimatedImage);
        assert_eq!(sequence.sample_count(), 4);
        assert_eq!(sequence.dimensions(), PixelDimensions::new(1, 1));
        assert_eq!(
            sequence.timing(),
            MediaTiming {
                frame_count: Some(6),
                duration_ms: Some(120),
                nominal_frame_rate_milli_fps: Some(50_000),
            }
        );
        assert_eq!(sequence.samples()[0].timestamp_ms, 0);
        assert_eq!(sequence.samples()[3].timestamp_ms, 100);
    }

    #[test]
    fn timed_summary_routes_through_shared_still_image_renderer() {
        let gif = TempGifPath::new();
        write_grayscale_gif(gif.path(), &[0, 255]);
        let probe = probe_path(gif.path());
        let sequence =
            decode_timed_sequence(gif.path(), &probe).expect("gif sequence should decode");
        let summary =
            summarize_timed_sequence(&sequence).expect("contact sheet summary should build");
        let plan = plan_render(
            &probe,
            &crate::terminal::detect_terminal_profile(&TerminalEnvironment {
                term: Some("xterm-256color".to_string()),
                colorterm: Some("truecolor".to_string()),
                no_color: false,
                tmux: false,
                ssh_connection: false,
                stdout_is_tty: true,
                columns: Some(4),
                rows: Some(2),
            }),
        );

        let output = render_still_image(&summary, &plan)
            .expect("shared still-image renderer should accept timed summary frame");

        assert_eq!(plan.output, crate::render::OutputKind::SingleFrame);
        assert_eq!(plan.intent.mode, crate::render::RenderMode::ContactSheet);
        assert!(!output.is_empty());
    }

    #[test]
    fn decode_timed_sequence_rejects_video_until_backend_exists() {
        let probe =
            ProbeResult::new(MediaKind::Video).with_completeness(ProbeCompleteness::Partial);
        let error = decode_timed_sequence(Path::new("clip.mp4"), &probe).unwrap_err();

        match error {
            TimedSequenceDecodeError::DecodeBackendUnavailable { kind, .. } => {
                assert_eq!(kind, MediaKind::Video);
            }
            other => panic!("expected video backend error, got {other:?}"),
        }
    }
}
