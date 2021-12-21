#![allow(dead_code)]
pub trait Pixel: Copy + Clone + Default {

}

#[derive(Copy, Clone, Debug, Default)]
pub struct Luma {
    y: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Ycbcr {
    y: u8,
    cb: u8,
    cr: u8,
}

impl Luma {
    pub fn new(y: u8) -> Self {
        Self {y}
    }
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b}
    }
}

impl Ycbcr {
    pub fn new(y: u8, cb: u8, cr: u8) -> Self {
        Self {y, cb, cr}
    }
}

impl Pixel for Luma {}
impl Pixel for Rgb {}
impl Pixel for Ycbcr {}

pub trait FromPixel<T: Pixel> {
    fn from_color(p: &T) -> Self;
}

impl<T: Pixel> FromPixel<T> for T {
    fn from_color(p: &T) -> Self {
        p.clone()
    }
}

impl FromPixel<Rgb> for Luma {
    fn from_color(p: &Rgb) -> Self {
        Self {
            y: (0.2126 * p.r as f32 + 0.7152 * p.g as f32 + 0.0722 * p.b as f32) as u8,
        }
    }
}

impl FromPixel<Luma> for Rgb {
    fn from_color(p: &Luma) -> Self {
        Self {
            r: p.y,
            g: p.y,
            b: p.y,
        }
    }
}

impl FromPixel<Rgb> for Ycbcr {
    fn from_color(p: &Rgb) -> Self {
        Self {
            y:  (0.299  * p.r as f32 + 0.587  * p.g as f32 + 0.114  * p.b as f32) as u8,
            cb: (-0.169 * p.r as f32 + -0.331 * p.g as f32 + 0.500  * p.b as f32 + 128.0) as u8,
            cr: (0.500  * p.r as f32 + -0.419 * p.g as f32 + -0.081 * p.b as f32 + 128.0) as u8,
        }
    }
}

#[derive(Debug)]
pub struct Pixelmap<T: Pixel> {
    pub data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Pixel> Pixelmap<T> 
{
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, T::default());
        Self {
            data,
            width,
            height,
        }
    }

    pub fn from_pixelmap<Other>(other: &Pixelmap<Other>) -> Self 
    where
        Other: Pixel,
        T: FromPixel<Other>
    {
        Self {
            data: other.data
                .iter()
                .map(FromPixel::from_color)
                .collect(),
            width: other.width,
            height: other.height,
        }
    }

    pub fn into_pixelmap<Other>(&self) -> Pixelmap<Other>
    where
        Other: Pixel + FromPixel<T>
    {
        Pixelmap::<Other>::from_pixelmap(self)
    }
}
