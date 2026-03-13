use crate::media::{MediaKind, ProbeResult};
use crate::terminal::{ColorSupport, SessionMode, TerminalProfile};

const AUDIO_SPECTROGRAM_MIN_COLUMNS: u16 = 48;
const AUDIO_SPECTROGRAM_MIN_ROWS: u16 = 12;

/// Output shape produced by the selected renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputKind {
    SingleFrame,
    FrameSequence,
    AudioVisualization,
    MetadataSummary,
}

/// High-level renderer family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderMode {
    Ascii,
    Blocks,
    Braille,
    ContactSheet,
    Waveform,
    Spectrogram,
    Metadata,
}

/// User or system intent that guides renderer selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderIntent {
    pub mode: RenderMode,
    pub max_width_cells: Option<u16>,
    pub max_height_cells: Option<u16>,
    pub frame_rate_hint: Option<u16>,
    pub color_enabled: bool,
}

impl Default for RenderIntent {
    fn default() -> Self {
        Self {
            mode: RenderMode::Braille,
            max_width_cells: None,
            max_height_cells: None,
            frame_rate_hint: None,
            color_enabled: true,
        }
    }
}

/// Why render planning selected its current strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanningReason {
    Direct,
    CapturedSequenceFallback,
    DensityFallback,
    UnicodeFallback,
    UnknownMediaFallback,
    DocumentFallback,
}

/// A render decision derived from probe and terminal capability data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderPlan {
    pub intent: RenderIntent,
    pub output: OutputKind,
    pub degraded: bool,
    pub reason: PlanningReason,
}

/// Combine probe and terminal capability data into a concrete render plan.
pub fn plan_render(probe: &ProbeResult, terminal: &TerminalProfile) -> RenderPlan {
    let mut intent = RenderIntent {
        mode: RenderMode::Braille,
        max_width_cells: terminal.size.map(|size| size.columns),
        max_height_cells: terminal.size.map(|size| size.rows),
        frame_rate_hint: None,
        color_enabled: terminal.color_support != ColorSupport::None,
    };

    match probe.kind {
        MediaKind::Unknown => {
            intent.mode = RenderMode::Metadata;
            RenderPlan {
                intent,
                output: OutputKind::MetadataSummary,
                degraded: true,
                reason: PlanningReason::UnknownMediaFallback,
            }
        }
        MediaKind::Document => {
            intent.mode = RenderMode::Metadata;
            RenderPlan {
                intent,
                output: OutputKind::MetadataSummary,
                degraded: true,
                reason: PlanningReason::DocumentFallback,
            }
        }
        MediaKind::Audio => {
            if !terminal.unicode_reliable {
                intent.mode = RenderMode::Ascii;
                RenderPlan {
                    intent,
                    output: OutputKind::AudioVisualization,
                    degraded: true,
                    reason: PlanningReason::UnicodeFallback,
                }
            } else if audio_supports_spectrogram(terminal) {
                intent.mode = RenderMode::Spectrogram;
                RenderPlan {
                    intent,
                    output: OutputKind::AudioVisualization,
                    degraded: false,
                    reason: PlanningReason::Direct,
                }
            } else {
                intent.mode = RenderMode::Waveform;
                RenderPlan {
                    intent,
                    output: OutputKind::AudioVisualization,
                    degraded: true,
                    reason: PlanningReason::DensityFallback,
                }
            }
        }
        MediaKind::AnimatedImage | MediaKind::Video => {
            if !terminal.unicode_reliable {
                intent.mode = RenderMode::Ascii;
                RenderPlan {
                    intent,
                    output: OutputKind::SingleFrame,
                    degraded: true,
                    reason: PlanningReason::UnicodeFallback,
                }
            } else if matches!(terminal.session_mode, SessionMode::Captured)
                || !terminal.animation_allowed
            {
                intent.mode = RenderMode::ContactSheet;
                RenderPlan {
                    intent,
                    output: OutputKind::SingleFrame,
                    degraded: true,
                    reason: PlanningReason::CapturedSequenceFallback,
                }
            } else {
                intent.mode = RenderMode::ContactSheet;
                RenderPlan {
                    intent,
                    output: OutputKind::SingleFrame,
                    degraded: false,
                    reason: PlanningReason::Direct,
                }
            }
        }
        MediaKind::Image => {
            if !terminal.unicode_reliable {
                intent.mode = RenderMode::Ascii;
                RenderPlan {
                    intent,
                    output: OutputKind::SingleFrame,
                    degraded: true,
                    reason: PlanningReason::UnicodeFallback,
                }
            } else {
                intent.mode = RenderMode::Braille;
                RenderPlan {
                    intent,
                    output: OutputKind::SingleFrame,
                    degraded: false,
                    reason: PlanningReason::Direct,
                }
            }
        }
    }
}

