# Wavelet Matrix for Rust language

It provides the various analytics on very large sequence of unsigned integers in constant time.

## Usage

After adding to Cargo.toml, add this line to lib.rs or main.rs.

```rust
extern crate wavelet_matrix;
```

See crate document top for further examples.

- https://docs.rs/wavelet-matrix/

## Features

Given an unsigned integer sequence T, it provides the following queries.

### Basic operations

- `.len()`:
  - returns the length of T.
- `.lookup(pos)`:
  - returns the value at the position of T, T[pos].

### Counting

Counting is performed in O(1).

- `.count(start..end, value)`:
  - returns the number of the element which satisfies `e == value` included in `T[start..end]`
- `.count_prefix(start..end, value, ignore_bit)`:
  - returns the number of the element which satisfies `e >> ignore_bit == value >> ignore_bit` included in `T[start..end]`
  - This will be useful for counting the number of IPv4 address that satisfies IPv4 prefix such as `192.168.10.0/24`. In this case, the ignore_bit will be 8.
- `.count_lt(start..end, value)`:
  - returns the number of the element which satisfies `e < value` included in `T[start..end]`
- `.count_gt(start..end, value)`:
  - returns the number of the element which satisfies `e > value` included in `T[start..end]`
- `.count_range(start..end, val_start..val_end)`:
  - returns the number of the element which satisfies `val_start <= e < val_end` included in `T[start..end]`

### Searching

Searching is performed in O(1) per a next index.

- `.search(start..end, value)`:
  - returns the iterator that find indexes of the element which satisfies `e == value` included in `T[start..end]`
- `.search_prefix(start..end, value, ignore_bit)`:
  - returns the iterator that find indexes of the element which satisfies `e >> ignore_bit == value >> ignore_bit` included in `T[start..end]`
- [TODO] implement various conditions other than equal.

### Ranking

Ranking is performed in roughly O(k), where k is the number of `(value, count)` tuples.

- `.top_k(start..end, val_start..val_end, k)`:
  - list the (value, count) pairs in most-frequent-one-first order.
  - values are constrained to the range `val_start..val_end`.
  - [TODO] implement iterator based on this.
  - [TODO] extensive testing.
  - [TODO] clarify the order of same count.
- `.max_k(start..end, val_start..val_end, k)`:
  - list the (value, count) pairs in descending order.
  - values are constrained to the range `val_start..val_end`.
- `.min_k(start..end, val_start..val_end, k)`:
  - list the (value, count) pairs in ascending order.
  - values are constrained to the range `val_start..val_end`.
- Should we implement the above functions using iterator interface?

### Statistics

- [TODO] `.median(start..end)`:
  - returns the median value from `T[start..end]`.
  - same as `.quantile(start..end, start + (end - start) / 2)`.
- [TODO] `.quantile(start..end, k)`:
  - returns the (k+1)th smallest value from `T[start..end]`.

### Classical WaveletMatrix operations

- `.rank(pos, value)`: counts value included in T[0..pos].
  - Note: pos is exclusive. When pos is 0, .rank() always returns 0.
- `.select(rank, value)`: return the position of the (rank+1)-th value
  - Note: When found nothing, it returns .len() instead of None.

## Releases

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
