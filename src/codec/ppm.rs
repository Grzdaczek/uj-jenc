#![allow(dead_code)]

use std::io::{BufRead, Cursor, Read};

use crate::image::{Image, ImageBuffer};
use crate::color::Rgb8;

use super::{Decoder, Encoder};

pub struct Ppm;

impl Ppm {
    pub fn new() -> Self {
        Self
    }
}

impl Encoder<Rgb8> for Ppm {
    fn encode(&self, image: Image<Rgb8>) -> ImageBuffer {
        let mut data: Vec<u8> = Vec::new();

        let header = format!("P6\n{:?} {}\n255\n", image.width(), image.height());
        data.append(& mut header.into_bytes());

        image
            .data()
            .iter()
            .for_each(|p| {
                data.push(p.r);
                data.push(p.g);
                data.push(p.b);
            });

        ImageBuffer {data}
    }
}

impl Decoder<Rgb8> for Ppm {
    fn decode(&self, buffer: ImageBuffer) -> Image<Rgb8> {
        let mut cursor = Cursor::new(buffer.data);
        
        // read magic number
        let mut str = String::new();
        cursor.read_line(& mut str).unwrap();
        assert!(str.trim().eq("P6"));

        // read size
        let mut str = String::new();
        cursor.read_line(& mut str).unwrap();
        let mut parts = str
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap());
        
        let width = parts.next().unwrap();
        let height = parts.next().unwrap();

        // read bit depth number
        let mut str = String::new();
        cursor.read_line(& mut str).unwrap();
        assert!(str.trim().eq("255"));

        // read image data
        let mut data: Vec<Rgb8> = Vec::with_capacity(width*height);
        for _ in 0..(width*height) {
            let mut buf :[u8; 3] = [0; 3];
            cursor.read_exact(& mut buf).unwrap();
            data.push(Rgb8 {
                r: buf[0],
                g: buf[1],
                b: buf[2],
            });
        }

        Image::new(width, height, data)
    }
}