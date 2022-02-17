use benchy::Benchy;
use walkdir::WalkDir;

use super::sorted_map::SortedMap;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ImgToRender {
    pub animation: String,
    pub direction: u32,
    pub frame: u32,
    pub path: String,
}

impl ImgToRender {
    pub fn new(entry: &walkdir::DirEntry) -> Option<Self> {
        let extension = ".png";

        let file_name = String::from(entry.file_name().to_string_lossy());
        if !file_name.ends_with(extension) {
            return None;
        }

        let parsed_name = file_name.replace(extension, "");

        // Use '_ESCAPED' so we aren't mucking with randomly names sprites and we can consistently break where
        // it is wanted.
        let split: Vec<String> = parsed_name
            .split("_ESCAPED")
            .map(|s| s.to_string())
            .collect();

        if split.len() != 3 {
            return None;
        }

        const ANIMATION_IDX: usize = 0;
        const DIRECTION_IDX: usize = 1;
        const FRAME_IDX: usize = 2;

        let animation = split[ANIMATION_IDX].clone();
        let direction = if let Ok(f) = split[DIRECTION_IDX].parse::<u32>() {
            f
        } else {
            return None;
        };
        let frame = if let Ok(f) = split[FRAME_IDX].parse::<u32>() {
            f
        } else {
            return None;
        };

        let path = entry.path().to_str().unwrap_or_default().to_string();

        Some(Self {
            animation,
            direction,
            frame,
            path,
        })
    }
}

/// Sources images for a single spritesheet.
pub fn execute<'a>(path: &'a str) -> SortedMap<String, Vec<ImgToRender>> {
    Benchy::time("source_imgs");

    // Make a list of images to render
    let mut imgs: SortedMap<String, Vec<ImgToRender>> = SortedMap::new();

    // Traverse all files in directory
    // batching images by animations
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        if let Some(img) = ImgToRender::new(&entry) {
            if let Some(directions) = imgs.get_mut(&img.animation) {
                directions.push(img);
            } else {
                imgs.insert(img.animation.clone(), vec![img]);
            }
        }
    }

    // Sort it all so that it's consistently ordered.
    // Loading the files isn't always deterministic.
    for (_animation, imgs) in imgs.iter_mut() {
        imgs.sort();
    }

    imgs
}
