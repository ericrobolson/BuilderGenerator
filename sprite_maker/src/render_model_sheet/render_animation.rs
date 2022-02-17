use super::{sorted_map::SortedMap, *};
use crate::internal_sheet::*;
use crate::Render;
use benchy::Benchy;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, GenericImageView};

pub fn execute(animation: String, mut imgs: Vec<ImgToRender>) -> Render {
    Benchy::time("render_animation");

    // Group directions
    let mut directions: SortedMap<u32, Vec<ImgToRender>> = SortedMap::new();
    while imgs.is_empty() == false {
        let img = imgs.remove(0);

        if let Some(direction) = directions.get_mut(&img.direction) {
            direction.push(img);
        } else {
            directions.insert(img.direction, vec![img]);
        }
    }

    // Render each direction
    let mut directional_renders: Vec<(DirectionalAnimation, DynamicImage)> = vec![];
    for (direction, imgs) in directions.iter() {
        let (img, frames) = render_frames(imgs);

        let direction = DirectionalAnimation {
            start_x_px: 0,
            start_y_px: 0,
            direction: *direction,
            frames,
        };

        directional_renders.push((direction, img));
    }

    // Join and return a single animation + render
    join_directional_renders(animation, directional_renders)
}

fn join_directional_renders(
    animation: String,
    mut directional_renders: Vec<(DirectionalAnimation, DynamicImage)>,
) -> (Animation, DynamicImage) {
    let mut final_image = {
        let mut width = 0;
        let mut height = 0;

        for (_, render) in directional_renders.iter() {
            width = width.max(render.width());
            height += render.height();
        }

        DynamicImage::new_rgba8(width, height)
    };

    let mut final_animation = Animation {
        start_x_px: 0,
        start_y_px: 0,
        name: animation,
        directional_animations: vec![],
    };

    let offset_x_px = 0;
    let mut offset_y_px = 0;
    while directional_renders.is_empty() == false {
        let (mut dir_animation, img) = directional_renders.remove(0);

        // Update offsets on animation
        dir_animation.start_x_px = offset_x_px;
        dir_animation.start_y_px = offset_y_px;

        // Copy image
        final_image
            .copy_from(&img, offset_x_px, offset_y_px)
            .unwrap();

        // Increment offsets
        offset_y_px += img.height();

        // Add to final animation
        final_animation.directional_animations.push(dir_animation);
    }

    (final_animation, final_image)
}

fn render_frames(frames_to_render: &Vec<ImgToRender>) -> (DynamicImage, Vec<Frame>) {
    let mut frames = vec![];
    let mut frame_renders = vec![];

    let mut height_px = 0;
    let mut width_px = 0;

    // Process each frame
    for img_to_render in frames_to_render.iter() {
        let (img, mut frame) = render_frame(img_to_render);

        // Update the start positions for the frame
        frame.start_x_px = width_px;
        frame.start_y_px = 0;

        // Increase final image height + width
        height_px = height_px.max(img.height());
        width_px += img.width();

        // Add to collections
        frames.push(frame);
        frame_renders.push(img);
    }

    // Join up images
    let mut img = DynamicImage::new_rgba8(width_px, height_px);
    let mut copy_x = 0;
    let copy_y = 0;
    for frame_render in frame_renders.iter() {
        img.copy_from(frame_render, copy_x, copy_y).unwrap();

        copy_x += frame_render.width();
    }

    // Return frames + final render
    (img, frames)
}

fn render_frame(image: &ImgToRender) -> (DynamicImage, Frame) {
    let mut img = ImageReader::open(&image.path).unwrap().decode().unwrap();

    // Get coordinates to crop
    let mut new_start_x = img.width();
    let mut new_start_y = img.height();
    let mut new_end_x = 0;
    let mut new_end_y = 0;

    for (x, y, rgba) in img.pixels() {
        // If alpha is not zero, update the coordinates
        if rgba[3] != 0 {
            new_start_x = x.min(new_start_x);
            new_start_y = y.min(new_start_y);

            new_end_x = x.max(new_end_x);
            new_end_y = y.max(new_end_y);
        }
    }

    // Add a 1px buffer to prevent jitter
    if new_start_x > 0 {
        new_start_x -= 1;
    }
    if new_start_y > 0 {
        new_start_y -= 1;
    }
    if new_end_x < img.width() {
        new_end_x += 1;
    }
    if new_end_y < img.height() {
        new_end_y += 1;
    }

    // Crop img
    let width_px = new_end_x - new_start_x;
    let height_px = new_end_y - new_start_y;

    let img = img.crop(new_start_x, new_start_y, width_px, height_px);

    // Keep track of the new offsets
    let offset_x_px = new_start_x;
    let offset_y_px = new_start_y;

    let frame = Frame {
        height_px,
        width_px,
        offset_x_px,
        offset_y_px,
        start_x_px: 0,
        start_y_px: 0,
    };

    (img, frame)
}
