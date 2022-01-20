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

pub fn encode<T>(mut output: T, settings: Settings, image: &Image<Lab8>) -> Result<()>
where T: Write
{
    assert!(image.width() % 8 == 0);
    assert!(image.height() % 8 == 0);

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

    image
        .iter_block()
        .for_each(|block| {
            let mut l = [0; 64];
            let mut a = [0; 64];
            let mut b = [0; 64];

            for i in 0..64 {
                l[i] = block[i].l;
                a[i] = block[i].a;
                b[i] = block[i].b;
            }

            let mut write_helper = |x: [i8; 64], t: Unit<i32>| -> Result<_> {
                output.write(&Unit::new(x)
                    .convert(|x| x as f32)
                    .dct()
                    .convert(|x| x as i32)
                    .quantize(t)
                    .convert(|x| i8::to_be_bytes(x as i8)[0])
                    .zigzag()
                    .unwrap()
                )
            };

            // TODO: proper error handler
            write_helper(l, settings.luma_table).unwrap();
            write_helper(a, settings.chroma_table).unwrap();
            write_helper(b, settings.chroma_table).unwrap();
        });

    Ok(())
}

pub fn decode<T>(input: T) -> Result<Image<Lab8>>
where T: Read
{
    let mut input = BufReader::new(input);
    let mut bytes = [0; 2];

    input.read_exact(&mut bytes)?;
    let width: u16 = u16::from_be_bytes(bytes);

    input.read_exact(&mut bytes)?;
    let height: u16 = u16::from_be_bytes(bytes);
    
    // TODO: proper error handler
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
            let mut read_helper = |t: Unit<i32>| -> Result<[i8; 64]> {
                let mut raw = [0; 64];
                input.read_exact(&mut raw)?;

                let data = Unit::new(raw)
                    .inv_zigzag()
                    .convert(|x| i8::from_be_bytes([x]) as i32)
                    .inv_quantize(t)
                    .convert(|x| x as f32)
                    .inv_dct()
                    .convert(|x| x as i8)
                    .unwrap();

                Ok(data)
            };

            let l = read_helper(luma_table)?;
            let a = read_helper(chroma_table)?;
            let b = read_helper(chroma_table)?;

            for j in 0..8 {
                for i in 0..8 {
                    let index = i + (8 * x) + (8 * y + j) * width as usize;
                    data[index].l = l[i + 8 * j];
                    data[index].a = a[i + 8 * j];
                    data[index].b = b[i + 8 * j];
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
