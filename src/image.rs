#![allow(dead_code)]

use std::{fs, io, usize};

use crate::color::{Rgb8, Lab8};
use crate::codec::{Encoder, Decoder};

#[derive(Debug, Clone)]
pub struct Image<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

pub struct ImageBuffer {
    pub data: Vec<u8>
}

impl<T> Image<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn encode(self, encoder: & dyn Encoder<T>) -> ImageBuffer {
        encoder.encode(self)
    }

    pub fn at(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    
    pub fn data_mut(& mut self) -> & mut Vec<T> {
        & mut self.data
    }

    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }

    // pub fn fragment_mut(
    //     &mut self,
    //     left: usize,
    //     top: usize,
    //     width: usize,
    //     height: usize) -> 
    // FragmentIteratorMut<T> {
    //     if left + width > self.width || top + height > self.height { 
    //         panic!("Fragment bounds are outside image");
    //     }
        
    //     FragmentIteratorMut {
    //         image: self,
    //         left,
    //         top,
    //         width,
    //         height,
    //         index: 0,
    //     }
    // }
}

// pub struct FragmentIteratorMut<'a, T> {
//     image: &'a mut Image<T>,
//     left: usize,
//     top: usize,
//     width: usize,
//     height: usize,
//     index: usize,
// }

// impl<'a, T> Iterator for FragmentIteratorMut<'a, T> {
//     type Item = &'a mut [T];

//     fn next(&mut self) -> Option<Self::Item> {
//         let y = self.index;
//         self.index += 1;

//         if y > self.height {
//             let begin = y * self.width + self.left;
//             let data = self.image.data_mut();
//             let slice = &mut data[begin..self.width];

//             Some(slice)
//         }
//         else {
//             None
//         }
//     }
// }

impl ImageBuffer {
    pub fn read(path: &str) -> io::Result<Self> {
        match fs::read(path) {
            Ok(data) => Ok(Self {data}),
            Err(err) => io::Result::Err(err),
        }
    }

    pub fn write(&self, path: &str) -> io::Result<()> {
        fs::write(path, &self.data)
    }

    pub fn decode<T>(self, decoder: & dyn Decoder<T>) -> Image<T> {
        decoder.decode(self)
    }
}

/*
impl<F, T> From<&Image<F>> for Image<T> 
where
    T: From<F>,
    F: Copy
{
    fn from(other: &Image<F>) -> Self {
        Self {
            data: other.data
                .iter()
                .map(|&x| T::from(x))
                .collect(),
            width: other.width,
            height: other.height,
        }
    }
}
*/

impl From<Image<Rgb8>> for Image<Lab8> {
    fn from(other: Image<Rgb8>) -> Self {
        Self {
            data: other.data
                .iter()
                .map(|&x| Lab8::from(x))
                .collect(),
            width: other.width,
            height: other.height,
        }
    }
}

impl From<Image<Lab8>> for Image<Rgb8> {
    fn from(other: Image<Lab8>) -> Self {
        Self {
            data: other.data
                .iter()
                .map(|&x| Rgb8::from(x))
                .collect(),
            width: other.width,
            height: other.height,
        }
    }
}