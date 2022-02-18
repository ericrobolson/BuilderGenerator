mod make_sheet;
mod render_animation;
mod render_animations;
mod save;
mod sorted_map;
mod source_imgs;

use source_imgs::ImgToRender;

use crate::Cfg;

/// Renders the sprite sheet
pub fn execute<'a>(input_path: &'a str, output_path: &'a str, model_name: &'a str, cfg: Cfg) {
    let source_images = source_imgs::execute(input_path);
    let rendered_animations = render_animations::execute(source_images);
    let (spritesheet, image_png_bytes) = make_sheet::execute(rendered_animations);

    // Debug final output
    if cfg.output_render_pngs {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::create(format!("{}{}.png", output_path, model_name)).unwrap();
        file.write_all(&image_png_bytes).unwrap();
    }

    save::execute(output_path, model_name, spritesheet);
}
