# wavelet-matrix crate for the Rust language

The Wavelet Matrix is a space-efficient variant of Wavelet Tree data structure.
- https://en.wikipedia.org/wiki/Wavelet_Tree

## Usage

Add to Cargo.toml:
```
[dependencies]
wavelet-matrix = "0.2.0"
```

Add to lib.rs or main.rs
```
extern crate wavelet_matrix; // Note: underscore is used here
```
## Example

Add to main.rs:
```
use wavelet_matrix::*;

fn main() {
    let vec = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];

    let wm = WaveletMatrix::new(vec);
    assert_eq!(wm.len(), 12);
    assert_eq!(wm.lookup(7), 6);
    assert_eq!(wm.rank(5, 1), 2);
    assert_eq!(wm.select(2, 2), Some(10));

    println!("Worked as expected!");
}
```

## Features

### Basic WaveletMatrix operations

Given unsigned integer sequence A, it provides the following queries: 
- `.access(pos)`: returns the value of A[pos]
- `.rank(pos, value)`: count value included in A[0..pos]
- `.select(rank, value)`: return the position of the (rank+1)-th value

### Advanced queries

- to be added

## TODO

- Implement .top_n for WaveletMatrix
- Add prefix queries like `.prefix_rank()` and `.prefix_select()`
- Add less_than queries like `.rank_less_than()` and the variations with the other operators such as `equal`, `greater_than`
- Add ranged queries like `.ranged_rank()` and `.ranged_select()`. 
- We may need to come up with the better name for those queries, though.
- impl SpaceUsage for WaveletMatrix
- Add Benchmark.
  - Implement same queries using trivial algorithm
  - Compare wm's queries against trivial one.
  - Make a nice plot.
- Profiling
- Optimize underlying rsdic structure.
- Add travis CI.

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
