# wavelet-matrix crate for the Rust language

The Wavelet Matrix is a space-efficient variant of Wavelet Tree data structure.

- https://en.wikipedia.org/wiki/Wavelet_Tree

## Usage

After adding to Cargo.toml, add this line to lib.rs or main.rs.

```rust
extern crate wavelet_matrix;
```

## Example

Add to main.rs:

```rust
use wavelet_matrix::*;

fn main() {
    let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    let wm = WaveletMatrix::new(&vec);

    assert_eq!(wm.len(), 12);
    assert_eq!(wm.lookup(7), 6);

    // Counting
    assert_eq!(wm.count(0..wm.len(), 2), 3);
    assert_eq!(wm.count(0..wm.len(), 4), 2);
    assert_eq!(wm.count(0..wm.len(), 5), 1);
    assert_eq!(wm.count(0..wm.len(), 7), 0);
    assert_eq!(wm.count(0..wm.len(), 39), 0);

    assert_eq!(wm.count_prefix(0..wm.len(), 8, 3), 1);
    assert_eq!(wm.count_prefix(0..wm.len(), 6, 1), 1);
    assert_eq!(wm.count_prefix(0..wm.len(), 0, 1), 4);
    assert_eq!(wm.count_prefix(0..wm.len(), 0, 2), 7);

    assert_eq!(wm.count_lt(0..wm.len(), 2), 4);
    assert_eq!(wm.count_lt(0..wm.len(), 7), 11);

    assert_eq!(wm.count_gt(0..wm.len(), 2), 5);
    assert_eq!(wm.count_gt(0..wm.len(), 7), 1);

    assert_eq!(wm.count_range(0..wm.len(), 0..10), 12);
    assert_eq!(wm.count_range(0..wm.len(), 4..6), 3);

    // Searching
    assert_eq!(wm.search(0..wm.len(), 4).collect::<Vec<usize>>(),
                vec![2, 6]);
    assert_eq!(wm.search(3..wm.len(), 4).collect::<Vec<usize>>(), vec![6]);
    assert_eq!(wm.search(0..wm.len(), 7).collect::<Vec<usize>>(), vec![]);

    // Ranking
    assert_eq!(wm.top_k(0..wm.len(), 0..10, 12),
                vec![(2, 3), (1, 2), (4, 2), (0, 2), (5, 1), (6, 1), (9, 1)]);
    assert_eq!(wm.top_k(0..wm.len(), 0..10, 4),
                vec![(2, 3), (1, 2), (4, 2), (0, 2)]);
    assert_eq!(wm.top_k(0..wm.len(), 2..9, 12),
                vec![(2, 3), (4, 2), (5, 1), (6, 1)]);

    // classic .rank()/.select() API
    assert_eq!(wm.rank(5, 1), 2);
    assert_eq!(wm.select(2, 2), 10);
}
```

## Features

Given an unsigned integer sequence A, it provides the following queries.

### Basic operations

- `.len()`:
  - returns the length of A.
- `.lookup(pos)`:
  - returns the value at the position of A, A[pos].

### Counting

- `.count(start..end, value)`:
  - returns the number of the element which satisfies `e == value` included in `A[start..end]`
- `.count_prefix(start..end, value, ignore_bit)`:
  - returns the number of the element which satisfies `e >> ignore_bit == value >> ignore_bit` included in `A[start..end]`
  - This will be useful for counting the number of IPv4 address that satisfies IPv4 prefix such as `192.168.10.0/24`. In this case, the ignore_bit will be 8.
- `.count_lt(start..end, value)`:
  - returns the number of the element which satisfies `e < value` included in `A[start..end]`
- `.count_gt(start..end, value)`:
  - returns the number of the element which satisfies `e > value` included in `A[start..end]`
- `.count_range(start..end, val_start..val_end)`:
  - returns the number of the element which satisfies `val_start <= e < val_end` included in `A[start..end]`

### Searching

- `.search(start..end, value)`:
  - returns the iterator that find indexes of the element which satisfies `e == value` included in `A[start..end]`
- `.search_prefix(start..end, value, ignore_bit)`:
  - returns the iterator that find indexes of the element which satisfies `e >> ignore_bit == value >> ignore_bit` included in `A[start..end]`
- [TODO] implement various conditions other than equal.

### Ranking

- [EXPERIMENTAL] `.top_k(start..end, val_start..val_end, k)`:
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

### Classical WaveletMatrix operations

- `.rank(pos, value)`: counts value included in A[0..pos].
  - Note: pos is exclusive. When pos is 0, .rank() always returns 0.
- `.select(rank, value)`: return the position of the (rank+1)-th value
  - Note: When found nothing, it returns .len() instead of None.

## Releases

### v0.4.3?

- doc update

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
