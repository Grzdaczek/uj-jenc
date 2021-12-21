#![allow(dead_code)]

use crate::traits::Color;

#[derive(Debug, Clone, Copy, Default)]
pub struct RgbU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color for RgbU8 {

}