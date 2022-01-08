#![allow(dead_code)]
#![allow(unused)]

use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Cursor;
use std::io::Seek;
use std::io::Write;
use uj_jenc::codec::*;
use uj_jenc::color::*;
use uj_jenc::image::*;

fn main() {
    let mut file = fs::read("./examples/in_image.ppm").unwrap();
    let mut rcr_data = Vec::new();
    let mut ppm_data = Vec::new();

    let img: Image<Lab8> = ppm::decode(&mut &file[..]).into();
    rcr::encode(&mut rcr_data, rcr::Settings::default(), &img);

    let img: Image<Rgb8> = rcr::decode(&mut &rcr_data[..], rcr::Settings::default()).into();
    ppm::encode(&mut ppm_data, img);

    fs::write("./examples/out_image.ppm", ppm_data).unwrap();
}