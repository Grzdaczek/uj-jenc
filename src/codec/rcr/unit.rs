use std::ops::{Div, Mul};

const COS_MUL_F: [f32; 64] = [
    /*
    lazy_static! {
        static ref COS_MUL_F: [f32; 64] = {
            let mut mul: [f32; 64] = [0_f32; 64];
            for k in 0..8 {
                for n in 0..8 {
                    let kf = k as f32;
                    let nf = n as f32;
                    let cos = ((std::f32::consts::PI / 8.0) * (0.0 + kf)* (0.5 + nf)).cos();
                    
                    let correction = match k {
                        0 => 0.5 * (1.0 / 2_f32.sqrt()),
                        _ => 0.5,
                    };

                    mul[k + 8*n] = cos * correction
                }
            }

            mul
        };
    }
    */

    0.353553,  0.490392,  0.461939,  0.415734,  0.353553,  0.277785,  0.191341,  0.097545,
    0.353553,  0.415734,  0.191341, -0.097545, -0.353553, -0.490392, -0.461939, -0.277785,
    0.353553,  0.277785, -0.191341, -0.490392, -0.353553,  0.097545,  0.461939,  0.415734,
    0.353553,  0.097545, -0.461939, -0.277785,  0.353553,  0.415734, -0.191341, -0.490392,
    0.353553, -0.097545, -0.461939,  0.277785,  0.353553, -0.415734, -0.191341,  0.490392,
    0.353553, -0.277785, -0.191341,  0.490392, -0.353553, -0.097545,  0.461939, -0.415735,
    0.353553, -0.415734,  0.191341,  0.097545, -0.353553,  0.490392, -0.461939,  0.277785,
    0.353553, -0.490392,  0.461939, -0.415734,  0.353553, -0.277785,  0.191341, -0.097545,
];

const Q: i32 = 21; // 32 - <sign bit> - 10 (due to dct output range for u8)
const DCT_MUL_I: [i32; 64] = [
    /*
    lazy_static! {
        static ref DCT_MUL_I: [i32; 64] = {
            let mut mul: [i32; 64] = [0; 64];
            for k in 0..8 {
                for n in 0..8 {
                    let kf = k as f32;
                    let nf = n as f32;
                    let cos = ((std::f32::consts::PI / 8.0) * (0.0 + kf)* (0.5 + nf)).cos();
                    
                    let correction = match k {
                        0 => 0.5 * (1.0 / 2_f32.sqrt()),
                        _ => 0.5,
                    };

                    mul[k + 8*n] = (cos * correction * 2f32.powi(Q)) as i32
                }
            }

            mul
        };
    }
    */

    741455,  1028427,  968757,   871859,  741455,   582557,  401272,   204567,
    741455,   871859,  401272,  -204567, -741455, -1028427, -968757,  -582557,
    741455,   582557, -401272, -1028427, -741455,   204567,  968757,   871859,
    741455,   204567, -968757,  -582557,  741455,   871859, -401273, -1028427, 
    741455,  -204567, -968757,   582557,  741455,  -871859, -401273,  1028427, 
    741455,  -582557, -401272,  1028427, -741455,  -204567,  968757,  -871859, 
    741455,  -871859,  401272,   204567, -741455,  1028427, -968757,   582557, 
    741455, -1028427,  968757,  -871859,  741455,  -582557,  401273,  -204567,
];



#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Unit<T>([T; 64]);

impl<T> Unit<T> where T: Default + Copy + Div<Output = T> + Mul<Output = T> {
    pub fn new(data: [T; 64]) -> Self {
        Self(data)
    }

    pub fn unwrap(self) -> [T; 64] {
        self.0
    }

    pub fn quantize(self, table: Self) -> Self {
        let mut out = [T::default(); 64];
        for i in 0..64 {
            out[i] = self.0[i] / table.0[i];
        }

        Self(out)
    }

    pub fn inv_quantize(self, table: Self) -> Self {
        let mut out = [T::default(); 64];
        for i in 0..64 {
            out[i] = self.0[i] * table.0[i];
        }

        Self(out)
    }

    fn inner_permute(self, permutation: [usize; 64]) -> Self {
        Self(permutation.map(|i| self.0[i]))
    }

    pub fn zigzag(self) -> Self {
        let permutation: [usize; 64] = [
             0,  1,  8, 16,  9,  2,  3, 10,
            17, 24, 32, 25, 18, 11,  4,  5,
            12, 19, 26, 33, 40, 48, 41, 34,
            27, 20, 13,  6,  7, 14, 21, 28,
            35, 42, 49, 56, 57, 50, 43, 36,
            29, 22, 15, 23, 30, 37, 44, 51,
            58, 59, 52, 45, 38, 31, 39, 46,
            53, 60, 61, 54, 47, 55, 62, 63,
        ];

        self.inner_permute(permutation)
    }

    pub fn inv_zigzag(self) -> Self {
        let permutation: [usize; 64] = [
             0,  1,  5,  6, 14, 15, 27, 28,
             2,  4,  7, 13, 16, 26, 29, 42,
             3,  8, 12, 17, 25, 30, 41, 43,
             9, 11, 18, 24, 31, 40, 44, 53,
            10, 19, 23, 32, 39, 45, 52, 54,
            20, 22, 33, 38, 46, 51, 55, 60,
            21, 34, 37, 47, 50, 56, 59, 61,
            35, 36, 48, 49, 57, 58, 62, 63,
        ];

        self.inner_permute(permutation)
    }

    pub fn convert<R>(self, f: fn(T) -> R) -> Unit<R> {
        Unit(self.0.map(f))
    }
}

