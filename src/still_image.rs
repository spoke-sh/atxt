use std::error::Error;
use std::fmt;

use image::imageops::FilterType;
use image::{Rgba, RgbaImage};
use txtplot::canvas::BrailleCanvas;

use crate::frame::VisualFrame;
use crate::render::{OutputKind, RenderMode, RenderPlan};

const ASCII_RAMP: &[u8] = b"@%#*+=-:. ";

/// Render failures for still-image text output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StillImageRenderError {
    UnsupportedPlan {
        mode: RenderMode,
        output: OutputKind,
    },
}

impl fmt::Display for StillImageRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlan { mode, output } => write!(
                f,
                "render plan mode {:?} with output {:?} is not supported by the still-image renderer",
                mode, output
            ),
        }
    }
}

impl Error for StillImageRenderError {}

/// Render a normalized still-image frame using the already-selected render plan.
pub fn render_still_image(
    frame: &VisualFrame,
    plan: &RenderPlan,
) -> Result<String, StillImageRenderError> {
    match (plan.intent.mode, plan.output) {
        (RenderMode::Braille, OutputKind::SingleFrame) => Ok(render_braille(frame, plan)),
        (RenderMode::Ascii, OutputKind::SingleFrame) => Ok(render_ascii(frame, plan)),
        (mode, output) => Err(StillImageRenderError::UnsupportedPlan { mode, output }),
    }
}

fn render_braille(frame: &VisualFrame, plan: &RenderPlan) -> String {
    let (width_cells, height_cells) = fit_cell_dimensions(
        frame.width_px().div_ceil(2),
        frame.height_px().div_ceil(4),
        plan.intent.max_width_cells,
        plan.intent.max_height_cells,
    );
    let resized = resize_frame(
        frame,
        u32::from(width_cells) * 2,
        u32::from(height_cells) * 4,
    );
    let mut canvas = BrailleCanvas::new(usize::from(width_cells), usize::from(height_cells));

    for y in 0..resized.height() {
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y);
            if pixel_is_foreground(pixel) {
                canvas.set_pixel_screen(x as usize, y as usize, None);
            }
        }
    }

    canvas.render_no_color()
}

fn render_ascii(frame: &VisualFrame, plan: &RenderPlan) -> String {
    let (width_chars, height_chars) = fit_cell_dimensions(
        frame.width_px(),
        frame.height_px(),
        plan.intent.max_width_cells,
        plan.intent.max_height_cells,
    );
    let resized = resize_frame(frame, u32::from(width_chars), u32::from(height_chars));
    let mut output = String::with_capacity(
        usize::from(width_chars)
            .checked_mul(usize::from(height_chars))
            .unwrap_or(0)
            .saturating_add(usize::from(height_chars)),
    );

    for y in 0..resized.height() {
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y);
            output.push(ascii_for_pixel(pixel));
        }
        output.push('\n');
    }

    output
}

fn fit_cell_dimensions(
    source_width: u32,
    source_height: u32,
    max_width: Option<u16>,
    max_height: Option<u16>,
) -> (u16, u16) {
    let source_width = source_width.max(1);
    let source_height = source_height.max(1);
    let max_width =
        u32::from(max_width.unwrap_or(u16::try_from(source_width).unwrap_or(u16::MAX))).max(1);
    let max_height =
        u32::from(max_height.unwrap_or(u16::try_from(source_height).unwrap_or(u16::MAX))).max(1);

    if source_width <= max_width && source_height <= max_height {
        return (
            u16::try_from(source_width).unwrap_or(u16::MAX),
            u16::try_from(source_height).unwrap_or(u16::MAX),
        );
    }

    let width_limited = u64::from(source_width) * u64::from(max_height)
        >= u64::from(source_height) * u64::from(max_width);

    let (target_width, target_height) = if width_limited {
        let target_width = max_width;
        let target_height = ((u64::from(source_height) * u64::from(max_width))
            .div_ceil(u64::from(source_width)))
        .max(1) as u32;
        (target_width, target_height.min(max_height))
    } else {
        let target_height = max_height;
        let target_width = ((u64::from(source_width) * u64::from(max_height))
            .div_ceil(u64::from(source_height)))
        .max(1) as u32;
        (target_width.min(max_width), target_height)
    };

    (
        u16::try_from(target_width).unwrap_or(u16::MAX),
        u16::try_from(target_height).unwrap_or(u16::MAX),
    )
}

