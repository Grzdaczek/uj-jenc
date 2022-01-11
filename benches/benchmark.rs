use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pixlib::codec::rcr;

fn rcr_benchmark(c: &mut Criterion) {
    let s: [f32; 64] = [
        16.0, 11.0, 10.0, 16.0,  24.0,  40.0,  51.0,  61.0,
        12.0, 12.0, 14.0, 19.0,  26.0,  58.0,  60.0,  55.0,
        14.0, 13.0, 16.0, 24.0,  40.0,  57.0,  69.0,  56.0,
        14.0, 17.0, 22.0, 29.0,  51.0,  87.0,  80.0,  62.0,
        18.0, 22.0, 37.0, 56.0,  68.0, 109.0, 103.0,  77.0,
        24.0, 35.0, 55.0, 64.0,  81.0, 104.0, 113.0,  92.0,
        49.0, 64.0, 78.0, 87.0, 103.0, 121.0, 120.0, 101.0,
        72.0, 92.0, 95.0, 98.0, 112.0, 100.0, 103.0,  99.0,
    ];

    let u1 = rcr::unit::Unit::new(s);
    let u2 = rcr::unit::Unit::new(s).convert(|x| x as i32);
    
    c.bench_function("dct", |b| b.iter(|| {
        let u = black_box(u1);
        u.dct();
    }));

    c.bench_function("inverse dct", |b| b.iter(|| {
        let u = black_box(u1);
        u.inv_dct();
    }));
    
    c.bench_function("quantize", |b| b.iter(|| {
        let u = black_box(u2);
        let (t, _) = rcr::tables::from_quality(5);
        u.quantize(t);
    }));

    c.bench_function("inverse quantize", |b| b.iter(|| {
        let u = black_box(u2);
        let (t, _) = rcr::tables::from_quality(5);
        u.inv_quantize(t);
    }));

    c.bench_function("full encode", |b| b.iter(|| {
        let u = black_box(u1);
        let (t, _) = rcr::tables::from_quality(5);
        u.dct()
            .convert(|x| x as i32)
            .quantize(t)
            .convert(|x| x as i8)
            .convert(|x| x.to_be_bytes()[0])
            .unwrap()
    }));
}

criterion_group!(benches, rcr_benchmark);
criterion_main!(benches);