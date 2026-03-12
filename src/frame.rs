use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::media::{MediaKind, PixelDimensions, probe_path};

/// RGBA pixel used by normalized still-image frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba8 {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Validation errors for the canonical still-image frame surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualFrameError {
    DimensionsOverflow {
        dimensions: PixelDimensions,
    },
    PixelCountMismatch {
        dimensions: PixelDimensions,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for VisualFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionsOverflow { dimensions } => write!(
                f,
                "visual frame dimensions overflow pixel count calculations: {}x{}",
                dimensions.width_px, dimensions.height_px
            ),
            Self::PixelCountMismatch {
                dimensions,
                expected,
                actual,
            } => write!(
                f,
                "visual frame pixel count mismatch for {}x{} frame: expected {}, got {}",
                dimensions.width_px, dimensions.height_px, expected, actual
            ),
        }
    }
}

impl Error for VisualFrameError {}

/// Canonical normalized still-image frame consumed by renderers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualFrame {
    dimensions: PixelDimensions,
    pixels: Vec<Rgba8>,
}

impl VisualFrame {
    pub fn new(dimensions: PixelDimensions, pixels: Vec<Rgba8>) -> Result<Self, VisualFrameError> {
        let expected = expected_pixel_count(dimensions)?;
        let actual = pixels.len();

        if actual != expected {
            return Err(VisualFrameError::PixelCountMismatch {
                dimensions,
                expected,
                actual,
            });
        }

        Ok(Self { dimensions, pixels })
    }

    pub const fn dimensions(&self) -> PixelDimensions {
        self.dimensions
    }

    pub const fn width_px(&self) -> u32 {
        self.dimensions.width_px
    }

    pub const fn height_px(&self) -> u32 {
        self.dimensions.height_px
    }

    pub fn pixel_count(&self) -> usize {
        self.pixels.len()
    }

    pub fn pixels(&self) -> &[Rgba8] {
        &self.pixels
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Rgba8> {
        if x >= self.width_px() || y >= self.height_px() {
            return None;
        }

        let width = usize::try_from(self.width_px()).ok()?;
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        let index = y.checked_mul(width)?.checked_add(x)?;

        self.pixels.get(index).copied()
    }
}

fn expected_pixel_count(dimensions: PixelDimensions) -> Result<usize, VisualFrameError> {
    let width = usize::try_from(dimensions.width_px)
        .map_err(|_| VisualFrameError::DimensionsOverflow { dimensions })?;
    let height = usize::try_from(dimensions.height_px)
        .map_err(|_| VisualFrameError::DimensionsOverflow { dimensions })?;

    width
        .checked_mul(height)
        .ok_or(VisualFrameError::DimensionsOverflow { dimensions })
}

/// Decode failures for path-based still-image loading.
#[derive(Debug)]
pub enum StillImageDecodeError {
    UnsupportedMediaKind {
        path: PathBuf,
        kind: MediaKind,
    },
    DecodeFailed {
        path: PathBuf,
        source: image::ImageError,
    },
    InvalidFrame(VisualFrameError),
}

impl fmt::Display for StillImageDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMediaKind { path, kind } => write!(
                f,
                "path '{}' is not a supported static image (detected kind: {:?})",
                path.display(),
                kind
            ),
            Self::DecodeFailed { path, source } => {
                write!(
                    f,
                    "failed to decode still image '{}': {}",
                    path.display(),
                    source
                )
            }
            Self::InvalidFrame(error) => {
                write!(f, "decoded still image produced invalid frame: {error}")
            }
        }
    }
}

impl Error for StillImageDecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DecodeFailed { source, .. } => Some(source),
            Self::InvalidFrame(source) => Some(source),
            Self::UnsupportedMediaKind { .. } => None,
        }
    }
}

