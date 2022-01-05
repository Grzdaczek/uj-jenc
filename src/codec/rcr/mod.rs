#![allow(unused)]

use std::f32::consts::PI;

use crate::codec::Encoder;
use crate::color::Lab8;
use crate::image::{Image, ImageBuffer};

use super::Decoder;

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

    fn dct(&self, u: Unit<Lab8>) -> Vec<u8> {
        assert!(u.size == u.size);
        let mut buffer = Vec::new();
        let size = self.unit_size;

        for y in 0..size {
            for x in 0..size {
                let mut l_sum = 0.0;
                let mut a_sum = 0.0;
                let mut b_sum = 0.0;

                let p = if x == 0 { 1.0 / (2.0_f32).sqrt() } else { 1.0 };
                let q = if x == 0 { 1.0 / (2.0_f32).sqrt() } else { 1.0 };

                for i in 0..size {
                    for j in 0..size {
                        let mul = 
                            ((PI / size as f32) * (i as f32 + 0.5) * x as f32).cos() * 
                            ((PI / size as f32) * (j as f32 + 0.5) * y as f32).cos();

                        l_sum += u.at(i, j).l as f32 * mul;
                        a_sum += (u.at(i, j).a as f32 - 128.0) * mul;
                        b_sum += (u.at(i, j).b as f32 - 128.0) * mul;
                    }
                }

                // let luma_q = (self.luma_table[y * size + x]) as f32 * (1.0 / self.quality as f32);
                // let chroma_q = (self.chroma_table[y * size + x]) as f32 * (1.0 / self.quality as f32);

                let luma_q = 1.0;
                let chroma_q = 1.0;

                buffer.push((p * q * l_sum * 0.25 / luma_q).round() as u8);
                buffer.push((p * q * a_sum * 0.25 / chroma_q).round() as u8);
                buffer.push((p * q * b_sum * 0.25 / chroma_q).round() as u8);
            }
        }

        buffer
    }

    // fn inverse_dct(&self, buffer: Vec<u8>) -> Unit<Lab8> {
    //     let mut data = Vec::new();
    //     let size = self.unit_size;
        
    //     for y in 0..size {
    //         for x in 0..size {

    //             for i in 0..size {
    //                 for j in 0..size {
    //                 }
    //             }
    //         }
    //     }

    //     return Unit::new(data, size);
    // }
}

impl Encoder<Lab8> for Rcr {
    fn encode(&self, image: Image<Lab8>) -> ImageBuffer {
        let data = image
            .into_unit_iter(8)
            .map(|u| self.dct(u))
            .map(Vec::into_iter)
            .flatten()
            .collect();

        ImageBuffer { data }
    }
}

impl Decoder<Lab8> for Rcr {
    fn decode(&self, buffer: ImageBuffer) -> Image<Lab8> {
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
        
        if y * size == h {
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

        self.x += 1;

        if self.x * size == w {
            self.x = 0;
            self.y += 1;
        }

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
