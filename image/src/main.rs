use std::{env, io::Result};

use image::Image;

fn main() -> Result<()> {
    let path_open = env::var("PATH_OPEN").unwrap_or("./image/data/cat.pgm".into());
    let path_save = env::var("PATH_SAVE").unwrap_or("image.pgm".into());

    let image = Image::open(&path_open)?;

    let blur = image.laplacian_filter();

    blur.save(&path_save)
}