fn audio_supports_spectrogram(terminal: &TerminalProfile) -> bool {
    terminal
        .size
        .map(|size| {
            size.columns >= AUDIO_SPECTROGRAM_MIN_COLUMNS && size.rows >= AUDIO_SPECTROGRAM_MIN_ROWS
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use crate::media::{MediaKind, ProbeCompleteness, ProbeResult};
    use crate::terminal::{ColorSupport, Multiplexer, SessionMode, TerminalProfile, TerminalSize};

    use super::{OutputKind, PlanningReason, RenderIntent, RenderMode, plan_render};

    #[test]
    fn render_intent_defaults_to_braille() {
        let intent = RenderIntent::default();
        assert_eq!(intent.mode, RenderMode::Braille);
        assert!(intent.color_enabled);
    }

    #[test]
    fn render_planning_falls_back_conservatively_for_captured_video() {
        let probe =
            ProbeResult::new(MediaKind::Video).with_completeness(ProbeCompleteness::Partial);
        let terminal = TerminalProfile {
            color_support: ColorSupport::Ansi256,
            unicode_reliable: true,
            animation_allowed: false,
            inline_images_supported: false,
            multiplexer: Multiplexer::Tmux,
            is_remote: true,
            session_mode: SessionMode::Captured,
            size: Some(TerminalSize::new(100, 32)),
        };

        let plan = plan_render(&probe, &terminal);

        assert_eq!(plan.intent.mode, RenderMode::ContactSheet);
        assert_eq!(plan.output, OutputKind::SingleFrame);
        assert!(plan.degraded);
        assert_eq!(plan.reason, PlanningReason::CapturedSequenceFallback);
    }

    #[test]
    fn render_planning_prefers_braille_for_interactive_images() {
        let probe =
            ProbeResult::new(MediaKind::Image).with_completeness(ProbeCompleteness::Partial);
        let terminal = TerminalProfile {
            color_support: ColorSupport::Truecolor,
            unicode_reliable: true,
            animation_allowed: true,
            inline_images_supported: false,
            multiplexer: Multiplexer::None,
            is_remote: false,
            session_mode: SessionMode::Interactive,
            size: Some(TerminalSize::new(120, 40)),
        };

        let plan = plan_render(&probe, &terminal);

        assert_eq!(plan.intent.mode, RenderMode::Braille);
        assert_eq!(plan.output, OutputKind::SingleFrame);
        assert!(!plan.degraded);
        assert_eq!(plan.reason, PlanningReason::Direct);
    }

    #[test]
    fn render_planning_prefers_contact_sheet_for_interactive_timed_media() {
        let probe = ProbeResult::new(MediaKind::AnimatedImage)
            .with_completeness(ProbeCompleteness::Partial);
        let terminal = TerminalProfile {
            color_support: ColorSupport::Truecolor,
            unicode_reliable: true,
            animation_allowed: true,
            inline_images_supported: false,
            multiplexer: Multiplexer::None,
            is_remote: false,
            session_mode: SessionMode::Interactive,
            size: Some(TerminalSize::new(120, 40)),
        };

        let plan = plan_render(&probe, &terminal);

        assert_eq!(plan.intent.mode, RenderMode::ContactSheet);
        assert_eq!(plan.output, OutputKind::SingleFrame);
        assert!(!plan.degraded);
        assert_eq!(plan.reason, PlanningReason::Direct);
    }

    #[test]
    fn render_planning_prefers_spectrogram_for_dense_audio_terminals() {
        let probe =
            ProbeResult::new(MediaKind::Audio).with_completeness(ProbeCompleteness::Partial);
        let terminal = TerminalProfile {
            color_support: ColorSupport::Truecolor,
            unicode_reliable: true,
            animation_allowed: true,
            inline_images_supported: false,
            multiplexer: Multiplexer::None,
            is_remote: false,
            session_mode: SessionMode::Interactive,
            size: Some(TerminalSize::new(120, 40)),
        };

        let plan = plan_render(&probe, &terminal);

        assert_eq!(plan.intent.mode, RenderMode::Spectrogram);
        assert_eq!(plan.output, OutputKind::AudioVisualization);
        assert!(!plan.degraded);
        assert_eq!(plan.reason, PlanningReason::Direct);
    }

    #[test]
    fn render_planning_falls_back_to_waveform_for_low_density_audio_terminals() {
        let probe =
            ProbeResult::new(MediaKind::Audio).with_completeness(ProbeCompleteness::Partial);
        let terminal = TerminalProfile {
            color_support: ColorSupport::Ansi256,
            unicode_reliable: true,
            animation_allowed: true,
            inline_images_supported: false,
            multiplexer: Multiplexer::None,
            is_remote: false,
            session_mode: SessionMode::Interactive,
            size: Some(TerminalSize::new(24, 8)),
        };

        let plan = plan_render(&probe, &terminal);

        assert_eq!(plan.intent.mode, RenderMode::Waveform);
        assert_eq!(plan.output, OutputKind::AudioVisualization);
        assert!(plan.degraded);
        assert_eq!(plan.reason, PlanningReason::DensityFallback);
    }
}
