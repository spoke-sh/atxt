use std::error::Error;
use std::fmt;

use txtplot::canvas::BrailleCanvas;

use crate::audio::AudioSummary;
use crate::render::{OutputKind, RenderMode, RenderPlan};

/// Render failures for audio-summary text output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioRenderError {
    UnsupportedPlan {
        mode: RenderMode,
        output: OutputKind,
    },
}

impl fmt::Display for AudioRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlan { mode, output } => write!(
                f,
                "render plan mode {:?} with output {:?} is not supported by the audio renderer",
                mode, output
            ),
        }
    }
}

impl Error for AudioRenderError {}

/// Render a normalized audio summary using the already-selected render plan.
pub fn render_audio_summary(
    summary: &AudioSummary,
    plan: &RenderPlan,
) -> Result<String, AudioRenderError> {
    match (plan.intent.mode, plan.output) {
        (RenderMode::Waveform, OutputKind::AudioVisualization) => Ok(render_waveform(summary, plan)),
        (RenderMode::Spectrogram, OutputKind::AudioVisualization) => {
            Ok(render_spectrogram(summary, plan))
        }
        (RenderMode::Ascii, OutputKind::AudioVisualization) => Ok(render_ascii_waveform(summary, plan)),
        (mode, output) => Err(AudioRenderError::UnsupportedPlan { mode, output }),
    }
}

fn render_waveform(summary: &AudioSummary, plan: &RenderPlan) -> String {
    let width_cells = plan.intent.max_width_cells.unwrap_or(64).max(1) as usize;
    let height_cells = plan.intent.max_height_cells.unwrap_or(8).max(1) as usize;
    let mut canvas = BrailleCanvas::new(width_cells, height_cells);

    let bins = summary.waveform().bins();
    let height_px = height_cells * 4;
    let mid_y = height_px / 2;

    for (x_cell, bin) in bins.iter().enumerate() {
        if x_cell >= width_cells * 2 {
            break;
        }

        // Map normalized -1000..1000 to pixel range
        let min_y = (mid_y as i32 - (bin.min_level_milli as i32 * mid_y as i32 / 1000))
            .clamp(0, height_px as i32 - 1) as usize;
        let max_y = (mid_y as i32 - (bin.max_level_milli as i32 * mid_y as i32 / 1000))
            .clamp(0, height_px as i32 - 1) as usize;

        let (low, high) = if min_y < max_y {
            (min_y, max_y)
        } else {
            (max_y, min_y)
        };

        for y in low..=high {
            canvas.set_pixel_screen(x_cell, y, None);
        }
    }

    canvas.render_no_color()
}

fn render_spectrogram(summary: &AudioSummary, plan: &RenderPlan) -> String {
    let width_cells = plan.intent.max_width_cells.unwrap_or(64).max(1) as usize;
    let height_cells = plan.intent.max_height_cells.unwrap_or(16).max(1) as usize;
    let mut canvas = BrailleCanvas::new(width_cells, height_cells);

    let spectrogram = summary.spectrogram();
    let intensities = spectrogram.intensities_milli();
    let slices = spectrogram.time_slices() as usize;
    let bands = spectrogram.frequency_bands() as usize;

    let width_px = width_cells * 2;
    let height_px = height_cells * 4;

    for y_px in 0..height_px {
        let band_index = bands.saturating_sub(1) - (y_px * bands / height_px);
        for x_px in 0..width_px {
            let slice_index = x_px * slices / width_px;
            let intensity = intensities[slice_index * bands + band_index];
            if intensity > 300 {
                canvas.set_pixel_screen(x_px, y_px, None);
            }
        }
    }

    canvas.render_no_color()
}

fn render_ascii_waveform(summary: &AudioSummary, plan: &RenderPlan) -> String {
    let width = plan.intent.max_width_cells.unwrap_or(64).max(1) as usize;
    let height = plan.intent.max_height_cells.unwrap_or(8).max(1) as usize;
    let mid_y = height / 2;
    let bins = summary.waveform().bins();

    let mut grid = vec![vec![' '; width]; height];

    for (x, bin) in bins.iter().enumerate() {
        if x >= width {
            break;
        }

        let min_y = (mid_y as i32 - (bin.min_level_milli as i32 * mid_y as i32 / 1000))
            .clamp(0, height as i32 - 1) as usize;
        let max_y = (mid_y as i32 - (bin.max_level_milli as i32 * mid_y as i32 / 1000))
            .clamp(0, height as i32 - 1) as usize;

        let (low, high) = if min_y < max_y {
            (min_y, max_y)
        } else {
            (max_y, min_y)
        };

        for y in low..=high {
            grid[y][x] = '#';
        }
    }

    let mut output = String::new();
    for row in grid {
        output.extend(row);
        output.push('\n');
    }
    output
}
