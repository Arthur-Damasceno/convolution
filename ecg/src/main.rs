use std::{io::Result, env};

use ecg::ECGData;

fn main() -> Result<()> {
    let path_read = env::var("PATH_READ").unwrap_or("data/sample_ecg.txt".into());
    let path_save = env::var("PATH_SAVE").unwrap_or("ecg_mean.txt".into());

    let data = ECGData::read(&path_read)?;

    let mean = data.mean_filter();

    mean.save(&path_save)?;

    Ok(())
}
