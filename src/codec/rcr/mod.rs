use std::io::{BufReader, Read, Result, Write};

use crate::color::Lab8;
use crate::image::Image;

use unit::Unit;
use tables::from_quality;

pub mod tables;
pub mod unit;

pub struct Settings {
    luma_table: Unit<i32>,
    chroma_table: Unit<i32>,
}

impl Settings {
    pub fn new() -> Self {
        Self::quality(5)
    }

    pub fn quality(quality: usize) -> Self {
        let (luma_table, chroma_table) = from_quality(quality);

        Self {
            luma_table,
            chroma_table,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

pub fn encode<T>(output: &mut T, settings: Settings, image: &Image<Lab8>) -> Result<()>
where T: Write
{
    assert!(image.width() % 8 == 0);
    assert!(image.height() % 8 == 0);

    let w = image.width() / 8;
    let h = image.height() / 8;

    output.write(&(image.width() as u16).to_be_bytes())?;
    output.write(&(image.height() as u16).to_be_bytes())?;

    output.write(&settings.luma_table
        .convert(|x| (x as i8).to_be_bytes()[0])
        .unwrap()
    )?;

    output.write(&settings.chroma_table
        .convert(|x| (x as i8).to_be_bytes()[0])
        .unwrap()
    )?;

    for y in 0..h {
        for x in 0..w {
            let mut l_spacial = [0; 64];
            let mut a_spacial = [0; 64];
            let mut b_spacial = [0; 64];

            for i in 0..8 {
                for j in 0..8 {
                    let index = i + (8 * x) + (8 * y + j) * image.width();
                    let color = image.data()[index];
                    l_spacial[i + 8 * j] = color.l;
                    a_spacial[i + 8 * j] = color.a;
                    b_spacial[i + 8 * j] = color.b;
                }
            }

            output.write(&Unit::new(l_spacial)
                .convert(|x| x as f32)
                .dct()
                .convert(|x| x as i32)
                .quantize(settings.luma_table)
                .convert(|x| x as i8)
                .convert(|x| i8::to_be_bytes(x)[0])
                .zigzag()
                .unwrap()
            )?;

            output.write(&Unit::new(a_spacial)
                .convert(|x| x as f32)
                .dct()
                .convert(|x| x as i32)
                .quantize(settings.chroma_table)
                .convert(|x| x as i8)
                .convert(|x| i8::to_be_bytes(x)[0])
                .zigzag()
                .unwrap()
            )?;

            output.write(&Unit::new(b_spacial)
                .convert(|x| x as f32)
                .dct()
                .convert(|x| x as i32)
                .quantize(settings.chroma_table)
                .convert(|x| x as i8)
                .convert(|x| i8::to_be_bytes(x)[0])
                .zigzag()
                .unwrap()
            )?;
        }
    };

    Ok(())
}

pub fn decode<T>(input: &mut T) -> Result<Image<Lab8>>
where T: Read + AsRef<[u8]>
{
    let mut input = BufReader::new(input);
    let mut bytes = [0; 2];

    input.read_exact(&mut bytes)?;
    let width: u16 = u16::from_be_bytes(bytes);

    input.read_exact(&mut bytes)?;
    let height: u16 = u16::from_be_bytes(bytes);
    
    assert!(width % 8 == 0);
    assert!(height % 8 == 0);

    let mut bytes = [0; 64];
    input.read_exact(&mut bytes)?;
    let luma_table = Unit::new(bytes)
        .convert(|x| i8::from_be_bytes([x]) as i32);

    let mut bytes = [0; 64];
    input.read_exact(&mut bytes)?;
    let chroma_table = Unit::new(bytes)
        .convert(|x| i8::from_be_bytes([x]) as i32);

    let w = (width / 8) as usize;
    let h = (height / 8) as usize;

    let mut data: Vec<Lab8> = vec![Lab8::default(); width as usize * height as usize];
    
    for y in 0..h {
        for x in 0..w {
            let mut l_raw = [0; 64];
            let mut a_raw = [0; 64];
            let mut b_raw = [0; 64];

            input.read_exact(&mut l_raw)?;
            input.read_exact(&mut a_raw)?;
            input.read_exact(&mut b_raw)?;

            let l_spacial = Unit::new(l_raw)
                .convert(|x| i8::from_be_bytes([x]))
                .convert(|x| x as i32)
                .inv_quantize(luma_table)
                .convert(|x| x as f32)
                .inv_dct()
                .convert(|x| x as i8)
                .unwrap();

            let a_spacial = Unit::new(a_raw)
                .convert(|x| i8::from_be_bytes([x]))
                .convert(|x| x as i32)
                .inv_quantize(chroma_table)
                .convert(|x| x as f32)
                .inv_dct()
                .convert(|x| x as i8)
                .unwrap();

            let b_spacial = Unit::new(b_raw)
                .convert(|x| i8::from_be_bytes([x]))
                .convert(|x| x as i32)
                .inv_quantize(chroma_table)
                .convert(|x| x as f32)
                .inv_dct()
                .convert(|x| x as i8)
                .unwrap();

            for j in 0..8 {
                for i in 0..8 {
                    let index = i + (8 * x) + (8 * y + j) * width as usize;
                    data[index].l = l_spacial[i + 8 * j];
                    data[index].a = a_spacial[i + 8 * j];
                    data[index].b = b_spacial[i + 8 * j];
                }
            }
        }
    }
    
    Ok(Image::new(
        width as usize,
        height as usize,
        data
    ))
}
