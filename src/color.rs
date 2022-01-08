#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Lab8 {
    pub l: i8,
    pub a: i8,
    pub b: i8,
}

impl From<Rgb8> for Lab8 {
    fn from(other: Rgb8) -> Self {
        let r = other.r as f32;
        let g = other.g as f32;
        let b = other.b as f32;

        Self {
            l: ( 0.2126 * r +  0.7152 * g +  0.0722 * b - 128.0) as i8,
            a: (-0.1146 * r + -0.3854 * g +     0.5 * b) as i8,
            b: (    0.5 * r + -0.4542 * g + -0.0458 * b) as i8,
        }
    }
}

impl From<Lab8> for Rgb8 {
    fn from(other: Lab8) -> Self {
        let l = other.l as f32 + 128.0;
        let a = other.a as f32;
        let b = other.b as f32;

        Self {
            r: (l               +  1.5748 * b) as u8,
            g: (l + -0.1873 * a + -0.4681 * b) as u8,
            b: (l +  1.8556 * a              ) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lab8_to_rgb8() {
        let black: Rgb8 = Lab8 { l: -128, a: 0, b: 0 } .into();
        let white: Rgb8 = Lab8 { l: 127, a: 0, b: 0 } .into();

        assert_eq!(black, Rgb8 { r: 0, g: 0, b: 0 });
        assert_eq!(white, Rgb8 { r: 255, g: 255, b: 255 });
    }

    #[test]
    fn rgb8_to_lab8() {
        let black: Lab8 = Rgb8 { r: 0, g: 0, b: 0 } .into();
        let white: Lab8 = Rgb8 { r: 255, g: 255, b: 255 } .into();

        assert_eq!(black, Lab8 { l: -128, a: 0, b: 0 });
        assert_eq!(white, Lab8 { l: 127, a: 0, b: 0 });
    }
}
