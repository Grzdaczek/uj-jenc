use crate::image::{Image, ImageBuffer};

pub mod ppm;
pub mod rcr;

pub trait Decode<T> {
    fn decode(&self, buffer: ImageBuffer) -> Image<T>;
}

pub trait Encode<T> {
    fn encode(&self, image: Image<T>) -> ImageBuffer;
}
