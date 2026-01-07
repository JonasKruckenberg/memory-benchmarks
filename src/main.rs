use std::hint::black_box;
use std::time::Instant;
use memmap2::{MmapMut, MmapOptions};
// const B: usize = 32;
//
// struct NodePool {}
//
// struct InternalNode {
//     keys: [u32; B],
//     children: [u32; B],
// }
// struct LeafNode {
//     keys: [u32; B],
// }

// const _: () = {
//     assert!(size_of::<InternalNode>() - (2 * 64) == 0);
//     assert!(size_of::<LeafNode>() - (2 * 64) == 0);
// };

pub const KIB: usize = 1024;
pub const MIB: usize = KIB * 1024;
pub const GIB: usize = MIB * 1024;

const D: usize = 1;


fn main() {
    println!("Array Size;Gops/s");
    for n in (4096..35*MIB).step_by(4096) {
        let k = 100_000_000 / n;

        let mut vec = MmapOptions::new().no_reserve_swap().populate().huge(Some(21)).len(n).map_anon().unwrap();

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
