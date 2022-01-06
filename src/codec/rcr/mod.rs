#![allow(unused)]

use crate::color::Lab8;
use crate::image::{Image, ImageBuffer};

use super::{Decoder, Encoder};

mod dct;

const DEFAULT_LUMA_TABLE: [u8; 64] =  [
    6,   11,  10,  16,  24,  40,  51,  61,
    12,  12,  14,  19,  26,  58,  60,  55,
    14,  13,  16,  24,  40,  57,  69,  56,
    14,  17,  22,  29,  51,  87,  80,  62,
    18,  22,  37,  56,  68,  109, 103, 77,
    24,  35,  55,  64,  81,  104, 113, 92,
    49,  64,  78,  87,  103, 121, 120, 101,
    72,  92,  95,  98,  112, 100, 103, 99,
];

const DEFAULT_CHROMA_TABLE: [u8; 64] =  [
    17,  18,  24,  47,  99,  99,  99,  99,
    18,  21,  26,  66,  99,  99,  99,  99,
    24,  26,  56,  99,  99,  99,  99,  99,
    47,  66,  99,  99,  99,  99,  99,  99,
    99,  99,  99,  99,  99,  99,  99,  99,
    99,  99,  99,  99,  99,  99,  99,  99,
    99,  99,  99,  99,  99,  99,  99,  99,
    99,  99,  99,  99,  99,  99,  99,  99,
];

/// Raw cosine representation
pub struct Rcr {
    quality: u8,
    unit_size: usize,
    luma_table: [u8; 64],
    chroma_table: [u8; 64],
}

impl Rcr {
    pub fn new(quality :u8) -> Self {
        Self {
            quality: quality.clamp(0, 100),
            unit_size: 8,
            chroma_table: DEFAULT_CHROMA_TABLE,
            luma_table: DEFAULT_LUMA_TABLE,
        }
    }
}

impl Encoder<Lab8> for Rcr {
    fn encode(&self, image: Image<Lab8>) -> ImageBuffer {
        ImageBuffer { data: Vec::new() }
    }
}

impl Decoder<Lab8> for Rcr {
    fn decode(&self, buffer: ImageBuffer) -> Image<Lab8> {
        Image::new(0, 0, Vec::new())
    }
}
