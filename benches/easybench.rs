extern crate easybench;
use easybench::{bench, bench_env, bench_limit};
use rand::Rng;
extern crate rand;
extern crate wavelet_matrix;

use wavelet_matrix::WaveletMatrix;
use rand::distributions::IndependentSample;

fn overall_helper(num: usize, desc: &str, upper: u64, limit_secs: u64) {
    let mut rng = rand::weak_rng();
    let range = rand::distributions::Range::new(0, upper);
    let vec: Vec<u64> = std::iter::repeat(0)
        .take(num)
        .map(|_| range.ind_sample(&mut rng) as u64)
        .collect::<Vec<_>>();

    println!("### N = {}", num);
    println!("");
    println!("```");

    let wm = WaveletMatrix::new(&vec);
    println!(
        "{:>24}, N = {}, {}: {}",
        "WaveletMatrix::new()",
        num,
        desc,
        bench_limit(|| WaveletMatrix::new(&vec), limit_secs)
    );
    println!(
        "{:>24}, N = {}, {}: {}",
        ".lookup()",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let idx = rng.next_u64() as usize % num;
            wm.lookup(idx)
        })
    );
    println!(
        "{:>24}, N = {}, {}: {}",
        "vec[]",
        num,
        desc,
        bench_env(0 as usize, |idx| {
            *idx += 1;
            vec[*idx]
        })
    );
    println!(
        "{:>24}, N = {}, {}: {}",
        ".rank()",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let pos = rng.next_u64() as usize % num;
            let value = rng.next_u64() % wm.dim();
            wm.rank(pos, value)
        })
    );
    println!(
        "{:>24}, N = {}, {}: {}",
        ".select()",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let pos = rng.next_u64() as usize % 10;
            let value = rng.next_u64() % wm.dim();
            wm.select(pos, value)
        })
    );
    println!(
        "{:>24}, N = {}, {}: {}",
        ".count()",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let pos = rng.next_u64() as usize % 10;
            let value = rng.next_u64() % wm.dim();
            wm.count(0..pos, value)
        })
    );
    println!(
        "{:>24}, N = {}, {}: {}",
        ".count_prefix()",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let pos = rng.next_u64() as usize % 10;
            let value = rng.next_u64() % wm.dim();
            wm.count_prefix(0..pos, value, wm.bit_len() / 2)
        })
    );
    println!(
        "{:>24}, N = {}, {}: {}",
        ".search().next()",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let pos = rng.next_u64() as usize % 10;
            let value = rng.next_u64() % wm.dim();
            wm.search(0..pos, value).next()
        })
    );
    println!(
        "{:>24}, N = {}, {}: {}",
        ".search_prefix().next()",
        num,
        desc,
        bench_env(rng.clone(), |rng| {
            let pos = rng.next_u64() as usize % 10;
            let value = rng.next_u64() % wm.dim();
            wm.search_prefix(0..pos, value, wm.bit_len() / 2).next()
        })
    );
    println!("```");
    println!("");
}

fn overall_performance() {
    let long_test = true;

    println!("# Overall Performance");
    println!("");

    println!("## 16-bit Values");
    println!("");

    if long_test {
        overall_helper(10000000, "16-bit values", std::u16::MAX as u64, 50);
        overall_helper(1000000, "16-bit values", std::u16::MAX as u64, 5);
    }
    overall_helper(100000, "16-bit values", std::u16::MAX as u64, 1);
    overall_helper(10000, "16-bit values", std::u16::MAX as u64, 1);
    overall_helper(1000, "16-bit values", std::u16::MAX as u64, 1);

    println!("## 32-bit Values");
    println!();

    if long_test {
        overall_helper(10000000, "32-bit values", std::u32::MAX as u64, 100);
        overall_helper(1000000, "32-bit values", std::u32::MAX as u64, 10);
    }
    overall_helper(100000, "32-bit values", std::u32::MAX as u64, 3);
    overall_helper(10000, "32-bit values", std::u32::MAX as u64, 1);
    overall_helper(1000, "32-bit values", std::u32::MAX as u64, 1);

    println!("## 64-bit Values");
    println!();

    if long_test {
        overall_helper(10000000, "64-bit values", std::u64::MAX as u64, 200);
        overall_helper(1000000, "64-bit values", std::u64::MAX as u64, 20);
    }
    overall_helper(100000, "64-bit values", std::u64::MAX as u64, 6);
    overall_helper(10000, "64-bit values", std::u64::MAX as u64, 1);
    overall_helper(1000, "64-bit values", std::u64::MAX as u64, 1);
}

