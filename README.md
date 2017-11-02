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

- `.count_value(pos_range, value)`: returns the number of the element which satisfies `e == value` included in A[pos_range]

### Classical WaveletMatrix operations

- `.rank(pos, value)`: counts value included in A[0..pos]. 
  - Note: pos is exclusive. When pos is 0, .rank() always returns 0.
- `.select(rank, value)`: return the position of the (rank+1)-th value
  - Note: When found nothing, it returns .len() instead of None.

### Advanced queries

- to be added

## Releases 

### v0.4.0
- [INCOMPATIBLE] WaveletMatrix::new() takes &Vec<u64>, instead of Vec<u64>

### v0.3.0
- [INCOMPATIBLE] .select() now returns .len() instead of None.

### TODO

- Implement .top_n for WaveletMatrix
- Add prefix queries like `.prefix_rank()` and `.prefix_select()`. They will be much faster and useful for IPv4 prefix search (eg. searching for 192.168.12.0/24).
- Add less_than queries like `.rank_less_than()` and the variations with the other operators such as `equal`, `greater_than`
- Add ranged queries like `.ranged_rank()` and `.ranged_select()`. 
- We may need to come up with the better name for those queries, though.
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
