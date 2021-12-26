#![allow(unused)]

use crate::codec::Encode;
use crate::color::YcbcrU8;
use crate::image::{Image, ImageBuffer};

use super::Decode;

/// Raw cosine representation
pub struct Rcr {
    quality: u8,
}

impl Rcr {
    pub fn new(quality :u8) -> Self {
        Self {
            quality: quality.clamp(0, 100),
        }
    }
}

const LUMA_QUANTIZATION_TABLE: [u8; 64] =  [
    6,  11, 10, 16, 24,  40,  51,  61,
    12, 12, 14, 19, 26,  58,  60,  55,
    14, 13, 16, 24, 40,  57,  69,  56,
    14, 17, 22, 29, 51,  87,  80,  62,
    18, 22, 37, 56, 68,  109, 103, 77,
    24, 35, 55, 64, 81,  104, 113, 92,
    49, 64, 78, 87, 103, 121, 120, 101,
    72, 92, 95, 98, 112, 100, 103, 99,
];

const CHROMA_QUANTIZATION_TABLE: [u8; 64] =  [
    17, 18, 24, 47, 99, 99, 99, 99,
    18, 21, 26, 66, 99, 99, 99, 99,
    24, 26, 56, 99, 99, 99, 99, 99,
    47, 66, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99,
];

impl<T> Encode<T> for Rcr {
    fn encode(&self, image: crate::image::Image<T>) -> ImageBuffer {
        ImageBuffer {
            data: Vec::new()
        }
    }
}

impl<T> Decode<T> for Rcr {
    fn decode(&self, buffer: ImageBuffer) -> Image<T> {
        Image::new(0, 0, Vec::new())
    }
}

