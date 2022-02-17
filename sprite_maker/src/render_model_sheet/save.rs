use crate::*;
use benchy::Benchy;
use sprite_sheet::*;
use std::fs::File;
use std::io::prelude::*;

pub fn execute<'a>(path: &'a str, file_name: &'a str, sprite_sheet: internal_sheet::SpriteSheet) {
    Benchy::time("save");

    let sprite_sheet = map_internal_to_external(sprite_sheet);
    let json = sprite_sheet.to_json().unwrap();
    let mut file = File::create(format!(
        "{path}{file_name}.json",
        path = path,
        file_name = file_name
    ))
    .unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn map_internal_to_external(sprite_sheet: internal_sheet::SpriteSheet) -> SpriteSheet {
    let image_png_bytes = &sprite_sheet.image_png_bytes;
    let width_px = sprite_sheet.width_px;
    let height_px = sprite_sheet.height_px;
    let animations = map_animations(&sprite_sheet);

    SpriteSheet::new(image_png_bytes, width_px, height_px, animations)
}

fn map_animations(sprite_sheet: &internal_sheet::SpriteSheet) -> Vec<Animation> {
    let ctx = Ctx {
        sheet_w_px_f32: sprite_sheet.width_px as f32,
        sheet_h_px_f32: sprite_sheet.height_px as f32,
    };

    let animations = sprite_sheet
        .animations
        .iter()
        .map(|a| {
            let name = a.name.clone();
            let start_x_px = a.start_x_px;
            let start_y_px = a.start_y_px;
            let start_x_normalized = start_x_px as f32 / sprite_sheet.width_px as f32;
            let start_y_normalized = start_y_px as f32 / sprite_sheet.height_px as f32;

            let directional_animations =
                map_directional_animations(&ctx, start_x_px, start_y_px, &a);

            Animation::new(
                directional_animations,
                name,
                start_x_px,
                start_y_px,
                start_x_normalized,
                start_y_normalized,
            )
        })
        .collect();
    animations
}

fn map_directional_animations(
    ctx: &Ctx,
    start_x_px: u32,
    start_y_px: u32,
    animation: &internal_sheet::Animation,
) -> Vec<DirectionalAnimation> {
    let start_x_px = start_x_px;
    let mut start_y_px = start_y_px;

    let directions = animation
        .directional_animations
        .iter()
        .enumerate()
        .map(|(direction, d)| {
            let d_start_x_px = start_x_px;
            let d_start_y_px = start_y_px;
            let d_start_x_normalized = d_start_x_px as f32 / ctx.sheet_w_px_f32;
            let d_start_y_normalized = d_start_y_px as f32 / ctx.sheet_h_px_f32;

            let frames = map_frames(ctx, d_start_x_px, d_start_y_px, &d.frames);

            let mut mapped_h_px = 0;

            for f in frames.iter() {
                mapped_h_px = mapped_h_px.max(f.height_px());
            }

            start_y_px += mapped_h_px;

            DirectionalAnimation::new(
                frames,
                direction,
                d_start_x_px,
                d_start_y_px,
                d_start_x_normalized,
                d_start_y_normalized,
            )
        })
        .collect();

    directions
}

struct Ctx {
    sheet_w_px_f32: f32,
    sheet_h_px_f32: f32,
}

fn map_frames(
    ctx: &Ctx,
    start_x_px: u32,
    start_y_px: u32,
    frames: &Vec<internal_sheet::Frame>,
) -> Vec<Frame> {
    let mut start_x_px = start_x_px;
    let start_y_normalized = start_y_px as f32 / ctx.sheet_h_px_f32;

    let frames = frames
        .iter()
        .map(|f| {
            let width_px = f.width_px;
            let height_px = f.height_px;
            let width_normalized = width_px as f32 / ctx.sheet_w_px_f32;
            let height_normalized = height_px as f32 / ctx.sheet_h_px_f32;
            let offset_x_px = f.offset_x_px;
            let offset_y_px = f.offset_y_px;
            let offset_x_normalized = offset_x_px as f32 / ctx.sheet_w_px_f32;
            let offset_y_normalized = offset_y_px as f32 / ctx.sheet_h_px_f32;
            let f_start_x_px = start_x_px;
            let f_start_y_px = start_y_px;
            let f_start_x_normalized = start_x_px as f32 / ctx.sheet_w_px_f32;
            let f_start_y_normalized = start_y_normalized;

            start_x_px += width_px;

            Frame::new(
                width_px,
                height_px,
                width_normalized,
                height_normalized,
                offset_x_px,
                offset_y_px,
                offset_x_normalized,
                offset_y_normalized,
                f_start_x_px,
                f_start_y_px,
                f_start_x_normalized,
                f_start_y_normalized,
            )
        })
        .collect();

    frames
}
