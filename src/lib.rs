//! # Overview
//! This library is a simple way to create banners from Minecraft. It uses the [image](https://docs.rs/image/) crate under the hood to generate the images.
//!
//! ## Usage
//! The [Banner] struct is the main entrypoint of the library.
//! ```rust,no_run
//! use mcbanner::{Banner, Pattern, MCColor};
//! 
//! fn main() {
//!     let mut banner = Banner::new(MCColor::Red);
//!     banner.add_pattern(Pattern::Bricks, MCColor::Orange);
//!     banner.render();
//!     banner.save("banner.png").unwrap();
//! }

mod banner;
mod patterns;
mod colors;

pub use banner::Banner;
pub use patterns::Pattern;
pub use colors::MCColor;

#[cfg(test)]
mod test {
    use crate::Banner;

    #[test]
    fn test_banner() {
        let mut banner = Banner::new(crate::colors::MCColor::Red);
        banner.render();
    }

    #[test]
    fn test_banner_with_pattern() {
        let mut banner = Banner::new(crate::colors::MCColor::Lime);
        banner.add_pattern(crate::patterns::Pattern::Creeper, crate::colors::MCColor::Black);
        banner.render();
        banner.save("banner.png").unwrap();
    }
}