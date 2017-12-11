extern crate easybench;
use easybench::{bench, bench_env, bench_limit};
use rand::Rng;
extern crate rand;
extern crate wavelet_matrix;

use wavelet_matrix::WaveletMatrix;
use rand::distributions::IndependentSample;

struct WaitCooldown {
    fastest: f64,
    wait: f64,
    ratio: f64,
}

impl WaitCooldown {
    fn new() -> WaitCooldown {
        let result = WaitCooldown::measure();
        let time = result.ns_per_iter;
        WaitCooldown {
            fastest: time as f64,
            wait: 1.0,
            ratio: 1.1,
        }
    }

    fn wait(&mut self) {
        loop {
            let result = WaitCooldown::measure();
            let time = result.ns_per_iter;
            if time > self.fastest * self.ratio {
                let dur = std::time::Duration::from_secs(self.wait as u64);
                std::thread::sleep(dur);
                self.wait *= self.ratio;
            } else if time < self.fastest {
                self.fastest = time;
                break;
            } else {
                break;
            }
        }

        self.wait = 1.0;
    }

    fn measure() -> easybench::Stats {
        let mut rng = rand::weak_rng();
        let num = std::u64::MAX;
        let stats = bench_env(rng.clone(), |rng| rng.gen_range(0, num));
        println!("(wait for cpu cooldown) {}", stats);
        stats
    }
}

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

    use std::time::*;
    let time_start = Instant::now();
    let wm = WaveletMatrix::new(&vec);
    let dur = time_start.elapsed();
    let dur_ns = dur.as_secs() as f64 * 1e9 + dur.subsec_nanos() as f64;
    println!(
        "{:>24}, N = {}, {}: {:>10} ns   (only 1 iteration)",
        "WaveletMatrix::new()",
        num,
        desc,
        dur_ns,
    );
    println!("{:>24}, N = {}, {}: {}",
             ".lookup(rand)",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let idx = rng.gen_range(0, num);
                 wm.lookup(idx)
             }));
    println!("{:>24}, N = {}, {}: {}",
             "vec[rand]",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let idx = rng.gen_range(0, num);
                 vec[idx]
             }));
    println!("{:>24}, N = {}, {}: {}",
             "rand only",
             num,
             desc,
             bench_env(rng.clone(), |rng| rng.gen_range(0, num)));
    println!("{:>24}, N = {}, {}: {}",
             ".rank()",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let pos = rng.gen_range(0, num);
                 let value = rng.gen_range(0, wm.dim());
                 wm.rank(pos, value)
             }));
    println!("{:>24}, N = {}, {}: {}",
             ".select()",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let rank = rng.gen_range(0, num);
                 let value = rng.gen_range(0, wm.dim());
                 wm.select(rank, value)
             }));
    println!("{:>24}, N = {}, {}: {}",
             ".count()",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let end = rng.gen_range(0, num);
                 let start = rng.gen_range(0, end);
                 let value = rng.gen_range(0, wm.dim());
                 wm.count(start..end, value)
             }));
    println!("{:>24}, N = {}, {}: {}",
             ".count_prefix()",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let end = rng.gen_range(0, num);
                 let start = rng.gen_range(0, end);
                 let value = rng.gen_range(0, wm.dim());
                 wm.count_prefix(start..end, value, wm.bit_len() / 2)
             }));
    println!("{:>24}, N = {}, {}: {}",
             ".search().next()",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let end = rng.gen_range(0, num);
                 let start = rng.gen_range(0, end);
                 let value = rng.gen_range(0, wm.dim());
                 wm.search(start..end, value).next()
             }));
    println!("{:>24}, N = {}, {}: {}",
             ".search_prefix().next()",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let end = rng.gen_range(0, num);
                 let start = rng.gen_range(0, end);
                 let value = rng.gen_range(0, wm.dim());
                 wm.search_prefix(start..end, value, wm.bit_len() / 2).next()
             }));
    println!("{:>24}, N = {}, {}: {}",
             ".median()",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let end = rng.gen_range(0, num);
                 let start = rng.gen_range(0, end);
                 wm.median(start..end)
             }));
    println!("{:>24}, N = {}, {}: {}",
             ".quantile()",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
                 let end = rng.gen_range(0, num);
                 let start = rng.gen_range(0, end);
                 let k = rng.gen_range(0, end - start);
                 wm.quantile(start..end, k)
             }));
    println!("{:>24}, N = {}, {}: {}",
             ".sum_ex3(k=16)",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
        let end = rng.gen_range(0, num);
        let start = rng.gen_range(0, end);
        let val_end = rng.gen_range(0, wm.dim());
        let val_start = rng.gen_range(0, val_end);
        let k = 16;
        wm.sum_experiment3(start..end, val_start..val_end, k)
    }));
    println!("{:>24}, N = {}, {}: {}",
             ".sum_ex3(k=256)",
             num,
             desc,
             bench_env(rng.clone(), |rng| {
        let end = rng.gen_range(0, num);
        let start = rng.gen_range(0, end);
        let val_end = rng.gen_range(0, wm.dim());
        let val_start = rng.gen_range(0, val_end);
        let k = 256;
        wm.sum_experiment3(start..end, val_start..val_end, k)
    }));
    println!("```");
    println!("");
}

