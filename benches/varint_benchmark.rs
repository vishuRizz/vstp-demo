use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use vstp::encoding::{decode_varint, encode_varint, varint_len};

fn varint_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("varint");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);

    // Benchmark small numbers
    group.bench_function("encode_small", |b| {
        b.iter(|| {
            for i in 0..128u64 {
                black_box(encode_varint(i));
            }
        })
    });

    group.bench_function("decode_small", |b| {
        let encoded: Vec<_> = (0..128u64).map(|i| encode_varint(i)).collect();
        b.iter(|| {
            for bytes in &encoded {
                black_box(decode_varint(bytes));
            }
        })
    });

    // Benchmark large numbers
    group.bench_function("encode_large", |b| {
        b.iter(|| {
            for i in (u64::MAX - 128)..u64::MAX {
                black_box(encode_varint(i));
            }
        })
    });

    group.bench_function("decode_large", |b| {
        let encoded: Vec<_> = ((u64::MAX - 128)..u64::MAX)
            .map(|i| encode_varint(i))
            .collect();
        b.iter(|| {
            for bytes in &encoded {
                black_box(decode_varint(bytes));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, varint_benchmark);
criterion_main!(benches);
