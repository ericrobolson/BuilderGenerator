use builder_sheet_rust::*;
use ggez::event;
use ggez::filesystem;
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::io::Read;
use std::path;

struct MainState {
    image: graphics::Image,
    sprite_sheet: SpriteSheet,
    animation: String,
    animation_idx: usize,
    direction: usize,
    frame: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        filesystem::print_all(ctx);

        let sprite_sheet = {
            let mut file = filesystem::open(ctx, "/renders/test_cube.json").unwrap();

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let json = std::str::from_utf8(&buffer).unwrap();

            SpriteSheet::from_json(&json).unwrap()
        };

        let animation = sprite_sheet.animations()[0].name().to_string();
        let direction = 0;
        let frame = 0;

        let image = graphics::Image::from_bytes(ctx, &sprite_sheet.png_bytes())?;

        let s = MainState {
            animation_idx: 0,
            sprite_sheet,
            image,
            animation,
            frame,
            direction,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        std::thread::sleep(std::time::Duration::from_millis(32));

        self.frame += 1;

        if let Some(animation) = self.sprite_sheet.animation(&self.animation) {
            // Increment direction
            if self.frame as usize >= animation.direction(self.direction).unwrap().frames().len() {
                self.frame = 0;
                self.direction += 1;
            }

            // Increment animation
            if self.direction as usize >= animation.directions().len() {
                self.direction = 0;
                self.animation_idx += 1;

                // Wrap animations
                if self.animation_idx >= self.sprite_sheet.animations().len() {
                    self.animation_idx = 0;
                }

                self.animation = self.sprite_sheet.animations()[self.animation_idx]
                    .name()
                    .to_string();
            }

            println!("dir: {:?}", self.direction);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let color = Color::from((255, 255, 255));
        let dest_point = glam::Vec2::new(0.0, 0.0);
        let param = {
            let mut param: graphics::DrawParam = (dest_point, 0.0, color).into();

            if let Some(animation) = self.sprite_sheet.animation(&self.animation) {
                let direction = animation.direction(self.direction).unwrap();
                let frame = direction.frame(self.frame).unwrap();

                // Do subsprite
                param = param.src(graphics::Rect::new(
                    frame.start_x_normalized() as f32,
                    frame.start_y_normalized() as f32,
                    frame.width_normalized() as f32,
                    frame.height_normalized() as f32,
                ));

                // Do offset
                let offset =
                    glam::Vec2::new(frame.offset_x_px() as f32, frame.offset_y_px() as f32);
                param = param.dest(offset);
            }

            param
        };

        graphics::draw(ctx, &self.image, param)?;

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("imageview", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
