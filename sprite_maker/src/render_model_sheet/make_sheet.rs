use crate::*;
use benchy::Benchy;
use image::{DynamicImage, GenericImage, GenericImageView};

/// Converts all animations into a sprite sheet
pub fn execute(mut animations: Vec<Render>) -> (internal_sheet::SpriteSheet, Vec<u8>) {
    Benchy::time("make_sheet");

    // Split out animations and pngs

    let offset_x = 0;
    let mut offset_y = 0;

    let (animations, pngs) = {
        let mut a = vec![];
        let mut p = vec![];

        while animations.is_empty() == false {
            let (mut animation, png) = animations.remove(0);

            // Note the offset for the animation
            animation.start_x_px = offset_x;
            animation.start_y_px = offset_y;

            // Update offsets for next image; only doing vertical since everything gets stacked vertically
            offset_y += png.height();

            a.push(animation);
            p.push(png);
        }

        (a, p)
    };

    let final_png = join_pngs_vertically(pngs).expect("Didn't get a sprite sheet!");

    let width_px = final_png.width();
    let height_px = final_png.height();

    let mut image_png_bytes: Vec<u8> = Vec::new();
    final_png
        .write_to(&mut image_png_bytes, image::ImageOutputFormat::Png)
        .unwrap();

    (
        internal_sheet::SpriteSheet::new(image_png_bytes.clone(), width_px, height_px, animations),
        image_png_bytes,
    )
}

fn join_pngs_vertically(pngs: Vec<DynamicImage>) -> Option<DynamicImage> {
    // Source dimensions
    let mut height = 0;
    let mut width = 0;

    for png in pngs.iter() {
        height += png.height();
        width = png.width().max(width);
    }

    if height == 0 || width == 0 {
        return None;
    }

    // Create new img
    let mut img = DynamicImage::new_rgba8(width, height);

    // Note the coordinates to copy to
    let copy_x = 0;
    let mut copy_y = 0;
    for png in pngs.iter() {
        img.copy_from(png, copy_x, copy_y).unwrap();

        // Increment the height for the next img
        copy_y += png.height();
    }

    Some(img)
}