fn resize_frame(frame: &VisualFrame, width: u32, height: u32) -> RgbaImage {
    let width = width.max(1);
    let height = height.max(1);
    let source = RgbaImage::from_vec(frame.width_px(), frame.height_px(), frame_bytes(frame))
        .expect("visual frame byte layout must match dimensions");

    if source.width() == width && source.height() == height {
        source
    } else {
        image::imageops::resize(&source, width, height, FilterType::Nearest)
    }
}

fn frame_bytes(frame: &VisualFrame) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(frame.pixel_count().saturating_mul(4));
    for pixel in frame.pixels() {
        bytes.extend_from_slice(&[pixel.r, pixel.g, pixel.b, pixel.a]);
    }
    bytes
}

fn pixel_is_foreground(pixel: &Rgba<u8>) -> bool {
    blended_luma(pixel) < 192
}

fn ascii_for_pixel(pixel: &Rgba<u8>) -> char {
    let luma = usize::from(blended_luma(pixel));
    let last_index = ASCII_RAMP.len().saturating_sub(1);
    let index = luma.saturating_mul(last_index) / 255;
    ASCII_RAMP[index] as char
}

fn blended_luma(pixel: &Rgba<u8>) -> u8 {
    let [r, g, b, a] = pixel.0;
    let base = (u16::from(r) * 77 + u16::from(g) * 150 + u16::from(b) * 29) >> 8;
    let alpha = u16::from(a);
    let blended = (base * alpha + 255 * (255 - alpha)) / 255;
    blended as u8
}

#[cfg(test)]
mod tests {
    use crate::frame::{Rgba8, VisualFrame};
    use crate::media::{MediaKind, PixelDimensions, ProbeCompleteness, ProbeResult};
    use crate::render::{OutputKind, RenderMode, RenderPlan, plan_render};
    use crate::terminal::{ColorSupport, Multiplexer, SessionMode, TerminalProfile, TerminalSize};

    use super::{StillImageRenderError, render_still_image};

    #[test]
    fn braille_renderer_renders_a_full_cell_via_txtplot() {
        let frame = VisualFrame::new(
            PixelDimensions::new(2, 4),
            vec![Rgba8::new(0, 0, 0, 255); 8],
        )
        .expect("frame should validate");
        let plan = RenderPlan {
            intent: crate::render::RenderIntent {
                mode: RenderMode::Braille,
                max_width_cells: Some(1),
                max_height_cells: Some(1),
                frame_rate_hint: None,
                color_enabled: false,
            },
            output: OutputKind::SingleFrame,
            degraded: false,
            reason: crate::render::PlanningReason::Direct,
        };

        let output = render_still_image(&frame, &plan).expect("braille render should succeed");

        assert_eq!(output, "⣿\n");
    }

    #[test]
    fn ascii_renderer_is_used_when_shared_planning_degrades_image_output() {
        let frame = VisualFrame::new(
            PixelDimensions::new(2, 1),
            vec![Rgba8::new(0, 0, 0, 255), Rgba8::new(255, 255, 255, 255)],
        )
        .expect("frame should validate");
        let probe =
            ProbeResult::new(MediaKind::Image).with_completeness(ProbeCompleteness::Partial);
        let terminal = TerminalProfile {
            color_support: ColorSupport::None,
            unicode_reliable: false,
            animation_allowed: false,
            inline_images_supported: false,
            multiplexer: Multiplexer::None,
            is_remote: false,
            session_mode: SessionMode::Captured,
            size: Some(TerminalSize::new(2, 1)),
        };
        let plan = plan_render(&probe, &terminal);

        let output = render_still_image(&frame, &plan).expect("ascii render should succeed");

        assert_eq!(plan.intent.mode, RenderMode::Ascii);
        assert_eq!(output, "@ \n");
    }

    #[test]
    fn still_image_renderer_rejects_non_single_frame_modes() {
        let frame = VisualFrame::new(PixelDimensions::new(1, 1), vec![Rgba8::new(0, 0, 0, 255)])
            .expect("frame should validate");
        let plan = RenderPlan {
            intent: crate::render::RenderIntent {
                mode: RenderMode::ContactSheet,
                max_width_cells: Some(1),
                max_height_cells: Some(1),
                frame_rate_hint: None,
                color_enabled: false,
            },
            output: OutputKind::SingleFrame,
            degraded: true,
            reason: crate::render::PlanningReason::CapturedSequenceFallback,
        };

        let error = render_still_image(&frame, &plan).unwrap_err();

        assert_eq!(
            error,
            StillImageRenderError::UnsupportedPlan {
                mode: RenderMode::ContactSheet,
                output: OutputKind::SingleFrame,
            }
        );
    }
}
