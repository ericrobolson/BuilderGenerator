use serde::{Deserialize, Serialize};
pub use serde_json::Error;

/// A sprite sheet which contains many sprites in a single image.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpriteSheet {
    image_png_bytes_b64: String,
    width_px: u32,
    height_px: u32,
    animations: Vec<Animation>,
}

impl SpriteSheet {
    /// Creates a new sprite sheet.
    pub fn new(
        image_png_bytes: &Vec<u8>,
        width_px: u32,
        height_px: u32,
        animations: Vec<Animation>,
    ) -> Self {
        Self {
            width_px,
            height_px,
            animations,
            image_png_bytes_b64: Self::bytes_to_b64(image_png_bytes),
        }
    }

    /// Returns the total width of the sheet in pixels.
    pub fn width_px(&self) -> u32 {
        self.width_px
    }

    /// Returns the total height of the sheet in pixels.
    pub fn height_px(&self) -> u32 {
        self.height_px
    }

    /// Returns the animations for the sheet.
    pub fn animations(&self) -> &Vec<Animation> {
        &self.animations
    }

    /// Returns the given animation
    pub fn animation<'a>(&self, animation_name: &'a str) -> Option<&Animation> {
        for a in self.animations() {
            if a.name == animation_name {
                return Some(a);
            }
        }

        None
    }

    /// Attempts to create a spritesheet from some JSON
    pub fn from_json<'a>(json: &'a str) -> Result<Self, Error> {
        let v: Self = serde_json::from_str(json)?;
        Ok(v)
    }

    /// Decodes the given image as a string of bytes.
    pub fn png_bytes(&self) -> Vec<u8> {
        match base64::decode(&self.image_png_bytes_b64) {
            Ok(bytes) => bytes,
            Err(_) => vec![],
        }
    }

    /// Converts the sheet to JSON
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self)
    }

    /// Encodes some bytes to b64
    fn bytes_to_b64(bytes: &Vec<u8>) -> String {
        base64::encode(&bytes)
    }
}

/// An animation in the sprite sheet
/// Positions are relative to the parent object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    directional_animations: Vec<DirectionalAnimation>,
    name: String,
    start_x_px: u32,
    start_y_px: u32,
    start_x_normalized: f32,
    start_y_normalized: f32,
}

impl Animation {
    /// Creates a new animation.
    pub fn new(
        directional_animations: Vec<DirectionalAnimation>,
        name: String,
        start_x_px: u32,
        start_y_px: u32,
        start_x_normalized: f32,
        start_y_normalized: f32,
    ) -> Self {
        Self {
            directional_animations,
            name,
            start_x_px,
            start_y_px,
            start_x_normalized,
            start_y_normalized,
        }
    }

    /// Returns the name of the animation.
    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    /// Returns the directional animations for the sheet.
    pub fn directions(&self) -> &Vec<DirectionalAnimation> {
        &self.directional_animations
    }

    /// Returns the given directional animation.
    pub fn direction<'a>(&self, direction: usize) -> Option<&DirectionalAnimation> {
        if self.directional_animations.len() > direction {
            return Some(&self.directional_animations[direction]);
        }

        None
    }

    /// Returns the start X of this animation on the sprite sheet in pixels.
    pub fn start_x_px(&self) -> u32 {
        self.start_x_px
    }

    /// Returns the start Y of this animation on the sprite sheet in pixels.
    pub fn start_y_px(&self) -> u32 {
        self.start_y_px
    }

    /// Returns the start X of this animation on the sprite sheet in a normalized range of 0.0-1.0 with respect to the full sprite sheet.
    pub fn start_x_normalized(&self) -> f32 {
        self.start_x_normalized
    }

    /// Returns the start Y of this animation on the sprite sheet in a normalized range of 0.0-1.0 with respect to the full sprite sheet.
    pub fn start_y_normalized(&self) -> f32 {
        self.start_y_normalized
    }
}

/// A directional animation in the sprite sheet.
/// Positions are relative to the parent object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectionalAnimation {
    direction: usize,
    frames: Vec<Frame>,
    start_x_px: u32,
    start_y_px: u32,
    start_x_normalized: f32,
    start_y_normalized: f32,
}

impl DirectionalAnimation {
    /// Creates a new directional animation.
    pub fn new(
        frames: Vec<Frame>,
        direction: usize,
        start_x_px: u32,
        start_y_px: u32,
        start_x_normalized: f32,
        start_y_normalized: f32,
    ) -> Self {
        Self {
            frames,
            direction,
            start_x_px,
            start_y_px,
            start_x_normalized,
            start_y_normalized,
        }
    }

