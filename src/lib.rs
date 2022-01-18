#![forbid(unsafe_code)]
// #![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;

pub mod codec {
    pub mod ppm;
    pub mod rcr;
}

pub mod color;
pub mod image;
