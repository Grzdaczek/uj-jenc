use crate::color::{Rgb8, Lab8};

#[derive(Debug, Clone)]
pub struct Image<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Image<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            data,
            width,
            height,
        }
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

    pub fn iter_block(&self) -> ImageBlockIterator<T> {
        ImageBlockIterator {
            image: self,
            x: 0,
            y: 0,
            w: self.width / 8,
            h: self.height / 8,
        }
    }
}

pub struct ImageBlockIterator<'a, T> {
    image: &'a Image<T>,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl<'a, T> Iterator for ImageBlockIterator<'a, T>
where T: Default + Copy
{
    type Item = [T; 64];

    fn next(&mut self) -> Option<Self::Item> {
        let mut u = [T::default(); 64];
        let x = self.x;
        let y = self.y;

        if self.y == self.h {
            None
        }
        else {
            for i in 0..8 {
                for j in 0..8 {
                    let index = i + (8 * x) + (8 * y + j) * self.image.width();
                    let color = self.image.data()[index];
                    u[i + 8 * j] = color;
                }
            }
    
            self.x += 1;
            if self.x == self.w {
                self.x = 0;
                self.y += 1;
            }

            Some(u)
        }
    }
}

impl<T> Image<T> {

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