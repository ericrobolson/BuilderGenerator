/// Top level spritesheet for a given sprite.
#[derive(Clone, Debug, PartialEq)]
pub struct SpriteSheet {
    pub image_png_bytes: Vec<u8>,
    pub width_px: u32,
    pub height_px: u32,
    pub animations: Vec<Animation>,
}

impl SpriteSheet {
    /// Creates a new sprite sheet.
    pub fn new(
        image_png_bytes: Vec<u8>,
        width_px: u32,
        height_px: u32,
        animations: Vec<Animation>,
    ) -> Self {
        Self {
            width_px,
            height_px,
            animations,
            image_png_bytes,
        }
    }
}

/// An animation in the sprite sheet
/// Positions are relative to the parent object.
#[derive(Clone, Debug, PartialEq)]
pub struct Animation {
    pub directional_animations: Vec<DirectionalAnimation>,
    pub name: String,
    pub start_x_px: u32,
    pub start_y_px: u32,
}

/// A directional animation in the sprite sheet.
/// Positions are relative to the parent object.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectionalAnimation {
    pub direction: u32,
    pub frames: Vec<Frame>,
    pub start_x_px: u32,
    pub start_y_px: u32,
}

/// An individual frame in the animation.
/// Positions are relative to the parent object.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub height_px: u32,
    pub offset_x_px: u32,
    pub offset_y_px: u32,
    pub start_x_px: u32,
    pub start_y_px: u32,
    pub width_px: u32,
}
