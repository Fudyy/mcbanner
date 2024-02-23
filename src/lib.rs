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
        let mut banner = Banner::new(crate::colors::MCColor::Green);
        banner.render();
    }

    #[test]
    fn test_banner_with_pattern() {
        let mut banner = Banner::new(crate::colors::MCColor::Green);
        banner.add_pattern(crate::patterns::Pattern::Bricks, crate::colors::MCColor::Red);
        banner.render();
    }
}