extern crate easybench;
use easybench::{bench, bench_env};

extern crate rand;
extern crate wavelet_matrix;

use wavelet_matrix::WaveletMatrix;
use rand::distributions::IndependentSample;

fn helper(num: usize, upper: u64) {
    let mut rng = rand::weak_rng();
    let range = rand::distributions::Range::new(0, upper);
    let vec: Vec<u64> = std::iter::repeat(0)
        .take(num)
        .map(|_| range.ind_sample(&mut rng) as u64)
        .collect::<Vec<_>>();

    println!(
        "WaveletMatrix::new(), N = {}, values < {}:\t {}",
        num,
        upper,
        bench(|| WaveletMatrix::new(&vec))
    );
}

fn main() {
    helper(1000000, std::u32::MAX as u64);
    helper(100000, std::u32::MAX as u64);
    helper(10000, std::u32::MAX as u64);
    helper(1000, std::u32::MAX as u64);

    helper(1000000, std::u64::MAX as u64);
    helper(100000, std::u64::MAX as u64);
    helper(10000, std::u64::MAX as u64);
    helper(1000, std::u64::MAX as u64);
}
