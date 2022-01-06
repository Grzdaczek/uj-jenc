#![allow(unused)]

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

type Dct8Buffer = [[f32; 8]; 8];

lazy_static! {
    /// inverse of sqrt(2)
    static ref DCT_SQRT2_INV: f32 = {
        1_f32 / 2_f32.sqrt()
    };
    
    /// DCT cosine multipliers
    static ref DCT_COS_MUL: Dct8Buffer = {
        let mut mul: Dct8Buffer = [[0_f32; 8]; 8];
        for k in 0..8 {
            for n in 0..8 {
                let kf = k as f32;
                let nf = n as f32;
                mul[k][n] = (
                    (std::f32::consts::PI / 8.0)
                    * (0.5 + kf)
                    * (0.5 + nf)
                ).cos() * 0.5;
            }
        }

        mul
    };
}

fn dct(in_buf: &Dct8Buffer, out_buf: &mut Dct8Buffer) {
    let mut mid_buf: Dct8Buffer = [[0.0; 8]; 8];
    
    for k in 0..8 {
        for y in 0..8 {
            mid_buf[y][k] = (0..8)
                .map(|n| in_buf[y][n] * DCT_COS_MUL[k][n])
                .sum();
        }
    }

    for k in 0..8 {
        for x in 0..8 {
            out_buf[k][x] = (0..8)
                .map(|n| mid_buf[n][x] * DCT_COS_MUL[k][n])
                .sum();
        }
    }
}

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


#[cfg(test)]
mod tests {
    use super::dct;

    #[test]
    fn dct_is_equal_to_its_inverese() {
        let spacial = [
            [16.0, 11.0, 10.0, 16.0,  24.0,  40.0,  51.0,  61.0],
            [12.0, 12.0, 14.0, 19.0,  26.0,  58.0,  60.0,  55.0],
            [14.0, 13.0, 16.0, 24.0,  40.0,  57.0,  69.0,  56.0],
            [14.0, 17.0, 22.0, 29.0,  51.0,  87.0,  80.0,  62.0],
            [18.0, 22.0, 37.0, 56.0,  68.0, 109.0, 103.0,  77.0],
            [24.0, 35.0, 55.0, 64.0,  81.0, 104.0, 113.0,  92.0],
            [49.0, 64.0, 78.0, 87.0, 103.0, 121.0, 120.0, 101.0],
            [72.0, 92.0, 95.0, 98.0, 112.0, 100.0, 103.0,  99.0]
        ];

        let mut frequency = [[0.0; 8];8 ];
        dct(&spacial, &mut frequency);

        let mut new_spacial = [[0.0; 8];8 ];
        dct(&frequency, &mut new_spacial);

        for x in 0..8 {
            for y in 0..8 {
                assert_eq!(spacial[x][y].round(), new_spacial[x][y].round());
            }
        }
    }    


}