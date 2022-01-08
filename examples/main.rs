#![allow(dead_code)]
#![allow(unused)]

use std::fs;
use uj_jenc::codec::*;
use uj_jenc::color::*;
use uj_jenc::image::*;
fn main() {
    let rcr_codec = rcr::Rcr::new(10.0);
    let ppm_codec = ppm::Ppm::new();

    let img: Image<Lab8> = ImageBuffer::read("./examples/in_image.ppm")
        .unwrap()
        .decode(&ppm_codec)
        .into();

    let buf = img.encode(&rcr_codec);

    buf.write("./examples/tmp");

    let img: Image<Rgb8> = buf.decode(&rcr_codec).into();

    img.encode(&ppm_codec)
        .write("./examples/out_image.ppm")
        .unwrap();
}