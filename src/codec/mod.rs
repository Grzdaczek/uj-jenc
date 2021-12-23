use crate::image::Image;

pub mod ppm;
pub mod rcr;

pub trait Decode<T> {
    fn decode(&self, buffer: &[u8]) -> Image<T>;
}

pub trait Encode<T> {
    fn encode(&self, image: &Image<T>) -> Vec<u8>;
}
