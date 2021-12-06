fn main() {
    let x: Vec<f64> = vec![
        1.0,
        1.0,
        1.0,
        1.0,
    ];

    let x_hat = dct2(&x);

    println!("{:?}", x_hat);
}

fn dct2(x: & Vec<f64>) -> Vec<f64> {
    let n_large = x.len();
    let mut x_hat = Vec::with_capacity(n_large);
    unsafe { x_hat.set_len(n_larg); }

    const PI = 3.1415;

    for k in 0..n_large {
        x_hat[k] = x
            .iter()
            .enumerate()
            .map(|(i, xn)| {
                xn * ((PI / n_large as f64) * (n + 1/2) * k).cos()
            })
            .sum();
    }

    x_hat
        .iter_mut()
        .enumerate()
        .for_each(|(i, xn)| {});
    
    x_hat
}

fn dct3() {

}
