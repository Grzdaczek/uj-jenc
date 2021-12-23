#![allow(dead_code)]

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