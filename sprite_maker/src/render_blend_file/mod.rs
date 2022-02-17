use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::{fs::DirEntry, path::PathBuf};

use crate::Cfg;

const SCRIPT_NAME: &'static str = "render.py";
const SCRIPT: &'static str = std::include_str!("render.py");

/// Renders the blender file
pub fn execute<'a>(blend_file: DirEntry, model_name: &'a str, temp_path: &PathBuf, cfg: Cfg) {
    //ex: blender -b test_cube\\"${FILE_NAME}".blend -P render.py

    let path = blend_file.path();
    let file_name = path.to_str().unwrap_or_default();
    let render_width = cfg.sprite_w_px;
    let render_height = cfg.sprite_h_px;

    // Write script to temp dir
    let mut script_path = temp_path.clone();
    script_path.push(SCRIPT_NAME);
    let mut file = File::create(&script_path).unwrap();
    file.write_all(SCRIPT.as_bytes()).unwrap();

    let command_output = Command::new("blender")
        .arg("-b")
        .arg(file_name)
        // Load a python script
        .arg("-P")
        .arg(script_path)
        // Add in some args for the Python script
        .arg("--")
        .arg(temp_path)
        .arg(model_name)
        .arg(render_width.to_string())
        .arg(render_height.to_string())
        .arg(cfg.num_directions.to_string())
        // Execute
        .output();

    match command_output {
        Ok(e) => {
            let output = std::str::from_utf8(&e.stdout).unwrap();

            println!("{}", output);
        }
        Err(e) => println!("Error rendering blend file {:?}: {:?}", blend_file, e),
    }
}
