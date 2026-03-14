use std::fs::File;
use std::path::Path;

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

impl MediaKind {
    pub const fn is_timed_visual(self) -> bool {
        matches!(self, Self::AnimatedImage | Self::Video)
    }
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
    pub nominal_frame_rate_milli_fps: Option<u32>,
}

/// Audio metadata relevant to waveform and spectrogram planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AudioMetadata {
    pub sample_rate_hz: Option<u32>,
    pub channels: Option<u16>,
    pub sample_count: Option<u64>,
    pub bits_per_sample: Option<u16>,
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

/// Classify a filesystem path into the initial media families used by render planning.
pub fn probe_path(path: &Path) -> ProbeResult {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase());

    let Some(extension) = extension else {
        return ProbeResult::default();
    };

    let classification = match extension.as_str() {
        "png" => Some((MediaKind::Image, "image/png")),
        "jpg" | "jpeg" => Some((MediaKind::Image, "image/jpeg")),
        "webp" => Some((MediaKind::Image, "image/webp")),
        "bmp" => Some((MediaKind::Image, "image/bmp")),
        "gif" => Some((MediaKind::AnimatedImage, "image/gif")),
        "mp4" => Some((MediaKind::Video, "video/mp4")),
        "mov" => Some((MediaKind::Video, "video/quicktime")),
        "mkv" => Some((MediaKind::Video, "video/x-matroska")),
        "webm" => Some((MediaKind::Video, "video/webm")),
        "wav" => Some((MediaKind::Audio, "audio/wav")),
        "mp3" => Some((MediaKind::Audio, "audio/mpeg")),
        "flac" => Some((MediaKind::Audio, "audio/flac")),
        "ogg" => Some((MediaKind::Audio, "audio/ogg")),
        "pdf" => Some((MediaKind::Document, "application/pdf")),
        _ => None,
    };

    match classification {
        Some((MediaKind::AnimatedImage, mime)) => probe_gif_metadata(path, mime),
        Some((MediaKind::Audio, mime)) => probe_audio_metadata(path, mime),
        Some((kind, mime)) => ProbeResult::new(kind)
            .with_mime(mime)
            .with_completeness(ProbeCompleteness::Partial),
        None => ProbeResult::default(),
    }
}

fn probe_gif_metadata(path: &Path, mime: &str) -> ProbeResult {
    let base = ProbeResult::new(MediaKind::AnimatedImage)
        .with_mime(mime)
        .with_completeness(ProbeCompleteness::Partial);

    let Ok(file) = File::open(path) else {
        return base;
    };

    let decoder = gif::DecodeOptions::new();
    let Ok(mut reader) = decoder.read_info(file) else {
        return base;
    };

    let dimensions = PixelDimensions::new(u32::from(reader.width()), u32::from(reader.height()));
    let mut frame_count = 0_u64;
    let mut duration_ms = 0_u64;
    let mut had_error = false;

    loop {
        match reader.read_next_frame() {
            Ok(Some(frame)) => {
                frame_count = frame_count.saturating_add(1);
                duration_ms = duration_ms.saturating_add(u64::from(frame.delay) * 10);
            }
            Ok(None) => break,
            Err(_) => {
                had_error = true;
                break;
            }
        }
    }

    let nominal_frame_rate_milli_fps = if duration_ms > 0 {
        frame_count
            .checked_mul(1_000_000)
            .and_then(|value| value.checked_div(duration_ms))
            .and_then(|value| u32::try_from(value).ok())
    } else {
        None
    };
    let timing = MediaTiming {
        frame_count: Some(frame_count),
        duration_ms: if duration_ms > 0 {
            Some(duration_ms)
        } else {
            None
        },
        nominal_frame_rate_milli_fps,
    };

    let completeness = if had_error || frame_count == 0 {
        ProbeCompleteness::Partial
    } else {
        ProbeCompleteness::Complete
    };

    base.with_completeness(completeness)
        .with_dimensions(dimensions)
        .with_timing(timing)
}

