#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use uj_jenc::codec::*;
use uj_jenc::color::*;
use uj_jenc::image::*;
fn main() {
    let buff = fs::read("./examples/in_image.ppm").unwrap();

    let hjpg_codec = rcr::Rcr::new();
    let ppm_codec = ppm::Ppm::new();
    
    let rgb = ppm_codec.decode(&buff);
    let ycbcr: Image<YcbcrU8> = Image::from(&rgb);
    let rgb: Image::<RgbU8> = Image::from(&ycbcr);

    let buff = ppm_codec.encode(&rgb);
    fs::write("./examples/out_image.ppm", buff).unwrap();
}