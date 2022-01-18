use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pixlib::codec::rcr;

fn rcr_benchmark(c: &mut Criterion) {
    let s: [u8; 64] = [
        16, 11, 10, 16,  24,  40,  51,  61,
        12, 12, 14, 19,  26,  58,  60,  55,
        14, 13, 16, 24,  40,  57,  69,  56,
        14, 17, 22, 29,  51,  87,  80,  62,
        18, 22, 37, 56,  68, 109, 103,  77,
        24, 35, 55, 64,  81, 104, 113,  92,
        49, 64, 78, 87, 103, 121, 120, 101,
        72, 92, 95, 98, 112, 100, 103,  99,
    ];

    let u_u8 = rcr::unit::Unit::new(s);
    let u_i32 = rcr::unit::Unit::new(s).convert(|x| x as i32);
    let u_f32 = rcr::unit::Unit::new(s).convert(|x| x as f32);
    
    c.bench_function("dct f", |b| b.iter(|| {
        let u = black_box(u_f32);
        u.dct();
    }));
    
    c.bench_function("dct i", |b| b.iter(|| {
        let u = black_box(u_i32);
        u.dct();
    }));
    
    c.bench_function("quantize", |b| b.iter(|| {
        let u = black_box(u_i32);
        let (t, _) = rcr::tables::from_quality(5);
        u.quantize(t);
    }));

    c.bench_function("inverse quantize", |b| b.iter(|| {
        let u = black_box(u_i32);
        let (t, _) = rcr::tables::from_quality(5);
        u.inv_quantize(t);
    }));

    c.bench_function("full encode f32", |b| b.iter(|| {
        let u = black_box(u_u8);
        let (t, _) = rcr::tables::from_quality(5);
        u.convert(|x| x as f32)
            .dct()
            .convert(|x| x as i32)
            .quantize(t)
            .convert(|x| x as i8)
            .convert(|x| x.to_be_bytes()[0])
            .unwrap()
    }));

    c.bench_function("full encode i32", |b| b.iter(|| {
        let u = black_box(u_u8);
        let (t, _) = rcr::tables::from_quality(5);
        u.convert(|x| x as i32)
            .dct()
            .quantize(t)
            .convert(|x| x as i8)
            .convert(|x| x.to_be_bytes()[0])
            .unwrap()
    }));
}

criterion_group!(benches, rcr_benchmark);
criterion_main!(benches);