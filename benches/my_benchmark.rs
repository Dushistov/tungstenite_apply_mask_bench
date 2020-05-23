use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tungstenite_apply_mask_bench::{
    apply_mask_fallback, apply_mask_fast32, apply_mask_fast32_safe,
};

fn bench_apply_mask(c: &mut Criterion) {
    let mask = [0x6d, 0xb6, 0xb2, 0x80];
    let unmasked: [u8; 29] = [
        0xf3, 0x00, 0x01, 0x02, 0x03, 0x80, 0x81, 0x82, 0xff, 0xfe, 0x00, 0x17, 0x74, 0xf9, 0x12,
        0x03, 0x17, 0x18, 0x19, 0x40, 0x52, 0xfa, 0xda, 0x7a, 0x18, 0xa0, 0x5f, 0x32, 0x55,
    ];
    let mut group = c.benchmark_group("apply_mask");
    for off in 0..=3 {
        group.bench_with_input(BenchmarkId::new("fallback", off), &off, |b, &off| {
            b.iter(|| {
                let mut masked = unmasked.clone();
                apply_mask_fallback(black_box(&mut masked[off..]), black_box(mask));
            });
        });
        group.bench_with_input(BenchmarkId::new("fast", off), &off, |b, &off| {
            b.iter(|| {
                let mut masked = unmasked.clone();
                apply_mask_fast32(black_box(&mut masked[off..]), black_box(mask));
            });
        });

        group.bench_with_input(BenchmarkId::new("fast_safe", off), &off, |b, &off| {
            b.iter(|| {
                let mut masked = unmasked.clone();
                apply_mask_fast32_safe(black_box(&mut masked[off..]), black_box(mask));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_apply_mask);
criterion_main!(benches);
