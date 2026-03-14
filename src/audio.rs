use std::error::Error;
use std::f32::consts::TAU;
use std::fmt;
use std::fs;
use std::path::Path;

use crate::media::{MediaKind, ProbeResult};

const MAX_DECODE_SAMPLES: usize = 16_384;
const WAVEFORM_BIN_BUDGET: usize = 64;
const SPECTROGRAM_TIME_SLICE_BUDGET: u16 = 32;
const SPECTROGRAM_BAND_BUDGET: u16 = 16;
const SPECTROGRAM_WINDOW_SIZE: usize = 64;
const NORMALIZED_LEVEL_LIMIT_MILLI: i16 = 1_000;
const NORMALIZED_INTENSITY_LIMIT_MILLI: u16 = 1_000;

/// One normalized waveform bucket covering a bounded span of audio time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveformBin {
    pub min_level_milli: i16,
    pub max_level_milli: i16,
}

impl WaveformBin {
    pub const fn new(min_level_milli: i16, max_level_milli: i16) -> Self {
        Self {
            min_level_milli,
            max_level_milli,
        }
    }
}

/// Waveform-oriented summary data for an audio input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveformSummary {
    bins: Vec<WaveformBin>,
}

impl WaveformSummary {
    pub fn new(bins: Vec<WaveformBin>) -> Result<Self, AudioSummaryError> {
        if bins.is_empty() {
            return Err(AudioSummaryError::WaveformBinsEmpty);
        }

        for (index, bin) in bins.iter().copied().enumerate() {
            validate_waveform_level(bin.min_level_milli, index, "min")?;
            validate_waveform_level(bin.max_level_milli, index, "max")?;

            if bin.min_level_milli > bin.max_level_milli {
                return Err(AudioSummaryError::WaveformBinRangeInvalid {
                    bin_index: index,
                    min_level_milli: bin.min_level_milli,
                    max_level_milli: bin.max_level_milli,
                });
            }
        }

        Ok(Self { bins })
    }

    pub fn bin_count(&self) -> usize {
        self.bins.len()
    }

    pub fn bins(&self) -> &[WaveformBin] {
        &self.bins
    }
}

/// Spectrogram-oriented summary data laid out as time slices by frequency bands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpectrogramSummary {
    time_slices: u16,
    frequency_bands: u16,
    intensities_milli: Vec<u16>,
}

impl SpectrogramSummary {
    pub fn new(
        time_slices: u16,
        frequency_bands: u16,
        intensities_milli: Vec<u16>,
    ) -> Result<Self, AudioSummaryError> {
        if time_slices == 0 || frequency_bands == 0 {
            return Err(AudioSummaryError::SpectrogramDimensionsZero {
                time_slices,
                frequency_bands,
            });
        }

        let expected = usize::from(time_slices)
            .checked_mul(usize::from(frequency_bands))
            .ok_or(AudioSummaryError::SpectrogramLayoutOverflow)?;
        let actual = intensities_milli.len();
        if actual != expected {
            return Err(AudioSummaryError::SpectrogramBinCountMismatch { expected, actual });
        }

        for (index, intensity_milli) in intensities_milli.iter().copied().enumerate() {
            if intensity_milli > NORMALIZED_INTENSITY_LIMIT_MILLI {
                return Err(AudioSummaryError::SpectrogramIntensityOutOfRange {
                    bin_index: index,
                    intensity_milli,
                });
            }
        }

        Ok(Self {
            time_slices,
            frequency_bands,
            intensities_milli,
        })
    }

    pub const fn time_slices(&self) -> u16 {
        self.time_slices
    }

    pub const fn frequency_bands(&self) -> u16 {
        self.frequency_bands
    }

    pub fn intensities_milli(&self) -> &[u16] {
        &self.intensities_milli
    }
}