    /// Returns the name of the animation.
    pub fn direction(&self) -> usize {
        self.direction
    }

    /// Returns the frames for the direction.
    pub fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }

    /// Returns the given frame.
    pub fn frame<'a>(&self, frame: usize) -> Option<&Frame> {
        if self.frames.len() > frame {
            return Some(&self.frames[frame]);
        }

        None
    }

    /// Returns the start X of this animation on the sprite sheet in pixels.
    pub fn start_x_px(&self) -> u32 {
        self.start_x_px
    }

    /// Returns the start Y of this animation on the sprite sheet in pixels.
    pub fn start_y_px(&self) -> u32 {
        self.start_y_px
    }

    /// Returns the start X of this animation on the sprite sheet in a normalized range of 0.0-1.0 with respect to the full sprite sheet.
    pub fn start_x_normalized(&self) -> f32 {
        self.start_x_normalized
    }

    /// Returns the start Y of this animation on the sprite sheet in a normalized range of 0.0-1.0 with respect to the full sprite sheet.
    pub fn start_y_normalized(&self) -> f32 {
        self.start_y_normalized
    }
}

/// An individual frame in the animation.
/// Positions are relative to the parent object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    width_px: u32,
    height_px: u32,
    width_normalized: f32,
    height_normalized: f32,
    offset_x_px: u32,
    offset_y_px: u32,
    offset_x_normalized: f32,
    offset_y_normalized: f32,
    start_x_px: u32,
    start_y_px: u32,
    start_x_normalized: f32,
    start_y_normalized: f32,
}

impl Frame {
    /// Creates a new directional animation.
    pub fn new(
        width_px: u32,
        height_px: u32,
        width_normalized: f32,
        height_normalized: f32,
        offset_x_px: u32,
        offset_y_px: u32,
        offset_x_normalized: f32,
        offset_y_normalized: f32,
        start_x_px: u32,
        start_y_px: u32,
        start_x_normalized: f32,
        start_y_normalized: f32,
    ) -> Self {
        Self {
            width_px,
            height_px,
            width_normalized,
            height_normalized,
            offset_x_px,
            offset_y_px,
            offset_x_normalized,
            offset_y_normalized,
            start_x_px,
            start_y_px,
            start_x_normalized,
            start_y_normalized,
        }
    }

    /// Returns the width of the frame in pixels.
    pub fn width_px(&self) -> u32 {
        self.width_px
    }

    /// Returns the height of the frame in pixels.
    pub fn height_px(&self) -> u32 {
        self.height_px
    }

    /// Returns the width of the frame in a normalized 0.0-1.0 range with respect to the full sprite sheet.
    pub fn width_normalized(&self) -> f32 {
        self.width_normalized
    }

    /// Returns the height of the frame in a normalized 0.0-1.0 range with respect to the full sprite sheet.
    pub fn height_normalized(&self) -> f32 {
        self.height_normalized
    }

    /// Returns the X offset of the frame in pixels.
    pub fn offset_x_px(&self) -> u32 {
        self.offset_x_px
    }

    /// Returns the Y offset of the frame in pixels.
    pub fn offset_y_px(&self) -> u32 {
        self.offset_y_px
    }

    /// Returns the X offset of the frame in a normalized 0.0-1.0 range with respect to the full sprite sheet.
    pub fn offset_x_normalized(&self) -> f32 {
        self.offset_x_normalized
    }

    /// Returns the Y offset of the frame in a normalized 0.0-1.0 range with respect to the full sprite sheet.
    pub fn offset_y_normalized(&self) -> f32 {
        self.offset_y_normalized
    }

    /// Returns the start X of this frame on the sprite sheet in pixels.
    pub fn start_x_px(&self) -> u32 {
        self.start_x_px
    }

    /// Returns the start Y of this frame on the sprite sheet in pixels.
    pub fn start_y_px(&self) -> u32 {
        self.start_y_px
    }

    /// Returns the start X of this frame on the sprite sheet in a normalized range of 0.0-1.0 with respect to the full sprite sheet.
    pub fn start_x_normalized(&self) -> f32 {
        self.start_x_normalized
    }

    /// Returns the start Y of this frame on the sprite sheet in a normalized range of 0.0-1.0 with respect to the full sprite sheet.
    pub fn start_y_normalized(&self) -> f32 {
        self.start_y_normalized
    }
}
