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

/// Probe-level metadata collected before full decoding.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProbeResult {
    pub kind: MediaKind,
    pub mime: Option<String>,
    pub width_px: Option<u32>,
    pub height_px: Option<u32>,
    pub frame_count: Option<u64>,
    pub duration_ms: Option<u64>,
    pub sample_rate_hz: Option<u32>,
    pub channels: Option<u16>,
}

impl ProbeResult {
    pub const fn new(kind: MediaKind) -> Self {
        Self {
            kind,
            mime: None,
            width_px: None,
            height_px: None,
            frame_count: None,
            duration_ms: None,
            sample_rate_hz: None,
            channels: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MediaKind, ProbeResult};

    #[test]
    fn probe_result_defaults_to_unknown() {
        let result = ProbeResult::default();
        assert_eq!(result.kind, MediaKind::Unknown);
    }
}
