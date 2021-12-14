#![allow(dead_code)]
#![allow(unused_variables)]

extern crate uj_jenc as jenc;

use std::f32::consts::PI;
use jenc::unit::Unit;

fn main() {
    let mut unit = Unit::<u8>::new();

    unit[(1, 1)] = 10;
    println!("{:?}", unit);

    // let mut x = Vec::new();

    // for _ in 0..4 {
    //     x.push(255.0);
    // }

    // for _ in 0..4 {
    //     x.push(0.0);
    // }

    // let x_hat = dct2(&x);

    // x_hat
    //     .iter()
    //     .enumerate()
    //     .for_each(|(n, x)| println!("{}: {}", n, x));

    // unit
    //     .into_iter()
    //     .enumerate()
    //     .for_each(|(n, x)| println!("{}: {}", n, x));

}

fn dct2(x: & Vec<f32>) -> Vec<f32> {
    let len = x.len();
    let step = PI / len as f32;
    
    return (0..len)
		.map(|k| match k {
            0 => (1.0 / (len as f32).sqrt()) * x.iter().sum::<f32>(),
            _ => (2.0 / len as f32).sqrt() * x.iter()
                .enumerate()
                .map(|(n, xn)| xn * ( step * k as f32 * (0.5 + n as f32)).cos() )
                .sum::<f32>(),
        })
        .collect::<Vec<f32>>();
}
