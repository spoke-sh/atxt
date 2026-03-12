/// Coarse media categories used during probing and normalization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MediaKind {
    Image,
    AnimatedImage,
    Video,
    Audio,
    Document,
    #[default]
    Unknown,
}

/// Whether probing produced a complete view of the asset or only partial facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ProbeCompleteness {
    Complete,
    Partial,
    #[default]
    Unknown,
}

/// Pixel dimensions for visually rendered media.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelDimensions {
    pub width_px: u32,
    pub height_px: u32,
}

impl PixelDimensions {
    pub const fn new(width_px: u32, height_px: u32) -> Self {
        Self {
            width_px,
            height_px,
        }
    }
}

/// Timing metadata relevant to timed visual media and long-form assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MediaTiming {
    pub frame_count: Option<u64>,
    pub duration_ms: Option<u64>,
}

/// Audio metadata relevant to waveform and spectrogram planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AudioMetadata {
    pub sample_rate_hz: Option<u32>,
    pub channels: Option<u16>,
}

/// Probe-level metadata collected before full decoding.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProbeResult {
    pub kind: MediaKind,
    pub mime: Option<String>,
    pub completeness: ProbeCompleteness,
    pub dimensions: Option<PixelDimensions>,
    pub timing: Option<MediaTiming>,
    pub audio: Option<AudioMetadata>,
}

impl ProbeResult {
    pub const fn new(kind: MediaKind) -> Self {
        Self {
            kind,
            mime: None,
            completeness: ProbeCompleteness::Unknown,
            dimensions: None,
            timing: None,
            audio: None,
        }
    }

    pub fn with_mime(mut self, mime: impl Into<String>) -> Self {
        self.mime = Some(mime.into());
        self
    }

    pub const fn with_completeness(mut self, completeness: ProbeCompleteness) -> Self {
        self.completeness = completeness;
        self
    }

    pub const fn with_dimensions(mut self, dimensions: PixelDimensions) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub const fn with_timing(mut self, timing: MediaTiming) -> Self {
        self.timing = Some(timing);
        self
    }

    pub const fn with_audio(mut self, audio: AudioMetadata) -> Self {
        self.audio = Some(audio);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AudioMetadata, MediaKind, MediaTiming, PixelDimensions, ProbeCompleteness, ProbeResult,
    };

    #[test]
    fn probe_result_defaults_to_unknown() {
        let result = ProbeResult::default();
        assert_eq!(result.kind, MediaKind::Unknown);
        assert_eq!(result.completeness, ProbeCompleteness::Unknown);
    }

    #[test]
    fn probe_result_can_capture_partial_media_metadata() {
        let result = ProbeResult {
            kind: MediaKind::Video,
            mime: Some("video/mp4".to_string()),
            completeness: ProbeCompleteness::Partial,
            dimensions: Some(PixelDimensions::new(1920, 1080)),
            timing: Some(MediaTiming {
                frame_count: Some(240),
                duration_ms: Some(4_000),
            }),
            audio: Some(AudioMetadata {
                sample_rate_hz: Some(48_000),
                channels: Some(2),
            }),
        };

        assert_eq!(result.completeness, ProbeCompleteness::Partial);
        assert_eq!(result.dimensions, Some(PixelDimensions::new(1920, 1080)));
        assert_eq!(
            result.timing,
            Some(MediaTiming {
                frame_count: Some(240),
                duration_ms: Some(4_000),
            })
        );
        assert_eq!(
            result.audio,
            Some(AudioMetadata {
                sample_rate_hz: Some(48_000),
                channels: Some(2),
            })
        );
    }
}
