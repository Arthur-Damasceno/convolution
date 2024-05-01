use std::{env, io::Result};

use image::Image;

fn main() -> Result<()> {
    let path_open = env::var("PATH_OPEN").unwrap_or("./image/data/cat.pgm".into());
    let path_save = env::var("PATH_SAVE").unwrap_or("image.pgm".into());
    let treshold = env::var("TRESHOLD")
        .map(|value| value.parse().unwrap())
        .unwrap_or(127);

    let image = Image::open(&path_open)?;

    let treshold = image.treshold(treshold);

    treshold.save(&path_save)
}
