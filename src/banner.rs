use crate::patterns::Pattern;
use crate::colors::MCColor;
use image::{imageops::overlay, DynamicImage, Rgba, RgbaImage};

const BANNER_WIDTH: u32 = 400;
const BANNER_HEIGHT: u32 = 780;

/// Represents a Minecraft banner.
pub struct Banner {
    base_color: MCColor,
    patterns: Vec<(Pattern, MCColor)>,
    image: RgbaImage,
}

impl Banner {
    /// Creates a new banner image.
    pub fn new(base_color: MCColor) -> Self {
        Self {
            base_color,
            patterns: Vec::new(),
            image: RgbaImage::new(BANNER_WIDTH, BANNER_HEIGHT),
        }
    }

    /// Adds a pattern to the banner with the given color.
    pub fn add_pattern(&mut self, pattern: Pattern, color: MCColor) -> &mut Self {
        self.patterns.push((pattern, color));
        self
    }

    /// Renders the banner image.
    pub fn render(&mut self) -> &mut Self {
        self.render_base();
        // Render the patterns
        let patterns: Vec<DynamicImage> = self.render_patterns();
        if patterns.is_empty() {
            return self
        }
        // Overlay the patterns on the base color
        patterns.iter().for_each(|pattern| {
            overlay(&mut self.image, pattern, 0, 0)
        });
        self
    }

    /// Renders the base color of the banner.
    fn render_base(&mut self) {
        for pixel in self.image.pixels_mut() {
            *pixel = Rgba([self.base_color.rgb()[0], self.base_color.rgb()[1], self.base_color.rgb()[2], 255]);
        }
    }

    /// Renders the patterns of the banner.
    fn render_patterns(&self) -> Vec<DynamicImage> {
        if self.patterns.is_empty() {
            return Vec::new();
        }

        let mut patterns: Vec<DynamicImage> = Vec::new();

        for (pattern, color) in &self.patterns {
            // Load the pattern image
            let mut pattern_image = pattern.image();

            // Change the color of the pattern to the given color
            pattern_image.as_mut_rgba8().unwrap().pixels_mut().for_each(|p| {
                if p[0] != 0 && p[1] != 0 && p[2] != 0 {
                    *p = Rgba([color.rgb()[0], color.rgb()[1], color.rgb()[2], p.0[3]]);
                }
            });

            patterns.push(pattern_image);
        };
        patterns
    }

    /// Saves the banner to the given path.
    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        self.image.save(path)
    }

    /// Returns the banner as a dynamic image.
    pub fn as_dynamic_image(&self) -> DynamicImage {
        DynamicImage::ImageRgba8(self.image.clone())
    }
}