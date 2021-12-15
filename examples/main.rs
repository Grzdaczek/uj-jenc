#![allow(dead_code)]
#![allow(unused_variables)]

use uj_jenc::pixel::*;

fn main() {
    // let file_name = "./image.ppm";

    let img_1 = Pixelmap::<Rgb>::new(8, 8);
    let img_2 = img_1.into_pixelmap::<Ycbcr>();

    // println!("{:?}", img_1);
    // println!("{:?}", img_2);
}