use std::fs;
use uj_jenc::codec::*;

fn main() {
    let file = fs::read("./examples/in_image.ppm").unwrap();
    let mut rcr_data = Vec::new();
    let mut ppm_data = Vec::new();

    let img = ppm::decode(&mut &file[..]);
    rcr::encode(&mut rcr_data, rcr::Settings::quality(10), &img.into());

    let img = rcr::decode(&mut &rcr_data[..]);
    ppm::encode(&mut ppm_data, &img.into());

    fs::write("./examples/tmp/out_image.ppm", ppm_data).unwrap();
}