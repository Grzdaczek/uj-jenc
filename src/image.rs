#![allow(dead_code)]

use std::{fs, io};

use crate::color::{RgbU8, YcbcrU8};
use crate::codec::{Encode, Decode};

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
}

impl<T> Image<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn encode(self, encoder: & dyn Encode<T>) -> ImageBuffer {
        encoder.encode(self)
    }
}

impl ImageBuffer {
    pub fn read(path: &str) -> io::Result<Self> {
        match fs::read(path) {
            Ok(data) => Ok(Self {data}),
            Err(err) => io::Result::Err(err),
        }
    }

    pub fn write(self, path: &str) -> io::Result<()> {
        fs::write(path, self.data)
    }

    pub fn decode<T>(self, decoder: & dyn Decode<T>) -> Image<T> {
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

impl From<Image<RgbU8>> for Image<YcbcrU8> {
    fn from(other: Image<RgbU8>) -> Self {
        Self {
            data: other.data
                .iter()
                .map(|&x| YcbcrU8::from(x))
                .collect(),
            width: other.width,
            height: other.height,
        }
    }
}

impl From<Image<YcbcrU8>> for Image<RgbU8> {
    fn from(other: Image<YcbcrU8>) -> Self {
        Self {
            data: other.data
                .iter()
                .map(|&x| RgbU8::from(x))
                .collect(),
            width: other.width,
            height: other.height,
        }
    }
}