# wavelet-matrix

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

## TODO

- Implement .top_n for WaveletMatrix
- Add prefix queries like `.prefix_rank()` and `.prefix_select()`
- impl SpaceUsage for WaveletMatrix
- Benchmark.
  - Implement same queries using trivial algorithm
  - Compare wm's queries against trivial one.
  - Make a nice plot.



