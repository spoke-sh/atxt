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

#[cfg(test)]
mod tests {
    use super::{RenderIntent, RenderMode};

    #[test]
    fn render_intent_defaults_to_braille() {
        let intent = RenderIntent::default();
        assert_eq!(intent.mode, RenderMode::Braille);
        assert!(intent.color_enabled);
    }
}
