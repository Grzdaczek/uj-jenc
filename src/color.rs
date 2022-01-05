#![allow(dead_code)]

pub type Rgb8 = Rgb<u8>;
pub type Luma8 = Luma<u8>;
pub type Lab8 = Lab<u8>;

#[derive(Debug, Clone, Copy, Default)]
pub struct Luma<T> {
    pub l: T,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Lab<T> {
    pub l: T,
    pub a: T,
    pub b: T,
}

impl From<Rgb8> for Lab8 {
    fn from(other: Rgb8) -> Self {
        let r = other.r as f32;
        let g = other.g as f32;
        let b = other.b as f32;

        Self {
            l: ( 0.2126 * r +  0.7152 * g +  0.0722 * b) as u8,
            a: (-0.1146 * r + -0.3854 * g +     0.5 * b + 128.0) as u8,
            b: (    0.5 * r + -0.4542 * g + -0.0458 * b + 128.0) as u8,
        }
    }
}

impl From<Lab8> for Rgb8 {
    fn from(other: Lab8) -> Self {
        let l = other.l as f32;
        let a = (other.a as f32) - 128.0;
        let b = (other.b as f32) - 128.0;

        Self {
            r: (l               +  1.5748 * b) as u8,
            g: (l + -0.1873 * a + -0.4681 * b) as u8,
            b: (l +  1.8556 * a              ) as u8,
        }
    }
}