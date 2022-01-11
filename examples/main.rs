use std::fs;
use std::io::Result;
use pixlib::codec::*;

fn main() -> Result<()> {
    let file = fs::File::open("./examples/in_1024x1280.ppm")?;
    let mut rcr_data = Vec::new();
    let mut ppm_data = Vec::new();

    let img = ppm::decode(file);
    rcr::encode(&mut rcr_data, rcr::Settings::quality(7), &img.into())?;

    let img = rcr::decode(&mut &rcr_data[..])?;
    ppm::encode(&mut ppm_data, &img.into());

    fs::write("./examples/tmp/out_image.ppm", &ppm_data)?;

    Ok(())
}