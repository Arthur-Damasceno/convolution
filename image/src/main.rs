use std::{env, error::Error};

use image::Image;

fn main() -> Result<(), Box<dyn Error>> {
    let operation = env::var("OPERATION")
        .unwrap_or("L".into())
        .chars()
        .next()
        .expect("Invalid operation");
    let path_open = env::var("PATH_OPEN").unwrap_or("./image/data/cat.pgm".into());
    let path_save = env::var("PATH_SAVE").unwrap_or("image.pgm".into());
    let thresh = env::var("THRESH").unwrap_or("127".into()).parse()?;

    let image = Image::open(&path_open)?;

    let processed = match operation {
        'M' => image.mean_filter(),
        'L' => image.laplacian_filter(),
        'T' => image.threshold(thresh),
        'N' => image.normalize(),
        _ => panic!("Invalid operation"),
    };

    processed.save(&path_save)?;

    Ok(())
}