fn statstical_helper(num: usize, lower: u64, upper: u64, k: usize) {
    println!("```");

    let desc = format!("{} <= val < {}", lower, upper);
    let mut rng = rand::weak_rng();
    let range = rand::distributions::Range::new(lower, upper);
    let vec: Vec<u64> = std::iter::repeat(0)
        .take(num)
        .map(|_| range.ind_sample(&mut rng) as u64)
        .collect::<Vec<_>>();
    let wm = WaveletMatrix::new(&vec);

    if wm.dim() < std::u32::MAX as u64 {
        // let k = 5;

        let actual = vec.iter().sum::<u64>();
        println!(
            "{:>24}, N = {}, {}: {:?}",
            "actual sum",
            num,
            desc,
            actual,
            // bench(|| vec.iter().sum::<u64>())
        );

        let computed = wm.sum_experiment1(0..wm.len(), 0..wm.dim(), k);
        let computed_sum = computed.0;
        println!(
            "{:>24}, N = {}, {}: {:?}, error = {}%, {}",
            format!(".sum_expreriment1(k={})", k),
            num,
            desc,
            computed,
            error_pct(computed_sum, actual),
            bench(|| wm.sum_experiment1(0..wm.len(), 0..wm.dim(), k)),
        );

        let computed = wm.sum_experiment2(0..wm.len(), 0..wm.dim(), k);
        let computed_sum = computed.0;
        println!(
            "{:>24}, N = {}, {}: {:?}, error = {}%, {}",
            format!(".sum_expreriment2(k={})", k),
            num,
            desc,
            computed,
            error_pct(computed_sum, actual),
            bench(|| wm.sum_experiment2(0..wm.len(), 0..wm.dim(), k)),
        );

        let computed = wm.sum_experiment3(0..wm.len(), 0..wm.dim(), k);
        let computed_sum = average(&computed.0);
        println!(
            "{:>24}, N = {}, {}: {:?}, error = {}%, {}",
            format!(".sum_expreriment3(k={})", k),
            num,
            desc,
            computed,
            error_pct(computed_sum, actual),
            bench(|| wm.sum_experiment3(0..wm.len(), 0..wm.dim(), k)),
        );
    }
    println!("```");
    println!("");
}

fn average(r: &std::ops::Range<u64>) -> u64 {
    (r.start + r.end) / 2
}

fn error_pct(computed: u64, actual: u64) -> f64 {
    let error = computed as f64 - actual as f64;
    error / (actual as f64) * 100.0
}

fn statistical_performance() {
    println!("# Statistical Performance");
    println!("");
    println!("## Uniform distribution");
    println!("");
    println!("For uniform distribution, `.sum()` produce good results using smaller number of `k`.");
    println!("");

    statstical_helper(10000, 0, 256, 5);
    statstical_helper(10000, 0, 257, 5);
    statstical_helper(10000, 256, 1024, 5);
    statstical_helper(10000, 255, 1024, 5);
    statstical_helper(10000, 256, 1025, 5);

    statstical_helper(10000, 400 , 2000, 5);

    println!("### Some value ranges require greater k to achieve error < 1%");
    println!("");
    println!("#### Bad (k=5)");
    println!("");
    statstical_helper(10000, 300 , 1500, 5);

    println!("#### Good (k=10)");
    println!("");
    statstical_helper(10000, 300 , 1500, 10);
}

fn main() {
    overall_performance();
    statistical_performance();
}
