extern crate easybench;
use easybench::{bench, bench_env, bench_limit};
use rand::Rng;
extern crate rand;
extern crate wavelet_matrix;

use wavelet_matrix::WaveletMatrix;
use rand::distributions::IndependentSample;

fn helper(num: usize, desc: &str, upper: u64, limit_secs: u64) {
    let mut rng = rand::weak_rng();
    let range = rand::distributions::Range::new(0, upper);
    let vec: Vec<u64> = std::iter::repeat(0)
        .take(num)
        .map(|_| range.ind_sample(&mut rng) as u64)
        .collect::<Vec<_>>();

    let wm = WaveletMatrix::new(&vec);
    println!(
        "WaveletMatrix::new(), N = {}, {}: {}",
        num,
        desc,
        bench_limit(|| WaveletMatrix::new(&vec), limit_secs)
    );
    println!(
        "           .lookup(), N = {}, {}: {}",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let idx = rng.next_u64() as usize % num;
            wm.lookup(idx)
        })
    );
    println!(
        "               vec[], N = {}, {}: {}",
        num,
        desc,
        bench_env(0 as usize, |idx| {
            *idx += 1;
            vec[*idx]
        })
    );
    println!(
        "             .rank(), N = {}, {}: {}",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let pos = rng.next_u64() as usize % num;
            let value = rng.next_u64() % wm.dim();
            wm.rank(pos, value)
        })
    );
    println!(
        "           .select(), N = {}, {}: {}",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let pos = rng.next_u64() as usize % 10;
            let value = rng.next_u64() % wm.dim();
            wm.select(pos, value)
        })
    );
}

fn main() {
    helper(1000000, "32-bit values", std::u32::MAX as u64, 10);
    helper(100000, "32-bit values", std::u32::MAX as u64, 3);
    helper(10000, "32-bit values", std::u32::MAX as u64, 1);
    helper(1000, "32-bit values", std::u32::MAX as u64, 1);

    helper(1000000, "64-bit values", std::u64::MAX as u64, 20);
    helper(100000, "64-bit values", std::u64::MAX as u64, 6);
    helper(10000, "64-bit values", std::u64::MAX as u64, 1);
    helper(1000, "64-bit values", std::u64::MAX as u64, 1);
}