/// Decode a supported static image file into the canonical still-image frame surface.
pub fn decode_still_image(path: &Path) -> Result<VisualFrame, StillImageDecodeError> {
    let probe = probe_path(path);
    if probe.kind != MediaKind::Image {
        return Err(StillImageDecodeError::UnsupportedMediaKind {
            path: path.to_path_buf(),
            kind: probe.kind,
        });
    }

    let image = image::open(path).map_err(|source| StillImageDecodeError::DecodeFailed {
        path: path.to_path_buf(),
        source,
    })?;
    let rgba = image.into_rgba8();
    let dimensions = PixelDimensions::new(rgba.width(), rgba.height());
    let pixels = rgba
        .pixels()
        .map(|pixel| {
            let [r, g, b, a] = pixel.0;
            Rgba8::new(r, g, b, a)
        })
        .collect();

    VisualFrame::new(dimensions, pixels).map_err(StillImageDecodeError::InvalidFrame)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::{Path, PathBuf};
    use std::process;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use image::{Rgb, RgbImage, Rgba, RgbaImage};

    use super::{Rgba8, StillImageDecodeError, VisualFrame, VisualFrameError, decode_still_image};
    use crate::media::{MediaKind, PixelDimensions};

    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TempImagePath {
        path: PathBuf,
    }

    impl TempImagePath {
        fn new(extension: &str) -> Self {
            let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
            let mut path = env::temp_dir();
            path.push(format!(
                "atext-frame-test-{}-{}.{}",
                process::id(),
                id,
                extension
            ));
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempImagePath {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    #[test]
    fn visual_frame_validates_pixel_count_and_exposes_pixels() {
        let frame = VisualFrame::new(
            PixelDimensions::new(2, 1),
            vec![Rgba8::new(1, 2, 3, 255), Rgba8::new(4, 5, 6, 128)],
        )
        .expect("frame should validate");

        assert_eq!(frame.dimensions(), PixelDimensions::new(2, 1));
        assert_eq!(frame.width_px(), 2);
        assert_eq!(frame.height_px(), 1);
        assert_eq!(frame.pixel_count(), 2);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba8::new(1, 2, 3, 255)));
        assert_eq!(frame.get_pixel(1, 0), Some(Rgba8::new(4, 5, 6, 128)));
        assert_eq!(frame.get_pixel(2, 0), None);

        let error =
            VisualFrame::new(PixelDimensions::new(2, 2), vec![Rgba8::default(); 3]).unwrap_err();
        assert_eq!(
            error,
            VisualFrameError::PixelCountMismatch {
                dimensions: PixelDimensions::new(2, 2),
                expected: 4,
                actual: 3,
            }
        );
    }

    #[test]
    fn decode_still_image_supports_initial_static_image_families() {
        let png_path = TempImagePath::new("png");
        let jpg_path = TempImagePath::new("jpg");
        let bmp_path = TempImagePath::new("bmp");

        let mut png = RgbaImage::new(2, 1);
        png.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        png.put_pixel(1, 0, Rgba([0, 255, 0, 128]));
        png.save(png_path.path()).expect("png fixture should save");

        let mut jpg = RgbImage::new(2, 1);
        jpg.put_pixel(0, 0, Rgb([10, 20, 30]));
        jpg.put_pixel(1, 0, Rgb([200, 180, 160]));
        jpg.save(jpg_path.path()).expect("jpeg fixture should save");

        let mut bmp = RgbaImage::new(1, 2);
        bmp.put_pixel(0, 0, Rgba([40, 50, 60, 255]));
        bmp.put_pixel(0, 1, Rgba([70, 80, 90, 255]));
        bmp.save(bmp_path.path()).expect("bmp fixture should save");

        let png_frame = decode_still_image(png_path.path()).expect("png should decode");
        assert_eq!(png_frame.dimensions(), PixelDimensions::new(2, 1));
        assert_eq!(png_frame.pixel_count(), 2);
        assert_eq!(png_frame.get_pixel(0, 0), Some(Rgba8::new(255, 0, 0, 255)));
        assert_eq!(png_frame.get_pixel(1, 0), Some(Rgba8::new(0, 255, 0, 128)));

        let jpg_frame = decode_still_image(jpg_path.path()).expect("jpeg should decode");
        assert_eq!(jpg_frame.dimensions(), PixelDimensions::new(2, 1));
        assert_eq!(jpg_frame.pixel_count(), 2);

        let bmp_frame = decode_still_image(bmp_path.path()).expect("bmp should decode");
        assert_eq!(bmp_frame.dimensions(), PixelDimensions::new(1, 2));
        assert_eq!(bmp_frame.pixel_count(), 2);
        assert_eq!(bmp_frame.get_pixel(0, 0), Some(Rgba8::new(40, 50, 60, 255)));
        assert_eq!(bmp_frame.get_pixel(0, 1), Some(Rgba8::new(70, 80, 90, 255)));
    }

    #[test]
    fn decode_still_image_rejects_non_static_media_kinds() {
        let error = decode_still_image(Path::new("loop.gif")).unwrap_err();

        match error {
            StillImageDecodeError::UnsupportedMediaKind { kind, .. } => {
                assert_eq!(kind, MediaKind::AnimatedImage);
            }
            other => panic!("expected unsupported media kind error, got {other:?}"),
        }
    }
}
