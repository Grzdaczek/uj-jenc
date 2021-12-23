#![allow(dead_code)]

use crate::color::FromColor;

#[derive(Debug, Clone)]
pub struct Image<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
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

    pub fn from_image<Other>(other: &Image<Other>) -> Self 
    where
        T: FromColor<Other>
    {
        Self {
            data: other.data
                .iter()
                .map(T::from_color)
                .collect(),
            width: other.width,
            height: other.height,
        }
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
}