/// Validation failures for the canonical normalized audio-summary surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioSummaryError {
    SampleRateZero,
    ChannelsZero,
    DurationZero,
    WaveformBinsEmpty,
    WaveformLevelOutOfRange {
        bin_index: usize,
        field: &'static str,
        level_milli: i16,
    },
    WaveformBinRangeInvalid {
        bin_index: usize,
        min_level_milli: i16,
        max_level_milli: i16,
    },
    SpectrogramDimensionsZero {
        time_slices: u16,
        frequency_bands: u16,
    },
    SpectrogramLayoutOverflow,
    SpectrogramBinCountMismatch {
        expected: usize,
        actual: usize,
    },
    SpectrogramIntensityOutOfRange {
        bin_index: usize,
        intensity_milli: u16,
    },
}

impl fmt::Display for AudioSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SampleRateZero => {
                write!(f, "audio summaries require a non-zero sample rate")
            }
            Self::ChannelsZero => {
                write!(f, "audio summaries require at least one channel")
            }
            Self::DurationZero => {
                write!(f, "audio summaries require a non-zero duration")
            }
            Self::WaveformBinsEmpty => {
                write!(f, "audio summaries require at least one waveform bin")
            }
            Self::WaveformLevelOutOfRange {
                bin_index,
                field,
                level_milli,
            } => write!(
                f,
                "waveform bin {} {} level {} is outside the normalized milli range",
                bin_index, field, level_milli
            ),
            Self::WaveformBinRangeInvalid {
                bin_index,
                min_level_milli,
                max_level_milli,
            } => write!(
                f,
                "waveform bin {} min/max levels are invalid: {} > {}",
                bin_index, min_level_milli, max_level_milli
            ),
            Self::SpectrogramDimensionsZero {
                time_slices,
                frequency_bands,
            } => write!(
                f,
                "spectrogram dimensions must be non-zero, got {} time slices and {} frequency bands",
                time_slices, frequency_bands
            ),
            Self::SpectrogramLayoutOverflow => {
                write!(f, "spectrogram dimensions overflowed the bin layout")
            }
            Self::SpectrogramBinCountMismatch { expected, actual } => write!(
                f,
                "spectrogram bin count mismatch: expected {}, got {}",
                expected, actual
            ),
            Self::SpectrogramIntensityOutOfRange {
                bin_index,
                intensity_milli,
            } => write!(
                f,
                "spectrogram bin {} intensity {} is outside the normalized milli range",
                bin_index, intensity_milli
            ),
        }
    }
}

impl Error for AudioSummaryError {}

/// Decode failures for the first bounded audio summary slice.
#[derive(Debug)]
pub enum AudioDecodeError {
    UnsupportedMediaKind {
        kind: MediaKind,
    },
    UnsupportedAudioFormat {
        mime: Option<String>,
    },
    SampleBudgetExceeded {
        sample_count: u64,
        max_sample_count: u64,
    },
    NoDefaultTrack,
    UnsupportedCodec(u64),
    EmptyAudio,
    Symphonia(symphonia::core::errors::Error),
    Summary(AudioSummaryError),
}

impl fmt::Display for AudioDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMediaKind { kind } => {
                write!(
                    f,
                    "audio summary decode only supports audio probes, got {kind:?}"
                )
            }
            Self::UnsupportedAudioFormat { mime } => match mime {
                Some(mime) => write!(f, "audio summary decode does not yet support {mime}"),
                None => write!(f, "audio summary decode requires a supported audio format"),
            },
            Self::SampleBudgetExceeded {
                sample_count,
                max_sample_count,
            } => write!(
                f,
                "audio sample budget exceeded: {} samples exceeds first-slice budget {}",
                sample_count, max_sample_count
            ),
            Self::NoDefaultTrack => write!(f, "audio file has no default track"),
            Self::UnsupportedCodec(id) => write!(f, "unsupported audio codec ID: {id}"),
            Self::EmptyAudio => write!(f, "audio summary decode requires at least one sample"),
            Self::Symphonia(source) => write!(f, "{source}"),
            Self::Summary(source) => write!(f, "{source}"),
        }
    }
}

impl Error for AudioDecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Symphonia(source) => Some(source),
            Self::Summary(source) => Some(source),
            _ => None,
        }
    }
}