fn probe_audio_metadata(path: &Path, mime: &str) -> ProbeResult {
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;

    let base = ProbeResult::new(MediaKind::Audio)
        .with_mime(mime)
        .with_completeness(ProbeCompleteness::Partial);

    let Ok(file) = File::open(path) else {
        return base;
    };

    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    );

    let Ok(probed) = probed else {
        return base;
    };

    let format = probed.format;
    let track = format.default_track();
    let codec_params = track.map(|t| &t.codec_params);

    let sample_rate = codec_params.and_then(|p| p.sample_rate);
    let channels = codec_params.and_then(|p| p.channels).map(|c| c.count() as u16);
    let sample_count = codec_params.and_then(|p| p.n_frames);
    let bits_per_sample = codec_params.and_then(|p| p.bits_per_sample).map(|b| b as u16);

    let duration_ms = if let (Some(n_frames), Some(sample_rate)) = (sample_count, sample_rate) {
        n_frames
            .checked_mul(1_000)
            .and_then(|v| v.checked_div(u64::from(sample_rate)))
    } else {
        None
    };

    base.with_completeness(ProbeCompleteness::Complete)
        .with_timing(MediaTiming {
            frame_count: None,
            duration_ms,
            nominal_frame_rate_milli_fps: None,
        })
        .with_audio(AudioMetadata {
            sample_rate_hz: sample_rate,
            channels,
            sample_count,
            bits_per_sample,
        })
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::path::Path;
    use std::path::PathBuf;
    use std::process;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{
        AudioMetadata, MediaKind, MediaTiming, PixelDimensions, ProbeCompleteness, ProbeResult,
        probe_path,
    };
    use gif::{Encoder, Frame, Repeat};
    use hound::{SampleFormat, WavSpec, WavWriter};

    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TempGifPath {
        path: PathBuf,
    }

    impl TempGifPath {
        fn new() -> Self {
            let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = env::temp_dir();
            path.push(format!("atext-media-test-{}-{}.gif", process::id(), id));
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

    struct TempWavPath {
        path: PathBuf,
    }

    impl TempWavPath {
        fn new() -> Self {
            let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = env::temp_dir();
            path.push(format!("atext-media-test-{}-{}.wav", process::id(), id));
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempWavPath {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    fn write_test_gif(path: &Path) {
        let file = File::create(path).expect("gif fixture should be created");
        let mut encoder = Encoder::new(file, 2, 1, &[]).expect("gif encoder should open");
        encoder
            .set_repeat(Repeat::Infinite)
            .expect("gif repeat should be set");

        let pixels_a = vec![0, 0, 0, 255, 255, 255];
        let mut frame_a = Frame::from_rgb_speed(2, 1, &pixels_a, 10);
        frame_a.delay = 2;
        encoder
            .write_frame(&frame_a)
            .expect("first gif frame should be written");

        let pixels_b = vec![255, 255, 255, 0, 0, 0];
        let mut frame_b = Frame::from_rgb_speed(2, 1, &pixels_b, 10);
        frame_b.delay = 3;
        encoder
            .write_frame(&frame_b)
            .expect("second gif frame should be written");
    }

    fn write_test_wav(path: &Path) {
        let spec = WavSpec {
            channels: 2,
            sample_rate: 8_000,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let mut writer = WavWriter::create(path, spec).expect("wav fixture should be created");

        for sample in 0_i16..2_000_i16 {
            writer
                .write_sample(sample)
                .expect("left channel sample should write");
            writer
                .write_sample(-sample)
                .expect("right channel sample should write");
        }

        writer.finalize().expect("wav fixture should finalize");
    }

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
                nominal_frame_rate_milli_fps: Some(60_000),
            }),
            audio: Some(AudioMetadata {
                sample_rate_hz: Some(48_000),
                channels: Some(2),
                sample_count: Some(192_000),
                bits_per_sample: Some(16),
            }),
        };

        assert_eq!(result.completeness, ProbeCompleteness::Partial);
        assert_eq!(result.dimensions, Some(PixelDimensions::new(1920, 1080)));
        assert_eq!(
            result.timing,
            Some(MediaTiming {
                frame_count: Some(240),
                duration_ms: Some(4_000),
                nominal_frame_rate_milli_fps: Some(60_000),
            })
        );
        assert_eq!(
            result.audio,
            Some(AudioMetadata {
                sample_rate_hz: Some(48_000),
                channels: Some(2),
                sample_count: Some(192_000),
                bits_per_sample: Some(16),
            })
        );
    }

    #[test]
    fn probe_path_classifies_initial_media_families() {
        assert_eq!(probe_path(Path::new("still.png")).kind, MediaKind::Image);
        let gif = probe_path(Path::new("loop.gif"));
        assert_eq!(gif.kind, MediaKind::AnimatedImage);
        assert_eq!(gif.completeness, ProbeCompleteness::Partial);
        assert_eq!(probe_path(Path::new("clip.mp4")).kind, MediaKind::Video);
        assert_eq!(probe_path(Path::new("tone.wav")).kind, MediaKind::Audio);
        assert_eq!(probe_path(Path::new("spec.pdf")).kind, MediaKind::Document);
        assert_eq!(probe_path(Path::new("blob.bin")).kind, MediaKind::Unknown);

        let video = probe_path(Path::new("clip.mp4"));
        assert_eq!(video.completeness, ProbeCompleteness::Partial);
        assert_eq!(video.mime.as_deref(), Some("video/mp4"));
    }

    #[test]
    fn media_kind_exposes_timed_visual_classification() {
        assert!(MediaKind::AnimatedImage.is_timed_visual());
        assert!(MediaKind::Video.is_timed_visual());
        assert!(!MediaKind::Image.is_timed_visual());
    }

    #[test]
    fn probe_path_collects_gif_timing_when_file_exists() {
        let gif = TempGifPath::new();
        write_test_gif(gif.path());

        let result = probe_path(gif.path());

        assert_eq!(result.kind, MediaKind::AnimatedImage);
        assert_eq!(result.completeness, ProbeCompleteness::Complete);
        assert_eq!(result.dimensions, Some(PixelDimensions::new(2, 1)));
        assert_eq!(
            result.timing,
            Some(MediaTiming {
                frame_count: Some(2),
                duration_ms: Some(50),
                nominal_frame_rate_milli_fps: Some(40_000),
            })
        );
    }

    #[test]
    fn probe_path_collects_wav_audio_metadata_when_file_exists() {
        let wav = TempWavPath::new();
        write_test_wav(wav.path());

        let result = probe_path(wav.path());

        assert_eq!(result.kind, MediaKind::Audio);
        assert_eq!(result.completeness, ProbeCompleteness::Complete);
        assert_eq!(
            result.audio,
            Some(AudioMetadata {
                sample_rate_hz: Some(8_000),
                channels: Some(2),
                sample_count: Some(2_000),
                bits_per_sample: Some(16),
            })
        );
        assert_eq!(
            result.timing,
            Some(MediaTiming {
                frame_count: None,
                duration_ms: Some(250),
                nominal_frame_rate_milli_fps: None,
            })
        );
    }
}
