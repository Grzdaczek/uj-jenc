use std::io::{BufRead, BufReader, Read, Write};

use crate::image::Image;
use crate::color::Rgb8;

pub struct Ppm;

impl Ppm {
    pub fn new() -> Self {
        Self
    }
}

pub fn encode<T>(mut output: T, image: &Image<Rgb8>)
where T: Write
{
    let header = format!("P6\n{:?} {}\n255\n", image.width(), image.height());
    output.write(&header.into_bytes()).unwrap();

    image
        .data()
        .iter()
        .for_each(|p| {
            output.write(&[p.r, p.g, p.b]).unwrap();
        });
}


pub fn decode<T>(input: T) -> Image<Rgb8>
where T: Read
{
    let mut input = BufReader::new(input);

    // read magic number
    let mut str = String::new();
    input.read_line(& mut str).unwrap();
    assert!(str.trim().eq("P6"));

    // read size
    let mut str = String::new();
    input.read_line(& mut str).unwrap();
    let mut parts = str
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    
    let width = parts.next().unwrap();
    let height = parts.next().unwrap();

    // read bit depth number
    let mut str = String::new();
    input.read_line(& mut str).unwrap();
    assert!(str.trim().eq("255"));

    // read image data
    let mut data: Vec<Rgb8> = Vec::with_capacity(width*height);
    for _ in 0..(width*height) {
        let mut buf :[u8; 3] = [0; 3];
        input.read_exact(& mut buf).unwrap();
        data.push(Rgb8 {
            r: buf[0],
            g: buf[1],
            b: buf[2],
        });
    }

    Image::new(width, height, data)
}