# wavelet-matrix crate for the Rust language

The Wavelet Matrix is a space-efficient variant of Wavelet Tree data structure.
- https://en.wikipedia.org/wiki/Wavelet_Tree

## Usage

After adding to Cargo.toml, add this line to lib.rs or main.rs.
```
extern crate wavelet_matrix;
```

## Example

Add to main.rs:
```
use wavelet_matrix::*;

fn main() {
    let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    //                       0  1  2  3  4  5  6  7  8  9 10 11
    let wm = WaveletMatrix::new(&vec);
    assert_eq!(wm.len(), 12);
    assert_eq!(wm.lookup(7), 6);
    assert_eq!(wm.rank(5, 1), 2);
    assert_eq!(wm.select(2, 2), 10);

    println!("Worked as expected!");
}
```

## Features

Given unsigned integer sequence A, it provides the following queries.

### Basic operations

- `.len()`: returns the length of A.
- `.lookup(pos)`: returns the value at the position pos of A, A[pos].

### Counting

- `.count(pos_range, value)`: returns the number of the element which satisfies `e == value` included in A[pos_range]
- `.count_prefix(pos_range, value, ignore_bit)`: returns the number of the element which satisfies `e >> ignore_bit == value >> ignore_bit` included in A[pos_range]
  - This will be useful for counting the number of IPv4 prefix such as `172.22.0.0/16`
- `.count_lt(pos_range, value)`: returns the number of the element which satisfies `e < value` included in A[pos_range]
- `.count_gt(pos_range, value)`: returns the number of the element which satisfies `e > value` included in A[pos_range]
- `.count_range(pos_range, val_range)`: returns the number of the element which satisfies `val_range.start <= e < val_range.end` included in A[pos_range]

### Classical WaveletMatrix operations

- `.rank(pos, value)`: counts value included in A[0..pos]. 
  - Note: pos is exclusive. When pos is 0, .rank() always returns 0.
- `.select(rank, value)`: return the position of the (rank+1)-th value
  - Note: When found nothing, it returns .len() instead of None.

### Advanced queries

- to be added

## Releases 

### v0.4.0
- Add `.count()`, `.count_prefix()`, `.count_lt()`, `.count_gt()` and `.count_range()`.
- [INCOMPATIBLE] WaveletMatrix::new() takes &Vec<u64>, instead of Vec<u64>

### v0.3.0
- [INCOMPATIBLE] .select() now returns .len() instead of None.

### TODO

- Implement .top_n for WaveletMatrix
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
