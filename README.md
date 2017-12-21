# Wavelet Matrix for Rust language

[![Build Status](https://travis-ci.org/sekineh/wavelet-matrix-rs.svg?branch=master)](https://travis-ci.org/sekineh/wavelet-matrix-rs)

It provides the various analytics on very large sequence of unsigned integers in constant time.

## Usage

After adding to Cargo.toml, add this line to lib.rs or main.rs.

```rust
extern crate wavelet_matrix;
```

See crate document top for further examples.

- https://docs.rs/wavelet-matrix/

## Benchmarks

- Overall Performance
  - https://github.com/sekineh/wavelet-matrix-rs/blob/master/BENCH.md
  - Bench is performed on Intel Xeon 2800 MHz

- Error evaluation of O(1) SUM
  - https://github.com/sekineh/wavelet-matrix-rs/blob/master/BENCH_SUM.md

## Features

Given an unsigned integer sequence T, it provides the following queries.

### Basic operations

- `.len()`:
  - Returns the length of `T`.
  - It's almost no overhead, just returning the stored value.
- `.lookup(pos)`:
  - Returns the value at the position of T, `T[pos]`.
  - It's slower than array lookup but still is O(1). You might want to save the original Vector for faster lookup.

### Counting

Counting is performed in O(1).

- `.count(start..end, value)`:
  - Returns the number of the element `e` which satisfies `e == value` included in `T[start..end]`
- `.count_prefix(start..end, value, ignore_bit)`:
  - Returns the number of the element `e` which satisfies `e >> ignore_bit == value >> ignore_bit` included in `T[start..end]`
  - This will be useful for counting the number of IPv4 address that satisfies IPv4 prefix such as `192.168.10.0/24`. In this case, the ignore_bit will be 8.
- `.count_lt(start..end, value)`:
  - Returns the number of the element `e` which satisfies `e < value` included in `T[start..end]`
- `.count_gt(start..end, value)`:
  - Returns the number of the element `e` which satisfies `e > value` included in `T[start..end]`
- `.count_range(start..end, val_start..val_end)`:
  - Returns the number of the element `e` which satisfies `val_start <= e < val_end` included in `T[start..end]`

### Searching

Searching is performed in O(1) per a next index.

- `.search(start..end, value)`:
  - Returns the iterator that find indexes of the element `e` which satisfies `e == value` included in `T[start..end]`
- `.search_prefix(start..end, value, ignore_bit)`:
  - Returns the iterator that find indexes of the element `e` which satisfies `e >> ignore_bit == value >> ignore_bit` included in `T[start..end]`
- [TODO] implement various conditions other than equal.

### Ranking

Ranking is performed in roughly O(1) with regard to the number of elements `n`.

- `.max_k(start..end, val_start..val_end, k)`:
  - list the `(value, count)` pairs in descending order.
  - values are constrained to the range `val_start..val_end`.
- `.min_k(start..end, val_start..val_end, k)`:
  - list the `(value, count)` pairs in ascending order.
  - values are constrained to the range `val_start..val_end`.

`.top_k()` is also performed in O(1) in best case, but may take O(n) in the worst case where every value occurs only once!

- `.top_k(start..end, val_start..val_end, k)`:
  - list the `(value, count)` pairs in most-frequent-one-first order.
  - values are constrained to the range `val_start..val_end`.
  - [TODO] clarify the order of same count.

To achieve O(1) performance regardless of the number of unique values, use `.top_k_ranges()` instead:

- [EXPERIMENTAL] `.top_k_ranges(start..end, val_start..val_end, k)`:
  - list the `(v_start..v_end, count)` pairs in most-frequent-one-first order.
  - unlike `.top_k()`, `.top_k_ranges()` returns the exhaustive range set that covers all of the values.
  - values are constrained to the range `val_start..val_end`.
  - [TODO] clarify the order of same count.

### Statistics

#### O(1) Median / O(1) Quantile

- [EXPERIMENTAL] `.median(start..end)`:
  - Returns the median value from `T[start..end]`.
  - same as `.quantile(start..end, start + (end - start) / 2)`.
- [EXPERIMENTAL] `.quantile(start..end, k)`:
  - Returns the (k+1)th smallest value from `T[start..end]`.

#### O(1) Sum / O(1) Average / O(1) Variance

##### Experiment 1

These methods use `.top_k_ranges()` to enumerate the most relevant value ranges.  

They are not as accurate as the method used in Experiment 2.

- [EXPERIMENTAL] `.sum_experiment1(start..end, val_start..val_end, k)`:
  - Approximately calculate the sum of `T[start..end]` using up to `k` wavelet tree nodes.
  - Only values included in the range `val_start..val_end` are processed.
  - To get the exact result, specify `k = m + 1` where `m` is the number of values which are unique.
- [EXPERIMENTAL] `.mean_experiment1(start..end, val_start..val_end, k)`:
  - Approximately calculate the average of `T[start..end]` using up to `k` wavelet tree nodes.
  - Only values included in the range `val_start..val_end` are processed.
  - To get the exact result, specify `k = m + 1` where `m` is the number of values which are unique.
- [EXPERIMENTAL] `.variance_experiment1(start..end, val_start..val_end, k)`:
  - Approximately calculate the variance of `T[start..end]` using up to `k` wavelet tree nodes.
  - Only values included in the range `val_start..val_end` are processed.
  - To get the exact result, specify `k = m + 1` where `m` is the number of values which are unique.

##### Experiment 2

Improvement over Experiment 1.  They use custom node enumerator to minimize the error.

- [EXPERIMENTAL] `.sum_experiment2(start..end, val_start..val_end, k)`:

##### Experiment 3

Improvement over Experiment 2.  They use `Range<u64>` to tell how accurate the computed value is.

- [EXPERIMENTAL] `.sum_experiment3(start..end, val_start..val_end, k)`:

### Classical WaveletMatrix operations

- `.rank(pos, value)`: counts value included in `T[0..pos]`.
  - Note: pos is exclusive. When pos is 0, `.rank()` always returns 0.
- `.select(rank, value)`: return the position of the (rank+1)-th value
  - Note: When found nothing, it returns `.len()` instead of None.

## Releases

### v0.4.4

- Add `.median()` and `.quantile()`. They are quite fast, only take 3-5 us on 16-bit values.
- Add `.top_k_ranges()` which is faster than `.top_k()` in worst case.

- Add `.sum_experiment1()`, `.mean_experiment1()` and `.variance_experiment1()`.
- Add `.sum_experiment2()`.
- Add `.sum_experiment3()`.

- Add BENCH.md bench report.
- Add BENCH_SUM.md bench report.

### v0.4.3

- Add `.bit_len()`.
- Add `.dim()`.
- Move examples to crate's document top.
- Add test for `.top_k()`, `.max_k()` and `min_k()`.
- Suppress warnings.

### v0.4.2

- Add `.top_k()`, `.max_k()` and `min_k()`.

### v0.4.1

- Add `.search()` and `.search_prefix()`.

### v0.4.0

- Add `.count()`, `.count_prefix()`, `.count_lt()`, `.count_gt()` and `.count_range()`.
- [INCOMPATIBLE] WaveletMatrix::new() takes `&Vec<u64>`, instead of `Vec<u64>`

### v0.3.0

- [INCOMPATIBLE] .select() now returns .len() instead of None.

### TODO

- Add Benchmark.
  - Implement same queries using trivial algorithm
  - Compare wm's queries against trivial one.
  - Make a nice plot.
- Profiling
- Optimize underlying rsdic structure.
- Add travis CI.
- Add u128 support or arbitrary-length integer support

- The fastest implementation on the planet

## Credits

- A Go package for myriad array operations using wavelet trees
  - https://github.com/hillbig/waveletTree
  - Basically, the algorithm is deeply derived from the above Go implementation.

- excellent slides in Japanese
  - https://www.slideshare.net/pfi/ss-15916040
  - https://www.slideshare.net/bonprosoft/the-wavelet-matrix

- The original inventor's pdf:
  - http://www.dcc.uchile.cl/~gnavarro/ps/spire12.4.pdf
