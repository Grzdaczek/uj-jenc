#![allow(unused)]

use std::io::{Cursor, Read, Write};
use std::vec;

use crate::color::Lab8;
use crate::image::{Image, ImageBuffer};

use super::{Decoder, Encoder};

mod transform;

/// Raw cosine representation
pub struct Rcr {
    quality: u8,
    unit_size: usize,
    luma_table: [f32; 64],
    chroma_table: [f32; 64],
}

impl Rcr {
    pub fn new(quality :u8) -> Self {
        Self {
            quality: quality.clamp(0, 100),
            unit_size: 8,
            chroma_table: transform::DEFAULT_CHROMA_TABLE,
            luma_table: transform::DEFAULT_LUMA_TABLE,
        }
    }
}

impl Encoder<Lab8> for Rcr {
    fn encode(&self, image: Image<Lab8>) -> ImageBuffer {
        assert!(image.width() % 8 == 0);
        assert!(image.height() % 8 == 0);

        let w = image.width() / 8;
        let h = image.height() / 8;

        let mut data = Vec::new();

        data.write(&(image.width() as u16).to_be_bytes());
        data.write(&(image.height() as u16).to_be_bytes());

        for y in 0..h {
            for x in 0..w {
                let mut l_spacial = [0.0; 64];
                let mut a_spacial = [0.0; 64];
                let mut b_spacial = [0.0; 64];
        
                for i in 0..8 {
                    for j in 0..8 {
                        let index = i + (8 * x) + (8 * y + j) * image.width();
                        let color = image.data()[index];
                        l_spacial[i + 8 * j] = color.l as f32;
                        a_spacial[i + 8 * j] = color.a as f32 - 128.0;
                        b_spacial[i + 8 * j] = color.b as f32 - 128.0;
                    }
                }

                let mut l_frequency = [0.0; 64];
                let mut a_frequency = [0.0; 64];
                let mut b_frequency = [0.0; 64];

                transform::dct(&l_spacial, &mut l_frequency);
                transform::dct(&a_spacial, &mut a_frequency);
                transform::dct(&b_spacial, &mut b_frequency);

                let mut l_raw = [0; 64];
                let mut a_raw = [0; 64];
                let mut b_raw = [0; 64];

                transform::quant(&l_frequency, &mut l_raw, &self.luma_table, self.quality);
                transform::quant(&a_frequency, &mut a_raw, &self.chroma_table, self.quality);
                transform::quant(&b_frequency, &mut b_raw, &self.chroma_table, self.quality);

                data.extend_from_slice(&l_raw);
                data.extend_from_slice(&a_raw);
                data.extend_from_slice(&b_raw);
            }
        }
        
        ImageBuffer { data }
    }
}

impl Decoder<Lab8> for Rcr {
    fn decode(&self, buffer: ImageBuffer) -> Image<Lab8> {
        let mut cursor = Cursor::new(buffer.data);
        let mut bytes = [0; 2];

        cursor.read_exact(&mut bytes);
        let width: u16 = u16::from_be_bytes(bytes);

        cursor.read_exact(&mut bytes);
        let height: u16 = u16::from_be_bytes(bytes);
        
        assert!(width % 8 == 0);
        assert!(height % 8 == 0);

        let w = (width / 8) as usize;
        let h = (height / 8) as usize;

        let mut data: Vec<Lab8> = vec![Lab8::default(); width as usize * height as usize];
        
        println!("width: {}", width);
        println!("height: {}", height);

        for y in 0..h {
            for x in 0..w {
                let mut l_raw = [0; 64];
                let mut a_raw = [0; 64];
                let mut b_raw = [0; 64];

                cursor.read_exact(&mut l_raw);
                cursor.read_exact(&mut a_raw);
                cursor.read_exact(&mut b_raw);

                let mut l_frequency = [0.0; 64];
                let mut a_frequency = [0.0; 64];
                let mut b_frequency = [0.0; 64];

                transform::inv_quant(&l_raw, &mut l_frequency, &self.luma_table, self.quality);
                transform::inv_quant(&a_raw, &mut a_frequency, &self.chroma_table, self.quality);
                transform::inv_quant(&b_raw, &mut b_frequency, &self.chroma_table, self.quality);

                let mut l_spacial = [0.0; 64];
                let mut a_spacial = [0.0; 64];
                let mut b_spacial = [0.0; 64];

                transform::inv_dct(&l_frequency, &mut l_spacial);
                transform::inv_dct(&a_frequency, &mut a_spacial);
                transform::inv_dct(&b_frequency, &mut b_spacial);

                for j in 0..8 {
                    for i in 0..8 {
                        let index = i + (8 * x) + (8 * y + j) * width as usize;
                        data[index].l = l_spacial[i + 8 * j] as u8;
                        data[index].a = (a_spacial[i + 8 * j] + 128.0) as u8;
                        data[index].b = (b_spacial[i + 8 * j] + 128.0) as u8;
                    }
                }
            }
        }
        
        Image::new(width as usize, height as usize, data)
    }
}
