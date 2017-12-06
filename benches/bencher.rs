#![allow(non_snake_case)]
// #[macro_use]
// extern crate bencher;
#![feature(test)]
extern crate rand;
extern crate test;
extern crate wavelet_matrix;

// use bencher::Bencher;
use test::Bencher;
use wavelet_matrix::WaveletMatrix;
use rand::distributions::IndependentSample;

fn new_helper(b: &mut Bencher, num: usize, upper: u64, bytes_per_elem: u64) {
    let mut rng = rand::weak_rng();
    let range = rand::distributions::Range::new(0, upper);
    let vec: Vec<u64> = std::iter::repeat(0)
        .take(num)
        .map(|_| range.ind_sample(&mut rng) as u64)
        .collect::<Vec<_>>();

    b.iter(|| WaveletMatrix::new(&vec));

    b.bytes = bytes_per_elem * num as u64;
}

/// bench new() with various bit length
mod new_by_bits {
    use ::*;

    #[bench]
    fn new_8bit_10000(b: &mut Bencher) {
        new_helper(b, 10000, std::u8::MAX as u64, 1)
    }

    #[bench]
    fn new_16bit_10000(b: &mut Bencher) {
        new_helper(b, 10000, std::u16::MAX as u64, 2)
    }

    #[bench]
    fn new_32bit_10000(b: &mut Bencher) {
        new_helper(b, 10000, std::u32::MAX as u64, 4)
    }

    #[bench]
    fn new_64bit_10000(b: &mut Bencher) {
        new_helper(b, 10000, std::u64::MAX as u64, 8)
    }
}

/// bench new() with various number of elements
mod new_by_N {
    use ::*;

    #[bench]
    fn new_16bit____1000(b: &mut Bencher) {
        new_helper(b, 1000, std::u16::MAX as u64, 2)
    }

    #[bench]
    fn new_16bit___10000(b: &mut Bencher) {
        new_helper(b, 10000, std::u16::MAX as u64, 2)
    }

    #[bench]
    fn new_16bit__100000(b: &mut Bencher) {
        new_helper(b, 100000, std::u16::MAX as u64, 2)
    }

    #[bench]
    fn new_16bit_1000000(b: &mut Bencher) {
        new_helper(b, 1000000, std::u16::MAX as u64, 2)
    }
}

// benchmark_group!(
//     new_for_different_sizes,
//     new_8bitx10000,
//     new_16bitx10000,
//     new_32bitx10000,
//     new_64bitx10000
// );

// benchmark_group!(
//     new_for_different_nums,
//     new_32bitx1000,
//     new_32bitx10000,
//     new_32bitx100000
// );

// benchmark_main!(new_for_different_sizes, new_for_different_nums);
