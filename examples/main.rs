#![allow(dead_code)]

use std::f32::consts::PI;

fn main() {
    let mut x = Vec::new();

    for _ in 0..4 {
        x.push(255.0);
    }

    for _ in 0..4 {
        x.push(0.0);
    }

    let x_hat = dct2(&x);

    x_hat
        .iter()
        .enumerate()
        .for_each(|(n, x)| println!("{}: {}", n, x));

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
