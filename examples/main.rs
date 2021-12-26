#![allow(dead_code)]
#![allow(unused)]

use std::fs;
use uj_jenc::codec::*;
use uj_jenc::color::*;
use uj_jenc::image::*;
fn main() {
    let rcr_codec = rcr::Rcr::new(50);
    let ppm_codec = ppm::Ppm::new();

    ImageBuffer::read("./examples/in_image.ppm")
        .unwrap()
        .decode(&ppm_codec)
        .encode(&rcr_codec)
        .decode(&rcr_codec)
        .encode(&ppm_codec)
        .write("./examples/out_image.ppm")
        .unwrap();
}