use std::io::Cursor;

use image::{io::Reader as ImageReader, DynamicImage};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "patterns/"]
struct Asset;

#[derive(Debug)]
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
    DiagonalLeft,
    DiagonalRight,
    DiagonalLeftMirror,
    DiagonalRightMirror,
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
            Pattern::DiagonalLeft => "diagonal_left.png",
            Pattern::DiagonalRight => "diagonal_right.png",
            Pattern::DiagonalLeftMirror => "diagonal_left_mirror.png",
            Pattern::DiagonalRightMirror => "diagonal_right_mirror.png",
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