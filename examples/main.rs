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

    let img = ppm::decode(&mut &file[..]);
    rcr::encode(&mut rcr_data, rcr::Settings::quality(1), &img.into());

    let img = rcr::decode(&mut &rcr_data[..], rcr::Settings::quality(1));
    ppm::encode(&mut ppm_data, &img.into());

    fs::write("./examples/out_image.ppm", ppm_data).unwrap();
}