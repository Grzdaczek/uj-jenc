#![allow(dead_code)]

pub type RgbU8 = Rgb<u8>;
pub type YcbcrU8 = Ycbcr<u8>;

pub trait FromColor<T> {
    fn from_color(other: &T) -> Self;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Ycbcr<T> {
    pub y: T,
    pub cb: T,
    pub cr: T,
}

impl<T: Clone> FromColor<T> for T {
    fn from_color(other: &T) -> Self {
        return other.clone();
    }
}

impl FromColor<RgbU8> for YcbcrU8 {
    fn from_color(other: &RgbU8) -> Self {
        let r = other.r as f32;
        let g = other.g as f32;
        let b = other.b as f32;

        Self {
            y:  ( 0.2126 * r +  0.7152 * g +  0.0722 * b) as u8,
            cb: (-0.1146 * r + -0.3854 * g +     0.5 * b + 128.0) as u8,
            cr: (    0.5 * r + -0.4542 * g + -0.0458 * b + 128.0) as u8,
        }
    }
}

impl FromColor<YcbcrU8> for RgbU8 {
    fn from_color(other: &YcbcrU8) -> Self {
        let y = other.y as f32;
        let cb = (other.cb as f32) - 128.0;
        let cr = (other.cr as f32) - 128.0;

        Self {
            r: (y                +  1.5748 * cr) as u8,
            g: (y + -0.1873 * cb + -0.4681 * cr) as u8,
            b: (y +  1.8556 * cb              ) as u8,
        }
    }
}
