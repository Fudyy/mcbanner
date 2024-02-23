pub mod patterns;
pub mod colors;

use patterns::Pattern;
use colors::MCColor;
use image::{imageops::overlay, DynamicImage, Rgba, RgbaImage};

const BANNER_WIDTH: u32 = 400;
const BANNER_HEIGHT: u32 = 780;
const PATTERNS_PATH: &str = "patterns/";

/// Represents a banner in Minecraft.
pub struct Banner {
    base_color: MCColor,
    patterns: Vec<(Pattern, MCColor)>,
    image: RgbaImage,
}

impl Banner {
    /// Creates a new banner image.
    pub fn new(base_color: MCColor) -> Banner {
        Banner {
            base_color,
            patterns: Vec::new(),
            image: RgbaImage::new(BANNER_WIDTH, BANNER_HEIGHT),
        }
    }

    /// Adds a pattern to the banner with the given color.
    pub fn add_pattern(&mut self, pattern: Pattern, color: MCColor) {
        self.patterns.push((pattern, color));
    }

    /// Renders the banner
    pub fn render(&mut self) {
        self.render_base();
        // Render the patterns
        let patterns: Vec<DynamicImage> = self.render_patterns();
        if patterns.is_empty() {
            return;
        }
        // Overlay the patterns on the base color
        patterns.iter().for_each(|pattern| {
            overlay(&mut self.image, pattern, 0, 0)
        });
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
            let pattern_path = format!("{}{}.png", PATTERNS_PATH, pattern.to_lowercase_string());
            let mut pattern_image = image::open(pattern_path).unwrap();

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
    pub fn save(&self, path: &str) {
        self.image.save(path).unwrap();
    }
}