impl From<symphonia::core::errors::Error> for AudioDecodeError {
    fn from(source: symphonia::core::errors::Error) -> Self {
        Self::Symphonia(source)
    }
}

impl From<AudioSummaryError> for AudioDecodeError {
    fn from(source: AudioSummaryError) -> Self {
        Self::Summary(source)
    }
}

/// Canonical normalized audio summary consumed by future waveform and spectrogram renderers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioSummary {
    sample_rate_hz: u32,
    channels: u16,
    duration_ms: u64,
    waveform: WaveformSummary,
    spectrogram: SpectrogramSummary,
}

impl AudioSummary {
    pub fn new(
        sample_rate_hz: u32,
        channels: u16,
        duration_ms: u64,
        waveform: WaveformSummary,
        spectrogram: SpectrogramSummary,
    ) -> Result<Self, AudioSummaryError> {
        if sample_rate_hz == 0 {
            return Err(AudioSummaryError::SampleRateZero);
        }
        if channels == 0 {
            return Err(AudioSummaryError::ChannelsZero);
        }
        if duration_ms == 0 {
            return Err(AudioSummaryError::DurationZero);
        }

        Ok(Self {
            sample_rate_hz,
            channels,
            duration_ms,
            waveform,
            spectrogram,
        })
    }

    pub const fn sample_rate_hz(&self) -> u32 {
        self.sample_rate_hz
    }

    pub const fn channels(&self) -> u16 {
        self.channels
    }

    pub const fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    pub const fn waveform(&self) -> &WaveformSummary {
        &self.waveform
    }

    pub const fn spectrogram(&self) -> &SpectrogramSummary {
        &self.spectrogram
    }
}

/// Decode a bounded audio input and transform it into the shared audio-summary surface.
pub fn decode_audio_summary(
    path: &Path,
    probe: &ProbeResult,
) -> Result<AudioSummary, AudioDecodeError> {
    use symphonia::core::audio::{AudioBuffer, Signal};
    use symphonia::core::codecs::DecoderOptions;
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;

    if probe.kind != MediaKind::Audio {
        return Err(AudioDecodeError::UnsupportedMediaKind { kind: probe.kind });
    }

    let file = fs::File::open(path).map_err(|e| symphonia::core::errors::Error::IoError(e))?;
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
    )?;

    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .ok_or(AudioDecodeError::NoDefaultTrack)?;

    let mut decoder =
        symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(0);
    let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(0);
    let sample_count = track.codec_params.n_frames.unwrap_or(0);

    if sample_count > MAX_DECODE_SAMPLES as u64 {
        return Err(AudioDecodeError::SampleBudgetExceeded {
            sample_count,
            max_sample_count: MAX_DECODE_SAMPLES as u64,
        });
    }

    let mut mono_samples = Vec::new();
    let mut float_buf: Option<AudioBuffer<f32>> = None;

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(symphonia::core::errors::Error::IoError(ref e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(e) => return Err(AudioDecodeError::from(e)),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder.decode(&packet)?;

        if float_buf.as_ref().map_or(true, |b| b.spec() != decoded.spec()) {
            float_buf = Some(AudioBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec()));
        }

        if let Some(ref mut buf) = float_buf {
            decoded.convert(buf);

            let num_channels = buf.spec().channels.count();
            let num_frames = buf.frames();

            for i in 0..num_frames {
                let mut sum = 0.0_f32;
                for c in 0..num_channels {
                    sum += buf.chan(c)[i];
                }
                mono_samples.push(normalized_level_to_milli(sum / num_channels as f32));

                if mono_samples.len() >= MAX_DECODE_SAMPLES {
                    break;
                }
            }
        }

        if mono_samples.len() >= MAX_DECODE_SAMPLES {
            break;
        }
    }

    if mono_samples.is_empty() {
        return Err(AudioDecodeError::EmptyAudio);
    }

    let duration_ms = (mono_samples.len() as u64)
        .checked_mul(1_000)
        .map(|value| value.div_ceil(u64::from(sample_rate.max(1))))
        .unwrap_or(0);
    let waveform = summarize_waveform(&mono_samples)?;
    let spectrogram = summarize_spectrogram(&mono_samples)?;

    AudioSummary::new(
        sample_rate,
        channels as u16,
        duration_ms,
        waveform,
        spectrogram,
    )
    .map_err(AudioDecodeError::from)
}

