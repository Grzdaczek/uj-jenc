use crate::image::{Image, ImageBuffer};

pub mod ppm;
pub mod rcr;

pub trait Decoder<T> {
    fn decode(&self, buffer: ImageBuffer) -> Image<T>;
}

pub trait Encoder<T> {
    fn encode(&self, image: Image<T>) -> ImageBuffer;
}
