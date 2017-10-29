# wavelet-matrix

## Usage

Add to Cargo.toml:
```
[dependencies]
wavelet-matrix = "0.1.0"
```

## Features

### Basic WaveletMatrix operations

Given integer sequence A,
- .rank(pos, value): count value included in A[0..pos]
- .select(rank, value): return the position of the (rank+1)-th value

## TODO

- Implement .top_n for WaveletMatrix
- impl SpaceUsage for WaveletMatrix
- Benchmark.
  - Implement same queries using trivial algorithm
  - Compare wm's queries against trivial one.
  - Make a nice plot.