fn summarize_waveform(samples_milli: &[i16]) -> Result<WaveformSummary, AudioDecodeError> {
    if samples_milli.is_empty() {
        return Err(AudioDecodeError::EmptyAudio);
    }

    let bin_count = WAVEFORM_BIN_BUDGET.min(samples_milli.len());
    let mut bins = Vec::with_capacity(bin_count);
    for bin_index in 0..bin_count {
        let start = bin_index * samples_milli.len() / bin_count;
        let end = ((bin_index + 1) * samples_milli.len() / bin_count).max(start + 1);
        let mut min_level = i16::MAX;
        let mut max_level = i16::MIN;
        for sample in &samples_milli[start..end] {
            min_level = min_level.min(*sample);
            max_level = max_level.max(*sample);
        }
        bins.push(WaveformBin::new(min_level, max_level));
    }

    WaveformSummary::new(bins).map_err(AudioDecodeError::from)
}

fn summarize_spectrogram(samples_milli: &[i16]) -> Result<SpectrogramSummary, AudioDecodeError> {
    if samples_milli.is_empty() {
        return Err(AudioDecodeError::EmptyAudio);
    }

    let time_slices = usize::from(SPECTROGRAM_TIME_SLICE_BUDGET);
    let frequency_bands = usize::from(SPECTROGRAM_BAND_BUDGET);
    let mut magnitudes = vec![0.0_f32; time_slices * frequency_bands];
    let mut max_magnitude = 0.0_f32;

    for slice_index in 0..time_slices {
        let start = slice_index * samples_milli.len() / time_slices;
        let window = sample_window(samples_milli, start);
        for band_index in 0..frequency_bands {
            let magnitude = dft_magnitude(&window, band_index + 1);
            let slot = slice_index * frequency_bands + band_index;
            magnitudes[slot] = magnitude;
            max_magnitude = max_magnitude.max(magnitude);
        }
    }

    let intensities_milli = if max_magnitude > 0.0 {
        magnitudes
            .into_iter()
            .map(|magnitude| {
                ((magnitude / max_magnitude) * NORMALIZED_INTENSITY_LIMIT_MILLI as f32).round()
                    as u16
            })
            .collect()
    } else {
        vec![0; time_slices * frequency_bands]
    };

    SpectrogramSummary::new(
        SPECTROGRAM_TIME_SLICE_BUDGET,
        SPECTROGRAM_BAND_BUDGET,
        intensities_milli,
    )
    .map_err(AudioDecodeError::from)
}

fn sample_window(samples_milli: &[i16], start: usize) -> [f32; SPECTROGRAM_WINDOW_SIZE] {
    let mut window = [0.0_f32; SPECTROGRAM_WINDOW_SIZE];
    let denominator = (SPECTROGRAM_WINDOW_SIZE.saturating_sub(1)).max(1) as f32;

    for (index, slot) in window.iter_mut().enumerate() {
        let sample = samples_milli.get(start + index).copied().unwrap_or(0) as f32 / 1_000.0;
        let phase = TAU * index as f32 / denominator;
        let hann_weight = 0.5 - 0.5 * phase.cos();
        *slot = sample * hann_weight;
    }

    window
}

fn dft_magnitude(window: &[f32; SPECTROGRAM_WINDOW_SIZE], band_index: usize) -> f32 {
    let mut real = 0.0_f32;
    let mut imaginary = 0.0_f32;

    for (sample_index, sample) in window.iter().copied().enumerate() {
        let angle = TAU * band_index as f32 * sample_index as f32 / SPECTROGRAM_WINDOW_SIZE as f32;
        real += sample * angle.cos();
        imaginary -= sample * angle.sin();
    }

    (real.mul_add(real, imaginary * imaginary)).sqrt() / SPECTROGRAM_WINDOW_SIZE as f32
}

