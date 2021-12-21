use crate::traits::Color;
use crate::image::Image;

pub mod ppm;
pub mod rcr;

pub trait Decode<C: Color> {
    fn decode(&self, buffer: &[u8]) -> Image<C>;
}

pub trait Encode<C: Color> {
    fn encode(&self, image: &Image<C>) -> Vec<u8>;
}
