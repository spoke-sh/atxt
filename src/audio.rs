use std::error::Error;
use std::fmt;

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
    use super::{
        AudioSummary, AudioSummaryError, SpectrogramSummary, WaveformBin, WaveformSummary,
    };

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
}
