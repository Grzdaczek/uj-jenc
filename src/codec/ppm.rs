#![allow(dead_code)]

use std::io::{BufRead, Cursor, Read};

use crate::image::Image;
use crate::color::RgbU8;

pub use crate::codec::{Decode, Encode};

pub struct Ppm;

impl Ppm {
    pub fn new() -> Self {
        Self
    }
}

pub trait SomeTrait {
    fn do_something();
}

impl SomeTrait for Ppm {
    fn do_something() {
        println!("Something!");
    }
}

impl Encode<RgbU8> for Ppm {
    fn encode(&self, image: &Image<RgbU8>) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let header = format!("P6\n{:?} {}\n255\n", image.width(), image.height());
        buffer.append(& mut header.into_bytes());

        image
            .data()
            .iter()
            .for_each(|p| {
                buffer.push(p.r);
                buffer.push(p.g);
                buffer.push(p.b);
            });

        buffer
    }
}

impl Decode<RgbU8> for Ppm {
    fn decode(&self, buffer: &[u8]) -> Image<RgbU8> {
        let mut cursor = Cursor::new(buffer);
        
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
        let mut data: Vec<RgbU8> = Vec::with_capacity(width*height);
        for _ in 0..(width*height) {
            let mut buf :[u8; 3] = [0; 3];
            cursor.read_exact(& mut buf).unwrap();
            data.push(RgbU8 {
                r: buf[0],
                g: buf[1],
                b: buf[2],
            });
        }

        Image::new(width, height, data)
    }
}