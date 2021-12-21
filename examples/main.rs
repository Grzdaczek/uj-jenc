#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use uj_jenc::codec::*;
fn main() {
    let buff = fs::read("/home/grzdaczek/repos/uj-jenc/examples/in_image.ppm").unwrap();

    // let hjpg = hjpg::Hjpg::new({});
    let ppm_codec = ppm::Ppm::new();
    
    let mut  img = ppm_codec.decode(&buff);

    img.data_mut()
        .iter_mut()
        .for_each(|p| {
            p.r = 0xFF;
        });

    let buff = ppm_codec.encode(&img);

    // println!("{:?}", img.data()[0]);
    
    // let buff = hjpg.encode(img);
    // let img = hjpg.decode(buff);

    // let buff = ppm.encode(buff);

    fs::write("./out_image.ppm", buff).unwrap();
}