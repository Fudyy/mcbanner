use std::io::Cursor;

use image::{io::Reader as ImageReader, DynamicImage};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "patterns/"]
struct Asset;

/// Represents a pattern for a Minecraft banner.
pub enum Pattern {
    SquareTopLeft,
    SquareTopRight,
    SquareBottomLeft,
    SquareBottomRight,
    StripeBottom,
    StripeTop,
    StripeLeft,
    StripeRight,
    StripeCenter,
    StripeMiddle,
    StripeDownright,
    StripeDownleft,
    StripeSmall,
    Cross,
    StraightCross,
    Border,
    CurlyBorder,
    TriangleBottom,
    TriangleTop,
    TrianglesBottom,
    TrianglesTop,
    DiagonalTopLeft,
    DiagonalTopRight,
    DiagonalBottomLeft,
    DiagonalBottomRight,
    CircleMiddle,
    RhombusMiddle,
    HalfVertical,
    HalfVerticalMirror,
    HalfHorizontal,
    HalfHorizontalMirror,
    Creeper,
    Bricks,
    Gradient,
    GradientUp,
    Skull,
    Flower,
    Thing,
}

impl Pattern {
    fn asset_name(&self) -> String {
        match self {
            Pattern::SquareTopLeft => "square_top_left.png",
            Pattern::SquareTopRight => "square_top_right.png",
            Pattern::SquareBottomLeft => "square_bottom_left.png",
            Pattern::SquareBottomRight => "square_bottom_right.png",
            Pattern::StripeBottom => "stripe_bottom.png",
            Pattern::StripeTop => "stripe_top.png",
            Pattern::StripeLeft => "stripe_left.png",
            Pattern::StripeRight => "stripe_right.png",
            Pattern::StripeCenter => "stripe_center.png",
            Pattern::StripeMiddle => "stripe_middle.png",
            Pattern::StripeDownright => "stripe_downright.png",
            Pattern::StripeDownleft => "stripe_downleft.png",
            Pattern::StripeSmall => "stripe_small.png",
            Pattern::Cross => "cross.png",
            Pattern::StraightCross => "straight_cross.png",
            Pattern::Border => "border.png",
            Pattern::CurlyBorder => "curly_border.png",
            Pattern::TriangleBottom => "triangle_bottom.png",
            Pattern::TriangleTop => "triangle_top.png",
            Pattern::TrianglesBottom => "triangles_bottom.png",
            Pattern::TrianglesTop => "triangles_top.png",
            Pattern::DiagonalTopLeft => "diagonal_top_left.png",
            Pattern::DiagonalTopRight => "diagonal_top_right.png",
            Pattern::DiagonalBottomLeft=> "diagonal_bottom_left.png",
            Pattern::DiagonalBottomRight => "diagonal_bottom_right.png",
            Pattern::CircleMiddle => "circle_middle.png",
            Pattern::RhombusMiddle => "rhombus_middle.png",
            Pattern::HalfVertical => "half_vertical.png",
            Pattern::HalfVerticalMirror => "half_vertical_mirror.png",
            Pattern::HalfHorizontal => "half_horizontal.png",
            Pattern::HalfHorizontalMirror => "half_horizontal_mirror.png",
            Pattern::Creeper => "creeper.png",
            Pattern::Bricks => "bricks.png",
            Pattern::Gradient => "gradient.png",
            Pattern::GradientUp => "gradient_up.png",
            Pattern::Skull => "skull.png",
            Pattern::Flower => "flower.png",
            Pattern::Thing => "thing.png",
        }
        .to_string()
    }

    pub fn image(&self) -> DynamicImage {  
        let path = self.asset_name();
        let image = Asset::get(&path).unwrap();
        let mut reader = ImageReader::new(Cursor::new(image.data));
        reader.set_format(image::ImageFormat::Png);
        reader.decode().unwrap()
    }
}