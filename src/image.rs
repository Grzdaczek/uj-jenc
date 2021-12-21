#![allow(dead_code)]

use crate::traits::Color;

pub struct Image<C: Color> {
    data: Vec<C>,
    width: usize,
    height: usize,
}

impl<C: Color> Image<C> {
    pub fn data(&self) -> &Vec<C> {
        &self.data
    }
    
    pub fn data_mut(& mut self) -> & mut Vec<C> {
        & mut self.data
    }

    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Image<T>
where
    T:  Color + Default + Clone + Copy
{
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}