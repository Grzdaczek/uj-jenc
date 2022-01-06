#![forbid(unsafe_code)]
// #![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;

pub mod codec {
    use crate::image::{Image, ImageBuffer};

    pub mod ppm;
    pub mod rcr;
    
    pub trait Decoder<T> {
        fn decode(&self, buffer: ImageBuffer) -> Image<T>;
    }
    
    pub trait Encoder<T> {
        fn encode(&self, image: Image<T>) -> ImageBuffer;
    }
}

pub mod color;
pub mod image;