fn normalized_level_to_milli(level: f32) -> i16 {
    (level.clamp(-1.0, 1.0) * NORMALIZED_LEVEL_LIMIT_MILLI as f32).round() as i16
}

fn validate_waveform_level(
    level_milli: i16,
    bin_index: usize,
    field: &'static str,
) -> Result<(), AudioSummaryError> {
    if !(-NORMALIZED_LEVEL_LIMIT_MILLI..=NORMALIZED_LEVEL_LIMIT_MILLI).contains(&level_milli) {
        return Err(AudioSummaryError::WaveformLevelOutOfRange {
            bin_index,
            field,
            level_milli,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::{Path, PathBuf};
    use std::process;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use hound::{SampleFormat, WavSpec, WavWriter};

    use crate::media::{MediaKind, PixelDimensions, ProbeResult, probe_path};

    use super::{
        AudioDecodeError, AudioSummary, AudioSummaryError, MAX_DECODE_SAMPLES,
        SPECTROGRAM_BAND_BUDGET, SPECTROGRAM_TIME_SLICE_BUDGET, SpectrogramSummary,
        WAVEFORM_BIN_BUDGET, WaveformBin, WaveformSummary, decode_audio_summary,
    };

    static TEMP_AUDIO_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TempWavFixture {
        path: PathBuf,
    }

    impl TempWavFixture {
        fn stereo_pulse() -> Self {
            let id = TEMP_AUDIO_COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = env::temp_dir();
            path.push(format!("atext-audio-test-{}-{}.wav", process::id(), id));

            let spec = WavSpec {
                channels: 2,
                sample_rate: 8_000,
                bits_per_sample: 16,
                sample_format: SampleFormat::Int,
            };
            let mut writer = WavWriter::create(&path, spec).expect("wav fixture should be created");
            for frame in 0..256_u16 {
                let left = if frame < 128 { i16::MAX } else { i16::MIN };
                let right = if frame % 32 < 16 {
                    i16::MAX / 2
                } else {
                    i16::MIN / 2
                };
                writer.write_sample(left).expect("left sample should write");
                writer
                    .write_sample(right)
                    .expect("right sample should write");
            }
            writer.finalize().expect("wav fixture should finalize");

            Self { path }
        }

        fn oversized() -> Self {
            let id = TEMP_AUDIO_COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = env::temp_dir();
            path.push(format!(
                "atext-audio-budget-test-{}-{}.wav",
                process::id(),
                id
            ));

            let spec = WavSpec {
                channels: 1,
                sample_rate: 8_000,
                bits_per_sample: 16,
                sample_format: SampleFormat::Int,
            };
            let mut writer = WavWriter::create(&path, spec).expect("wav fixture should be created");
            for frame in 0..(MAX_DECODE_SAMPLES as u32 + 1) {
                let sample = if frame % 2 == 0 { i16::MAX } else { i16::MIN };
                writer.write_sample(sample).expect("sample should write");
            }
            writer.finalize().expect("wav fixture should finalize");

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempWavFixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    #[test]
    fn generate_canonical_audio_fixture_if_missing() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut path = root.to_path_buf();
        path.push("src/testdata/pulse.wav");

        if path.exists() {
            return;
        }

        let spec = WavSpec {
            channels: 1,
            sample_rate: 8_000,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let mut writer = WavWriter::create(&path, spec).expect("pulse fixture should be created");
        for frame in 0..1024_u32 {
            let sample = if (frame / 128) % 2 == 0 {
                i16::MAX / 2
            } else {
                i16::MIN / 2
            };
            writer.write_sample(sample).expect("sample should write");
        }
        writer.finalize().expect("pulse fixture should finalize");
    }

    #[test]
    fn audio_summary_validates_waveform_and_spectrogram_metadata() {
        let waveform = WaveformSummary::new(vec![
            WaveformBin::new(-1_000, 250),
            WaveformBin::new(-500, 1_000),
        ])
        .expect("waveform should validate");
        let spectrogram = SpectrogramSummary::new(2, 2, vec![0, 250, 500, 1_000])
            .expect("spectrogram should validate");
        let summary = AudioSummary::new(48_000, 1, 250, waveform.clone(), spectrogram.clone())
            .expect("audio summary should validate");

        assert_eq!(summary.sample_rate_hz(), 48_000);
        assert_eq!(summary.channels(), 1);
        assert_eq!(summary.duration_ms(), 250);
        assert_eq!(summary.waveform(), &waveform);
        assert_eq!(summary.spectrogram(), &spectrogram);
        assert_eq!(summary.waveform().bin_count(), 2);
        assert_eq!(summary.spectrogram().time_slices(), 2);
        assert_eq!(summary.spectrogram().frequency_bands(), 2);
    }

    #[test]
    fn audio_summary_rejects_invalid_waveform_and_spectrogram_layout() {
        let waveform_error = WaveformSummary::new(vec![WaveformBin::new(500, -500)])
            .expect_err("inverted waveform bounds should fail");
        assert_eq!(
            waveform_error,
            AudioSummaryError::WaveformBinRangeInvalid {
                bin_index: 0,
                min_level_milli: 500,
                max_level_milli: -500,
            }
        );

        let spectrogram_error = SpectrogramSummary::new(2, 2, vec![0, 250, 500])
            .expect_err("mismatched spectrogram bins should fail");
        assert_eq!(
            spectrogram_error,
            AudioSummaryError::SpectrogramBinCountMismatch {
                expected: 4,
                actual: 3,
            }
        );
    }

    #[test]
    fn decode_audio_summary_builds_waveform_and_spectrogram_from_wav_probe() {
        let fixture = TempWavFixture::stereo_pulse();
        let probe = probe_path(fixture.path());

        let summary = decode_audio_summary(fixture.path(), &probe)
            .expect("wav probe should decode into an audio summary");

        assert_eq!(probe.kind, MediaKind::Audio);
        assert_eq!(summary.sample_rate_hz(), 8_000);
        assert_eq!(summary.channels(), 2);
        assert_eq!(summary.duration_ms(), 32);
        assert_eq!(summary.waveform().bin_count(), WAVEFORM_BIN_BUDGET.min(256));
        assert_eq!(
            summary.spectrogram().time_slices(),
            SPECTROGRAM_TIME_SLICE_BUDGET
        );
        assert_eq!(
            summary.spectrogram().frequency_bands(),
            SPECTROGRAM_BAND_BUDGET
        );
        assert!(
            summary
                .spectrogram()
                .intensities_milli()
                .iter()
                .any(|intensity| *intensity > 0),
            "spectrogram should retain non-zero signal energy"
        );
    }

    #[test]
    fn decode_audio_summary_rejects_inputs_beyond_the_first_slice_budget() {
        let fixture = TempWavFixture::oversized();
        let probe = probe_path(fixture.path());

        let error = decode_audio_summary(fixture.path(), &probe)
            .expect_err("oversized wav should be rejected");

        assert!(matches!(
            error,
            AudioDecodeError::SampleBudgetExceeded {
                sample_count,
                max_sample_count,
            }
            if sample_count == MAX_DECODE_SAMPLES as u64 + 1
                && max_sample_count == MAX_DECODE_SAMPLES as u64
        ));
    }

    #[test]
    fn decode_audio_summary_rejects_non_audio_probe_kinds() {
        let fixture = TempWavFixture::stereo_pulse();
        let probe = ProbeResult::new(MediaKind::Image).with_dimensions(PixelDimensions::new(8, 8));

        let error = decode_audio_summary(fixture.path(), &probe)
            .expect_err("non-audio probe should be rejected");

        assert!(matches!(
            error,
            AudioDecodeError::UnsupportedMediaKind {
                kind: MediaKind::Image
            }
        ));
    }
}
