#![allow(unused)]

use std::os::unix::prelude::MetadataExt;
use std::usize;

use crate::codec::Encode;
use crate::color::Lab8;
use crate::image::{self, Image, ImageBuffer};

use super::Decode;

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
    luma_table: [u8; 64],
    chroma_table: [u8; 64],
}

impl Rcr {
    pub fn new(quality :u8) -> Self {
        Self {
            quality: quality.clamp(0, 100),
            chroma_table: DEFAULT_CHROMA_TABLE,
            luma_table: DEFAULT_LUMA_TABLE,
        }
    }

    pub fn with_tables(quality :u8, chroma_table: [u8; 64], luma_table: [u8; 64]) -> Self {
        Self {
            quality: quality.clamp(0, 100),
            chroma_table,
            luma_table,
        }
    }
}

impl<T> Encode<T> for Rcr where T : Clone + core::fmt::Debug {
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

impl<T> Image<T> {
    fn into_unit_iter(&self, size: usize) -> ImageUnitIterator<T> {
        if self.width() % size != 0 || self.height() % size != 0 {
            // TODO: Padding not supported
            panic!("Image padding not supported, dimentions must be multiple of unit size.")
        }

        ImageUnitIterator {
            image: self,
            size,
            x: 0,
            y: 0,
        }
    }
}

pub struct ImageUnitIterator<'a, T> {
    image: &'a Image<T>,
    x: usize,
    y: usize,
    size: usize,
}

impl<'a, T> Iterator for ImageUnitIterator<'a, T> where T : Clone {
    type Item = Unit<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let w = self.image.width();
        let h = self.image.height();
        let x = self.x;
        let y = self.y;
        let size = self.size;

        self.x += 1;

        if self.x * size == w {
            self.x = 0;
            self.y += 1;
        }
        
        if self.y * size == h {
            return None;
        }
        
        let data = (0..self.size)
            .map(|i| {
                let begin = (y * size) + (i * self.image.width());
                return &self.image.data()[begin..begin + size];
            })
            .fold(Vec::<T>::with_capacity(size.pow(2)), |mut v, slice| {
                v.extend_from_slice(slice);
                return v;
            });

        Some(Self::Item { data, size })
    }
}

pub struct Unit<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> Unit<T> {
    fn new(data: Vec<T>, size: usize) -> Self {
        if data.len() != size.pow(2) {
            panic!("Invalid size");
        }

        Self { data, size }
    }

    fn at(&self, x: usize, y: usize) -> &T {
        &self.data[self.size * y + x]
    }
}
