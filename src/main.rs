use std::hint::black_box;
use std::time::Instant;
use memmap2::{MmapMut, MmapOptions};

pub const KIB: usize = 1024;
pub const MIB: usize = KIB * 1024;
pub const GIB: usize = MIB * 1024;

const D: usize = 1;


fn main() {
    println!("Array Size;Gops/s");
    for n in (4096..35*MIB).step_by(4096) {
        let k = 100_000_000 / n;

        let mut vec = MmapOptions::new().no_reserve_swap().populate().len(n).map_anon().unwrap();

        let start = Instant::now();

        for _ in 0..k {
            for i in (0..n).step_by(D) {
                vec[i] += 1;
            }
        }

        black_box(vec);

        let duration = start.elapsed();
        let v = 1e-9 * n as f64 * k as f64 / duration.as_secs_f64();
        println!("{n};{}", v);
    }
}
