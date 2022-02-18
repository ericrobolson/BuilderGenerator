mod internal_sheet;
mod render;
mod render_blend_file;
mod render_model_sheet;

use benchy::Benchy;
use image::DynamicImage;

pub type Render = (internal_sheet::Animation, DynamicImage);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cfg {
    pub output_render_pngs: bool,
    pub sprite_w_px: u32,
    pub sprite_h_px: u32,
    pub num_directions: u32,
}

fn main() {
    // Check if user wants help
    if std::env::args()
        .find(|f| f.to_lowercase().contains("help"))
        .is_some()
    {
        print_help();

        return;
    }

    let start = std::time::Instant::now();

    let input_path = std::env::args().nth(1).expect("no input path given");
    let output_path = std::env::args().nth(2).expect("no output path given");
    let sprite_w_px = std::env::args().nth(3).expect("no sprite width given");
    let sprite_h_px = std::env::args().nth(4).expect("no sprite height given");
    let num_directions = std::env::args()
        .nth(5)
        .expect("no number of directions given");

    let output_render_pngs = std::env::args().find(|f| f == "-o").is_some();

    // Convert things to proper types
    let sprite_w_px = sprite_w_px.parse::<u32>().unwrap();
    let sprite_h_px = sprite_h_px.parse::<u32>().unwrap();
    let num_directions = num_directions.parse::<u32>().unwrap();

    // Ensure output path ends with a "/"
    let output_path = if output_path.ends_with("/") {
        output_path.to_string()
    } else {
        format!("{}/", output_path)
    };

    // Build cfg
    let cfg = Cfg {
        output_render_pngs,
        sprite_w_px,
        sprite_h_px,
        num_directions,
    };

    // Clean output folder
    match std::fs::remove_dir_all(&output_path) {
        Ok(_) => {}
        Err(_) => {}
    }

    // Create output folders if they don't exist
    std::fs::create_dir_all(&output_path).unwrap();

    // Execute
    render::execute(&input_path, &output_path, &cfg);

    // Final benchmarks
    Benchy::save("benchmarks.json");
    println!("Took: {:?}", std::time::Instant::now() - start);
}

fn print_help() {
    println!("Welcome to the sprite_maker!");
    println!("Command line format:");
    println!("\tinput_path output_path sprite_width sprite_height num_directions -o?");
    println!();
    println!("Argument Descriptions");
    println!("\tinput_path");
    println!("\t\t-Required path for the Blender files to render.");
    println!("\t\t-Each subdirectory under the top folder should represent a single sheet.");
    println!("\t\t\t-Each Blender file under the sheet folder should represent an animation.");
    println!("\toutput_path");
    println!("\t\t-Required path to output the sprite sheet JSON objects.");
    println!("\tsprite_width");
    println!("\t\t-Required positive integer for each Blender render's width.");
    println!("\tsprite_height");
    println!("\t\t-Required positive integer for each Blender render's height.");
    println!("\tnum_directions");
    println!("\t\t-Required positive integer for the number of directions to render.");
    println!("\t-o");
    println!("\t\t-Outputs a PNG of the sprite sheet for debugging.");
    println!();
}