impl Unit<f32> {
    pub fn dct(self) -> Unit<f32> {
        let in_buf = self.0;
        let mut mid_buf = [0.0; 64];
        let mut out_buf = [0.0; 64];

        for k in 0..8 {
            for y in 0..8 {
                mid_buf[y + 8*k] = (0..8)
                    .map(|n| in_buf[y + 8*n] * COS_MUL_F[k + 8*n])
                    .sum();
            }
        }
    
        for k in 0..8 {
            for x in 0..8 {
                out_buf[k + 8*x] = (0..8)
                    .map(|n| mid_buf[n + 8*x] * COS_MUL_F[k + 8*n])
                    .sum();
            }
        }

        Unit(out_buf)
    }

    pub fn inv_dct(self) -> Unit<f32> {
        let in_buf = self.0;
        let mut mid_buf = [0.0; 64];
        let mut out_buf = [0.0; 64];

        for k in 0..8 {
            for y in 0..8 {
                mid_buf[y + 8*k] = (0..8)
                    .map(|n| in_buf[y + 8*n] * COS_MUL_F[n + 8*k])
                    .sum();
            }
        }
    
        for k in 0..8 {
            for x in 0..8 {
                out_buf[k + 8*x] = (0..8)
                    .map(|n| mid_buf[n + 8*x] * COS_MUL_F[n + 8*k])
                    .sum();
            }
        }

        Unit(out_buf)
    }
}

impl Unit<i32> {
    pub fn dct(self) -> Self {
        let in_buf = self.0;
        let mut mid_buf = [0; 64];
        let mut out_buf = [0; 64];

        for k in 0..8 {
            for y in 0..8 {
                let c: i32 = (0..8)
                    .map(|n| in_buf[y + 8*n] * DCT_MUL_I[k + 8*n])
                    .sum();
                mid_buf[y + 8*k] = c >> Q;
            }
        }
    
        for k in 0..8 {
            for x in 0..8 {
                let c: i32 = (0..8)
                    .map(|n| mid_buf[n + 8*x] * DCT_MUL_I[k + 8*n])
                    .sum();
                out_buf[k + 8*x] = c >> Q;
            }
        }

        Unit(out_buf)
    }

    pub fn inv_dct(self) -> Self {
        let in_buf = self.0;
        let mut mid_buf = [0; 64];
        let mut out_buf = [0; 64];

        for k in 0..8 {
            for y in 0..8 {
                let c: i32 = (0..8)
                    .map(|n| in_buf[y + 8*n] * DCT_MUL_I[n + 8*k])
                    .sum();
                mid_buf[y + 8*k] = c >> Q;
            }
        }
    
        for k in 0..8 {
            for x in 0..8 {
                let c: i32 = (0..8)
                    .map(|n| mid_buf[n + 8*x] * DCT_MUL_I[n + 8*k])
                    .sum();
                out_buf[k + 8*x] = c >> Q;
            }
        }

        Unit(out_buf)
    }
}

impl<T> IntoIterator for Unit<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 64>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// struct ImageUnitIterator<'a, T> {
//     _image: &'a Image<T>
// }

// impl<'a, T> Iterator for ImageUnitIterator<'a, T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

// impl<T> Image<T> {
//     fn _iter(&self) -> ImageUnitIterator<T> {
//         ImageUnitIterator {
//             _image: self,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dct_inverese_equality() {
        let spacial = [
            16, 11, 10, 16,  24,  40,  51,  61,
            12, 12, 14, 19,  26,  58,  60,  55,
            14, 13, 16, 24,  40,  57,  69,  56,
            14, 17, 22, 29,  51,  87,  80,  62,
            18, 22, 37, 56,  68, 109, 103,  77,
            24, 35, 55, 64,  81, 104, 113,  92,
            49, 64, 78, 87, 103, 121, 120, 101,
            72, 92, 95, 98, 112, 100, 103,  99,
        ];

        let new_spacial = Unit::new(spacial)
            .convert(|x| x as f32)
            .dct()
            .inv_dct()
            .convert(|x| x.round() as i32);

        spacial
            .iter()
            .zip(new_spacial.into_iter())
            .for_each(|(&a, b)| assert_eq!(a, b));
    }

    #[test]
    fn n_dct_inverese_equality() {
        let spacial = [
            16, 11, 10, 16,  24,  40,  51,  61,
            12, 12, 14, 19,  26,  58,  60,  55,
            14, 13, 16, 24,  40,  57,  69,  56,
            14, 17, 22, 29,  51,  87,  80,  62,
            18, 22, 37, 56,  68, 109, 103,  77,
            24, 35, 55, 64,  81, 104, 113,  92,
            49, 64, 78, 87, 103, 121, 120, 101,
            72, 92, 95, 98, 112, 100, 103,  99,
        ];

        let new_spacial = Unit::new(spacial)
            .dct()
            .inv_dct()
            .unwrap();
        
        new_spacial.chunks(8).for_each(|x| {
            println!("{:?}", x);
        });

        let a = DCT_MUL_I.iter();

        println!("{:?}", a);

        spacial
            .iter()
            .zip(new_spacial.iter())
            .for_each(|(a, b)| assert!((a - b).abs() < 8));
    }

    #[test]
    fn zigzag_inverse_equality() {
        let data = [
             0,  1,  2,  3,  4,  5,  6,  7,
             8,  9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39,
            40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55,
            56, 57, 58, 59, 60, 61, 62, 63,
        ];

        let a = Unit::new(data);
        let b = a.clone()
            .zigzag()
            .inv_zigzag();

        assert_eq!(a, b);
    }
}