fn overall_performance() {
    let long_test = true;

    let mut wcd = WaitCooldown::new();

    println!("# Overall Performance");
    println!("");

    println!("## 16-bit Values");
    println!("");

    if long_test {
        wcd.wait();
        overall_helper(10000000, "16-bit values", std::u16::MAX as u64, 50);
        wcd.wait();
        overall_helper(1000000, "16-bit values", std::u16::MAX as u64, 5);
    }
    wcd.wait();
    overall_helper(100000, "16-bit values", std::u16::MAX as u64, 1);
    wcd.wait();
    overall_helper(10000, "16-bit values", std::u16::MAX as u64, 1);
    wcd.wait();
    overall_helper(1000, "16-bit values", std::u16::MAX as u64, 1);

    println!("## 32-bit Values");
    println!();

    if long_test {
        wcd.wait();
        overall_helper(10000000, "32-bit values", std::u32::MAX as u64, 100);
        wcd.wait();
        overall_helper(1000000, "32-bit values", std::u32::MAX as u64, 10);
    }
    wcd.wait();
    overall_helper(100000, "32-bit values", std::u32::MAX as u64, 3);
    wcd.wait();
    overall_helper(10000, "32-bit values", std::u32::MAX as u64, 1);
    wcd.wait();
    overall_helper(1000, "32-bit values", std::u32::MAX as u64, 1);

    println!("## 64-bit Values");
    println!();

    if long_test {
        wcd.wait();
        overall_helper(10000000, "64-bit values", std::u64::MAX as u64, 200);
        wcd.wait();
        overall_helper(1000000, "64-bit values", std::u64::MAX as u64, 20);
    }
    wcd.wait();
    overall_helper(100000, "64-bit values", std::u64::MAX as u64, 6);
    wcd.wait();
    overall_helper(10000, "64-bit values", std::u64::MAX as u64, 1);
    wcd.wait();
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

/// perform test and find k for `accuracy_pct`
fn sum_uniform_find_sufficient_k(num: usize, lower: u64, upper: u64, accuracy_pct: f64) -> usize {
    println!("```");
    let mut errorpct = 0.0;

    let desc = format!("{} <= val < {}", lower, upper);
    let mut rng = rand::weak_rng();
    let range = rand::distributions::Range::new(lower, upper);
    let vec: Vec<u64> = std::iter::repeat(0)
        .take(num)
        .map(|_| range.ind_sample(&mut rng) as u64)
        .collect::<Vec<_>>();
    let wm = WaveletMatrix::new(&vec);

    let actual = vec.iter().sum::<u64>();
    println!("{:>24}, N = {}, {}: {:?}", "actual sum", num, desc, actual,);

    let mut k = 1;
    for _ in 0..100 {
        let computed = wm.sum_experiment3(0..wm.len(), 0..wm.dim(), k);
        let computed_sum = average(&computed.0);
        errorpct = error_pct(computed_sum, actual);
        println!(
            "{:>24}, N = {}, {}: {:?}, error = {}%, {}",
            format!(".sum_expreriment3(k={})", k),
            num,
            desc,
            computed,
            errorpct,
            "(bench omitted)" //bench(|| wm.sum_experiment3(0..wm.len(), 0..wm.dim(), k)),
        );
        if errorpct.abs() < accuracy_pct {
            break;
        } else {
            k += 1;
        }
    }

    println!("```");
    println!("");

    k
}

fn average(r: &std::ops::Range<u64>) -> u64 {
    (r.start + r.end) / 2
}

fn error_pct(computed: u64, actual: u64) -> f64 {
    let error = computed as f64 - actual as f64;
    error / (actual as f64) * 100.0
}

// use rand::distributions::range::SampleRange;

// fn random_upto<T: PartialOrd + SampleRange>(upper: u64) -> u64 {
//     rand::weak_rng().gen_range(0, upper)
// }

// fn random_range<T: PartialOrd + SampleRange>(lower: T, upper: T) -> T {
//     rand::weak_rng().gen_range(lower, upper)
// }

fn statistical_performance() {
    let num = 10000;
    println!("# Statistical Performance");
    println!("");
    println!("## Uniform distribution");
    println!("");
    println!("For uniform distribution, `.sum()` produce good results using smaller number of \
              `k`.");
    println!("");

    statstical_helper(num, 0, 256, 5);
    statstical_helper(num, 0, 257, 5);
    statstical_helper(num, 256, 1024, 5);
    statstical_helper(num, 255, 1024, 5);
    statstical_helper(num, 256, 1025, 5);

    statstical_helper(num, 400, 2000, 5);

    println!("### Some value ranges require greater k to achieve error < 1%");
    println!("");
    println!("#### Bad (k=5)");
    println!("");
    statstical_helper(num, 300, 1500, 5);

    println!("#### Good (k=10)");
    println!("");
    statstical_helper(num, 300, 1500, 10);
}

fn statistical_sufficient_k() {
    println!();
    println!("### Finding sufficient k that achieve error < 1% for various ranges");
    println!();

    let num = 1000;
    let trial = 100;
    let mut rng = rand::weak_rng();
    let mut ks = std::collections::BTreeMap::new();

    for _ in 0..trial {
        let a = rng.gen_range(0, 65536);
        let b = rng.gen_range(0, 65536);
        let lower = std::cmp::min(a, b);
        let upper = std::cmp::max(a, b);

        let k = sum_uniform_find_sufficient_k(num, lower, upper, 1.0);

        let k_entry = ks.entry(k).or_insert(0);
        *k_entry += 1;
    }
    println!("The distribution of sufficient k is:");
    println!();
    println!("```");
    for k in ks {
        println!("{:>2}: {:>2} {}",
                 k.0,
                 k.1,
                 std::iter::repeat('*').take(k.1).collect::<String>());
    }
    println!("```");
    println!();
}

fn main() {
    // BENCH.md
    overall_performance();

    // BENCH_SUM.md
    // statistical_performance();
    // statistical_sufficient_k();
}
