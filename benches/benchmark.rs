use criterion::{black_box, criterion_group, criterion_main, Criterion};

use uj_jenc::codec::rcr;

fn rcr_benchmark(c: &mut Criterion) {
    // let mut q = [0; 64];
    let mut f = [0.0; 64];
    let s = [
        16.0, 11.0, 10.0, 16.0,  24.0,  40.0,  51.0,  61.0,
        12.0, 12.0, 14.0, 19.0,  26.0,  58.0,  60.0,  55.0,
        14.0, 13.0, 16.0, 24.0,  40.0,  57.0,  69.0,  56.0,
        14.0, 17.0, 22.0, 29.0,  51.0,  87.0,  80.0,  62.0,
        18.0, 22.0, 37.0, 56.0,  68.0, 109.0, 103.0,  77.0,
        24.0, 35.0, 55.0, 64.0,  81.0, 104.0, 113.0,  92.0,
        49.0, 64.0, 78.0, 87.0, 103.0, 121.0, 120.0, 101.0,
        72.0, 92.0, 95.0, 98.0, 112.0, 100.0, 103.0,  99.0,
    ];
    
    c.bench_function("dct transform", |b| b.iter(|| {
        let input = black_box(&s);
        let output = black_box(&mut f);

        rcr::transform::dct(input, output);
    }));

    // c.bench_function("quantize", |b| b.iter(|| {
    //     let input = black_box(&f);
    //     let output = black_box(&mut q);

    //     rcr::transform::quant(input, output, &rcr::DEFAULT_LUMA_TABLE);
    // }));
    
}

criterion_group!(benches, rcr_benchmark);
criterion_main!(benches);