use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use crate::color::Lab8;
use crate::image::Image;

use transform::*;

mod transform;

pub const DEFAULT_LUMA_TABLE: [i8; 64] =  [
     6,  11,  10,  16,  24,  40,  51,  61,
    12,  12,  14,  19,  26,  58,  60,  55,
    14,  13,  16,  24,  40,  57,  69,  56,
    14,  17,  22,  29,  51,  87,  80,  62,
    18,  22,  37,  56,  68, 109, 103,  77,
    24,  35,  55,  64,  81, 104, 113,  92,
    49,  64,  78,  87, 103, 121, 120, 101,
    72,  92,  95,  98, 112, 100, 103,   9,
];

pub const DEFAULT_CHROMA_TABLE: [i8; 64] =  [
    17,  18,  24,  47,  99,  99,  99,  99,
    18,  21,  26,  66,  99,  99,  99,  99,
    24,  26,  56,  99,  99,  99,  99,  99,
    47,  66,  99,  99,  99,  99,  99,  99,
    99,  99,  99,  99,  99,  99,  99,  99,
    99,  99,  99,  99,  99,  99,  99,  99,
    99,  99,  99,  99,  99,  99,  99,  99,
    99,  99,  99,  99,  99,  99,  99,  99,
];

pub struct Settings {
    luma_table: [i8; 64],
    chroma_table: [i8; 64],
}

impl Settings {
    pub fn tabels(luma_table: [i8; 64], chroma_table: [i8; 64]) -> Self {
        Self { luma_table, chroma_table }
    }

    // TODO: implement quaity setting
    pub fn quality(_quality: i8) -> Self {
        let q = |x: i8| {
            1
        };

        Self {
            luma_table: DEFAULT_LUMA_TABLE.map(q),
            chroma_table: DEFAULT_CHROMA_TABLE.map(q),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            luma_table: DEFAULT_LUMA_TABLE,
            chroma_table: DEFAULT_CHROMA_TABLE,
        }
    }
}

/// Raw cosine representation
pub struct Rcr;

pub fn encode<T>(output: &mut T, settings: Settings, image: &Image<Lab8>)
where T: Write
{
    assert!(image.width() % 8 == 0);
    assert!(image.height() % 8 == 0);

    let w = image.width() / 8;
    let h = image.height() / 8;

    output.write(&(image.width() as u16).to_be_bytes()).unwrap();
    output.write(&(image.height() as u16).to_be_bytes()).unwrap();

    for y in 0..h {
        for x in 0..w {
            let mut l_spacial = [0.0; 64];
            let mut a_spacial = [0.0; 64];
            let mut b_spacial = [0.0; 64];
    
            let mut l_frequency = [0.0; 64];
            let mut a_frequency = [0.0; 64];
            let mut b_frequency = [0.0; 64];

            let mut l_raw = [0; 64];
            let mut a_raw = [0; 64];
            let mut b_raw = [0; 64];

            for i in 0..8 {
                for j in 0..8 {
                    let index = i + (8 * x) + (8 * y + j) * image.width();
                    let color = image.data()[index];
                    l_spacial[i + 8 * j] = color.l as f32;
                    a_spacial[i + 8 * j] = color.a as f32;
                    b_spacial[i + 8 * j] = color.b as f32;
                }
            }

            dct(&l_spacial, &mut l_frequency);
            dct(&a_spacial, &mut a_frequency);
            dct(&b_spacial, &mut b_frequency);

            quant(&l_frequency, &mut l_raw, &settings.luma_table);
            quant(&a_frequency, &mut a_raw, &settings.chroma_table);
            quant(&b_frequency, &mut b_raw, &settings.chroma_table);

            output.write(&l_raw).unwrap();
            output.write(&a_raw).unwrap();
            output.write(&b_raw).unwrap();
        }
    }
}

pub fn decode<T>(input: &mut T, settings: Settings) -> Image<Lab8>
where T: Read + AsRef<[u8]>
{
    let mut input = BufReader::new(input);
    let mut bytes = [0; 2];

    input.read_exact(&mut bytes).unwrap();
    let width: u16 = u16::from_be_bytes(bytes);

    input.read_exact(&mut bytes).unwrap();
    let height: u16 = u16::from_be_bytes(bytes);
    
    assert!(width % 8 == 0);
    assert!(height % 8 == 0);

    let w = (width / 8) as usize;
    let h = (height / 8) as usize;

    let mut data: Vec<Lab8> = vec![Lab8::default(); width as usize * height as usize];
    
    for y in 0..h {
        for x in 0..w {
            let mut l_raw = [0; 64];
            let mut a_raw = [0; 64];
            let mut b_raw = [0; 64];

            let mut l_frequency = [0.0; 64];
            let mut a_frequency = [0.0; 64];
            let mut b_frequency = [0.0; 64];

            let mut l_spacial = [0.0; 64];
            let mut a_spacial = [0.0; 64];
            let mut b_spacial = [0.0; 64];

            input.read_exact(&mut l_raw).unwrap();
            input.read_exact(&mut a_raw).unwrap();
            input.read_exact(&mut b_raw).unwrap();

            inv_quant(&l_raw, &mut l_frequency, &settings.luma_table);
            inv_quant(&a_raw, &mut a_frequency, &settings.chroma_table);
            inv_quant(&b_raw, &mut b_frequency, &settings.chroma_table);

            inv_dct(&l_frequency, &mut l_spacial);
            inv_dct(&a_frequency, &mut a_spacial);
            inv_dct(&b_frequency, &mut b_spacial);

            for j in 0..8 {
                for i in 0..8 {
                    let index = i + (8 * x) + (8 * y + j) * width as usize;
                    data[index].l = l_spacial[i + 8 * j] as i8;
                    data[index].a = a_spacial[i + 8 * j] as i8;
                    data[index].b = b_spacial[i + 8 * j] as i8;
                }
            }
        }
    }
    
    Image::new(width as usize, height as usize, data)
}
