use std::hint::black_box;
use std::slice;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

fn incrementing_loop(vec: &mut [u32], k: usize) {
    for _ in 0..k {
        for i in (0..vec.len()).step_by(D) {
            vec[i] += 1;
        }
    }
}

const D: usize = 1;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("Memory Bandwidth");
    for n in (2usize.pow(12)..2usize.pow(18)).step_by(16384) {

        let k = 100_000_000 / n;
        let m = D * n;

        let mut vec = vec![0; m];
        let ptr = vec.as_mut_ptr();

        g.throughput(Throughput::Elements(m as u64));

        g.bench_with_input(BenchmarkId::from_parameter(m), &(m, k), |b, (m, k)| {
            b.iter(|| incrementing_loop(unsafe { slice::from_raw_parts_mut(ptr, *m) }, *k))
        });
    }

    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
