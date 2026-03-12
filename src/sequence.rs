use std::error::Error;
use std::fmt;

use crate::frame::VisualFrame;
use crate::media::{MediaKind, MediaTiming, PixelDimensions};

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

#[cfg(test)]
mod tests {
    use crate::frame::{Rgba8, VisualFrame};

    use super::{TimedFrameSample, TimedVisualSequence, TimedVisualSequenceError};
    use crate::media::{MediaKind, MediaTiming, PixelDimensions};

    fn sample_frame(color: u8) -> VisualFrame {
        VisualFrame::new(
            PixelDimensions::new(1, 1),
            vec![Rgba8::new(color, color, color, 255)],
        )
        .expect("sample frame should be valid")
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
}
