//! This crate provides state-of-the-art O(1) or alike queries on large number of unsigned integers.
//!
//! The typical memory usage of this data structure is calculated like below.
//!
//! - `bit_len * num_of_elements / 8 * 1.25` [bytes]
//!
//! In other words, it roughly consumes additional 25% space compared with the original data.
//!
//! # Example
//!
//! ```rust
//! use wavelet_matrix::WaveletMatrix;
//!
//! let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
//! //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
//! let wm = WaveletMatrix::new(&vec);
//!
//! // Basic access
//! assert_eq!(wm.len(), vec.len());
//! assert_eq!(wm.lookup(7), vec[7]);
//! assert_eq!(wm.dim(), 10); // max value + 1
//! assert_eq!(wm.bit_len(), 4); // bit length stored internally
//!
//! // Counting
//! assert_eq!(wm.count(0..wm.len(), 2), 3);
//! assert_eq!(wm.count(0..wm.len(), 4), 2);
//! assert_eq!(wm.count(0..wm.len(), 5), 1);
//! assert_eq!(wm.count(0..wm.len(), 7), 0);
//! assert_eq!(wm.count(0..wm.len(), 39), 0);
//!
//! assert_eq!(wm.count_prefix(0..wm.len(), 8, 3), 1);
//! assert_eq!(wm.count_prefix(0..wm.len(), 6, 1), 1);
//! assert_eq!(wm.count_prefix(0..wm.len(), 0, 1), 4);
//! assert_eq!(wm.count_prefix(0..wm.len(), 0, 2), 7);
//!
//! assert_eq!(wm.count_lt(0..wm.len(), 2), 4);
//! assert_eq!(wm.count_lt(0..wm.len(), 7), 11);
//!
//! assert_eq!(wm.count_gt(0..wm.len(), 2), 5);
//! assert_eq!(wm.count_gt(0..wm.len(), 7), 1);
//!
//! assert_eq!(wm.count_range(0..wm.len(), 0..wm.dim()), 12);
//! assert_eq!(wm.count_range(0..wm.len(), 4..6), 3);
//!
//! // Searching
//! assert_eq!(wm.search(0..wm.len(), 4).collect::<Vec<usize>>(), vec![2, 6]);
//! assert_eq!(wm.search(3..wm.len(), 4).collect::<Vec<usize>>(), vec![6]);
//! assert_eq!(wm.search(0..wm.len(), 7).collect::<Vec<usize>>(), vec![]);
//!
//! // Ranking: (value, count), frequent values first
//! assert_eq!(wm.top_k(0..wm.len(), 0..wm.dim(), 12),
//!            vec![(2, 3), (1, 2), (4, 2), (0, 2), (5, 1), (6, 1), (9, 1)]);
//! assert_eq!(wm.top_k(0..wm.len(), 0..wm.dim(), 4),
//!            vec![(2, 3), (1, 2), (4, 2), (0, 2)]);
//! assert_eq!(wm.top_k(0..wm.len(), 2..9, 12),
//!            vec![(2, 3), (4, 2), (5, 1), (6, 1)]);
//!
//! // Ranking: (value, count), max values first
//! assert_eq!(wm.max_k(0..wm.len(), 0..wm.dim(), 12),
//!            vec![(9, 1), (6, 1), (5, 1), (4, 2), (2, 3), (1, 2), (0, 2)]);
//! assert_eq!(wm.max_k(0..wm.len(), 0..wm.dim(), 4),
//!            vec![(9, 1), (6, 1), (5, 1), (4, 2)]);
//! assert_eq!(wm.max_k(0..wm.len(), 2..9, 12),
//!            vec![(6, 1), (5, 1), (4, 2), (2, 3)]);
//!
//! // Ranking: (value, count), min values first
//! assert_eq!(wm.min_k(0..wm.len(), 0..wm.dim(), 12),
//!            vec![(0, 2), (1, 2), (2, 3), (4, 2), (5, 1), (6, 1), (9, 1)]);
//! assert_eq!(wm.min_k(0..wm.len(), 0..wm.dim(), 4),
//!            vec![(0, 2), (1, 2), (2, 3), (4, 2)]);
//! assert_eq!(wm.min_k(0..wm.len(), 2..9, 12),
//!            vec![(2, 3), (4, 2), (5, 1), (6, 1)]);
//!
//! // Statistics
//! assert_eq!(wm.quantile(0..wm.len(), 0), 0);
//! assert_eq!(wm.quantile(0..wm.len(), 8), 4);
//! assert_eq!(wm.quantile(0..wm.len(), 11), 9);
//! ```

extern crate succinct;

mod rsdic_simple;
mod node_range;
mod wavelet_matrix;

pub use crate::wavelet_matrix::*;
