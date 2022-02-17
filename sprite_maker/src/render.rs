use std::fs::{self, DirEntry};

use crate::{render_blend_file, render_model_sheet, Cfg};

/// Renders a single model's sheets
pub fn execute<'a>(input_path: &'a str, output_path: &'a str, cfg: &Cfg) {
    // Source all folders
    let folders: Vec<DirEntry> = fs::read_dir(input_path)
        .unwrap()
        .filter_map(|e| if e.is_ok() { Some(e.unwrap()) } else { None })
        .filter(|e| e.file_type().unwrap().is_dir())
        .collect();

    if folders.is_empty() {
        panic!("No nested folders found in input directory! Please ensure all .blend files are under subdirectories in the input directory.");
    }

    // Render sprites for each folder
    for folder in folders {
        render_folder(folder, output_path, cfg.clone())
    }
}

/// Renders the folder
fn render_folder<'a>(folder: DirEntry, output_path: &'a str, cfg: Cfg) {
    // Make temp directory for renders
    let temp_path = {
        let mut temp_path = folder.path().clone();
        temp_path.push("__tmp_renders");
        std::fs::create_dir_all(&temp_path).unwrap();

        temp_path
    };

    let model_name = folder
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or_default()
        .to_string();

    // Render all blend files
    let blend_files: Vec<DirEntry> = fs::read_dir(&folder.path())
        .unwrap()
        .filter_map(|e| if e.is_ok() { Some(e.unwrap()) } else { None })
        .filter(|e| e.path().to_str().unwrap_or_default().ends_with(".blend"))
        .collect();

    // Execute blender renders
    for blend_file in blend_files {
        render_blend_file::execute(blend_file, &model_name, &temp_path, cfg.clone());
    }

    // Make the sheet
    render_model_sheet::execute(
        &temp_path.to_str().unwrap_or_default(),
        output_path,
        &model_name,
        cfg,
    );

    // Clean temp folder
    match std::fs::remove_dir_all(&temp_path) {
        Ok(_) => {}
        Err(e) => println!(
            "Error cleaning temp path '{:?}', reason: '{:?}'.",
            temp_path, e
        ),
    }
}
