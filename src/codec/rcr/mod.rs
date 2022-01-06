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
}

// fn dct_1d_8(spacial: &[u8; 8], frequency: &mut [i16; 8]) {
//     for k in 0..8 {
//         frequency[k] = spacial
//             .iter()
//             .enumerate()
//             .map(|(n, &xn)| xn as f32 * ( PI / 8.0 * k as f32 * (0.5 + n as f32)).cos() )
//             .sum::<f32>() as i16;
//         ;
//     }
// }

fn dct(spacial: &[[u8; 8]; 8], frequency: &mut [[i16; 8]; 8]) {
    let mut buffer = [[0; 8]; 8];
    
    for y in 0..8 {
        for k in 0..8 {
            let mut sum = 0.0;
            for n in 0..8 {
                sum += spacial[y][n] as f32 * ( PI / 8.0 * k as f32 * (0.5 + n as f32)).cos()
            }
            buffer[y][k] = match k {
                0 => (sum / (2.0 * 2_f32.sqrt())) as i16,
                _ => (sum / 2.0) as i16,
            }
        }
    }

    for x in 0..8 {
        for k in 0..8 {
            let mut sum = 0.0;
            for n in 0..8 {
                sum += buffer[n][x] as f32 * ( PI / 8.0 * k as f32 * (0.5 + n as f32)).cos()
            }
            frequency[k][x] = match k {
                0 => (sum / (2.0 * 2_f32.sqrt())) as i16,
                _ => (sum / 2.0) as i16,
            }
        }
    }
}

impl Encoder<Lab8> for Rcr {
    fn encode(&self, image: Image<Lab8>) -> ImageBuffer {
        let mut data = Vec::new();

        image
            .into_unit_iter(8)
            .for_each(|u| {
                let mut l = [[0_u8; 8]; 8];
                let mut a = [[0_u8; 8]; 8];
                let mut b = [[0_u8; 8]; 8];
                
                for x in 0..8 {
                    for y in 0..8 {
                        let c = u.at(x, y);
                        l[x][y] = c.l;
                        a[x][y] = c.a;
                        b[x][y] = c.b;
                    }
                }

                let mut coeff = [[0; 8];8 ];
                dct(&l, &mut coeff);
                data.push(coeff);

                let mut coeff = [[0; 8];8 ];
                dct(&a, &mut coeff);
                data.push(coeff);

                let mut coeff = [[0; 8];8 ];
                dct(&b, &mut coeff);
                data.push(coeff);
            });

        println!("{:?}", data);
            
        // let x = [
        //     [16, 11, 10, 16, 24, 40, 51, 61],
        //     [12, 12, 14, 19, 26, 58, 60, 55],
        //     [14, 13, 16, 24, 40, 57, 69, 56],
        //     [14, 17, 22, 29, 51, 87, 80, 62],
        //     [18, 22, 37, 56, 68, 109, 103, 77],
        //     [24, 35, 55, 64, 81, 104, 113, 92],
        //     [49, 64, 78, 87, 103, 121, 120, 101],
        //     [72, 92, 95, 98, 112, 100, 103, 99]
        // ];

        // let mut y = [[0; 8];8 ];
        // dct(&x, &mut y);
        // y.iter().for_each(|yy| println!("{:?}", yy));

        ImageBuffer { data: Vec::new() }
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
    pub data: Vec<T>,
    pub size: usize,
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
