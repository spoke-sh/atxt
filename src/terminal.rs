/// Terminal color capability used during renderer planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSupport {
    None,
    Ansi16,
    Ansi256,
    Truecolor,
}

/// Known multiplexers that can affect rendering behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Multiplexer {
    None,
    Tmux,
    Screen,
    Unknown,
}

/// Shared terminal capability profile used to select renderers and fallbacks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalProfile {
    pub color_support: ColorSupport,
    pub unicode_reliable: bool,
    pub animation_allowed: bool,
    pub inline_images_supported: bool,
    pub multiplexer: Multiplexer,
    pub is_remote: bool,
    pub columns: Option<u16>,
    pub rows: Option<u16>,
}

impl Default for TerminalProfile {
    fn default() -> Self {
        Self {
            color_support: ColorSupport::Ansi256,
            unicode_reliable: true,
            animation_allowed: true,
            inline_images_supported: false,
            multiplexer: Multiplexer::None,
            is_remote: false,
            columns: None,
            rows: None,
        }
    }
}
