// use std;
use std::ops::Range;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use succinct::SpaceUsage;
use rsdic_simple::*;
use node_range::*;

/// WaveletMatrix supports various near-O(1) queries on the sequence of integers.
#[derive(Debug)]
pub struct WaveletMatrix {
    layers: Vec<RsDic>,
    dim: u64, // = max vals + 1
    num: usize, // = layers[0].len()
    bit_len: u8, // = layers.len()
}

impl WaveletMatrix {
    /// Create a new WaveletMatrix struct from a Vec<u64> `vals`.
    /// 
    /// The values contained in `vals` should be less than `u64::MAX`.
    /// 
    /// ## Panics
    /// 
    /// It panics when `vals` include `u64::MAX`.
    pub fn new(vals: &Vec<u64>) -> WaveletMatrix {
        // let dim = get_dim(&vals);
        let max = *vals.iter().max().unwrap();
        if max == ::std::u64::MAX {
            panic!("WaveletMatrix::new(): Can't store u64::MAX");
        }
        let dim = max + 1;
        let bit_len = get_bit_len(dim);
        let num = vals.len();
        let mut zeros: Vec<u64> = vals.clone();
        let mut ones: Vec<u64> = Vec::new();
        let mut layers: Vec<RsDic> = Vec::new();

        for depth in 0..bit_len {
            let mut next_zeros: Vec<u64> = Vec::with_capacity(vals.len());
            let mut next_ones: Vec<u64> = Vec::with_capacity(vals.len());
            let mut rsd_ = RsDicBuilder::new();
            Self::filter(&zeros,
                         bit_len - depth - 1,
                         &mut next_zeros,
                         &mut next_ones,
                         &mut rsd_);
            Self::filter(&ones,
                         bit_len - depth - 1,
                         &mut next_zeros,
                         &mut next_ones,
                         &mut rsd_);
            zeros = next_zeros;
            ones = next_ones;
            layers.push(rsd_.build());
        }

        WaveletMatrix {
            layers: layers,
            dim: dim,
            num: num,
            bit_len: bit_len,
        }
    }

    fn filter(vals: &Vec<u64>,
              shift: u8,
              next_zeros: &mut Vec<u64>,
              next_ones: &mut Vec<u64>,
              rsd: &mut RsDicBuilder) {
        for val in vals {
            let bit = get_bit_lsb(*val, shift);
            rsd.push_bit(bit);
            if bit {
                next_ones.push(*val);
            } else {
                next_zeros.push(*val);
            }
        }
    }

    /// Returns the length of `T`
    #[inline]
    pub fn len(&self) -> usize {
        self.num
    }

    /// Returns the value at the position `pos`, i.e. `T[pos]`.
    /// 
    /// ## Panics
    /// 
    /// It panics when index `pos` is out of range.
    pub fn lookup(&self, pos: usize) -> u64 {
        let mut val: u64 = 0;
        let mut pos: usize = pos;

        self.validate_pos(pos);

        for depth in 0..self.bit_len as usize {
            let rsd = &self.layers[depth];
            let bit = rsd.access(pos);
            pos = rsd.rank(pos, bit);
            val <<= 1;
            if bit {
                pos += rsd.zero_num();
                val |= 1;
            }
        }
        val
    }

    /// Returns the maximum value + 1.
    ///
    /// Useful to get the upper bound of value range.
    #[inline]
    pub fn dim(&self) -> u64 {
        self.dim
    }

    /// Returns the bit length stored internally
    #[inline]
    pub fn bit_len(&self) -> u8 {
        self.bit_len
    }

    #[inline]
    fn validate_pos(&self, pos: usize) {
        if pos >= self.len() {
            panic!("pos: out of range");
        }
    }

    #[inline]
    fn validate_pos_range(&self, pos_range: &Range<usize>) {
        if pos_range.start >= self.len() {
            panic!("pos_range.start: out of range");            
        } else if pos_range.end > self.len() {
            panic!("pos_range.end: out of range");
        }            
    }

    /// Returns the number of the element `e` which satisfies `e == value` included in T[pos_range]
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn count(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Ordering::Equal)
    }

    /// Returns the number of the element `e` which satisfies `e < value` included in T[pos_range]
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn count_lt(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Ordering::Less)
    }

    /// Returns the number of the element `e` which satisfies `e > value` included in T[pos_range]
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn count_gt(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Ordering::Greater)
    }

    /// Returns the number of the element `e` which satisfies `(e >> ignore_bit) == (value >> ignore_bit)` included in T[pos_range]
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn count_prefix(&self, pos_range: Range<usize>, value: u64, ignore_bit: u8) -> usize {
        self.prefix_rank_op(pos_range, value, ignore_bit, Ordering::Equal)
    }

    /// Returns the number of the element `e` which satisfies `val_range.start <= e < val_range.end` included in T[pos_range]
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn count_range(&self, pos_range: Range<usize>, val_range: Range<u64>) -> usize {
        self.count_lt(pos_range.clone(), val_range.end) - self.count_lt(pos_range, val_range.start)
    }

    /// Returns the iterator that generates indices that satisfy the condition `e == value`.
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn search(&self, pos_range: Range<usize>, value: u64) -> WaveletMatrixSearch {
        self.validate_pos_range(&pos_range);
        self.search_prefix(pos_range, value, 0)
    }

    /// Returns the iterator that generates indices that satisfy the condition `e >> ignore_bit == value >> ignore_bit`.
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn search_prefix(&self,
                         pos_range: Range<usize>,
                         value: u64,
                         ignore_bit: u8)
                         -> WaveletMatrixSearch {
        let rank = self.count_prefix(0..pos_range.start, value, ignore_bit);
        WaveletMatrixSearch {
            inner: &self,
            range: pos_range,
            rank: rank,
            value: value,
            ignore_bit: ignore_bit,
        }
    }

    /// Returns the number of val found in `T[0..pos]`.
    ///
    /// The range specified is half open, i.e. `[0, pos)`.  When `pos` is 0, the range includes no value.
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos` is greater than len().
    pub fn rank(&self, pos: usize, value: u64) -> usize {
        self.prefix_rank_op(0..pos, value, 0, Ordering::Equal)
    }

    /// `.rank()` with:
    /// - range support bpos..epos
    /// - prefix search support (`ignore_bit`)
    /// - operator support
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    #[inline]
    fn prefix_rank_op(&self,
                      pos_range: Range<usize>,
                      val: u64,
                      ignore_bit: u8,
                      operator: Ordering)
                      -> usize {
        self.validate_pos_range(&pos_range);

        let mut bpos = pos_range.start;
        let mut epos = pos_range.end;
        let mut rank = 0;

        if self.bit_len > ignore_bit {
            for depth in 0..self.bit_len - ignore_bit {
                let rsd = &self.layers[depth as usize];
                let bit = get_bit_msb(val, depth, self.bit_len);
                if bit {
                    if let Ordering::Less = operator {
                        rank += rsd.rank(epos, !bit) - rsd.rank(bpos, !bit);
                    }
                    let zero_num = rsd.zero_num();
                    bpos = rsd.rank(bpos, bit) + zero_num;
                    epos = rsd.rank(epos, bit) + zero_num;
                } else {
                    if let Ordering::Greater = operator {
                        rank += rsd.rank(epos, !bit) - rsd.rank(bpos, !bit);
                    }
                    bpos = rsd.rank(bpos, bit);
                    epos = rsd.rank(epos, bit);
                }
            }
        }
        match operator {
            Ordering::Equal => epos - bpos,
            _ => rank,
        }
    }

    /// Return the position of (rank+1)-th val in T.
    ///
    /// If no match has been found, it returns the length of self.
    pub fn select(&self, rank: usize, val: u64) -> usize {
        self.select_helper(rank, val, 0, 0, 0)
    }

    /// ignore_bit: experimental support for prefix search
    fn select_helper(&self, rank: usize, val: u64, pos: usize, depth: u8, ignore_bit: u8) -> usize {
        if self.bit_len < ignore_bit || depth == self.bit_len - ignore_bit {
            return ::std::cmp::min(pos + rank, self.len());
        }
        let mut pos = pos;
        let mut rank = rank;

        let bit = get_bit_msb(val, depth, self.bit_len);
        let rsd = &self.layers[depth as usize];
        if bit {
            let zero_num = rsd.zero_num();
            pos = rsd.rank(pos, bit) + zero_num;
            rank = self.select_helper(rank, val, pos, depth + 1, ignore_bit) - zero_num;
        } else {
            pos = rsd.rank(pos, bit);
            rank = self.select_helper(rank, val, pos, depth + 1, ignore_bit);
        }
        rsd.select(rank, bit)
    }

    /// list the `(value, count)` pairs in most-frequent-one-first order.
    /// values are constrained to the `val_range`.
    /// 
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn top_k(&self,
                 pos_range: Range<usize>,
                 val_range: Range<u64>,
                 k: usize)
                 -> Vec<(u64, usize)> {
        self.values::<NodeRangeByFrequency>(pos_range, val_range, k)
    }

    /// list the `(range, count)` pairs in most-frequent-one-first order.
    /// values are constrained to the `val_range`.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use wavelet_matrix::WaveletMatrix;
    ///
    /// let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    /// //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    ///
    /// let wm = WaveletMatrix::new(&vec);
    /// assert_eq!(wm.top_k_ranges(0..wm.len(), 0..wm.dim(), 3), vec![(0..4, 7), (4..8, 4), (8..16, 1)]);
    /// // You can drill down into any specific range.
    /// assert_eq!(wm.top_k_ranges(0..wm.len(), 0..4,        3), vec![(2..4, 3), (1..2, 2), (0..1, 2)]);
    ///
    /// assert_eq!(wm.top_k_ranges(0..wm.len(), 0..wm.dim(), 4), vec![(0..2, 4), (4..8, 4), (2..4, 3), (8..16, 1)]);
    /// // You can drill down into any specific range.
    /// assert_eq!(wm.top_k_ranges(0..wm.len(), 4..8,        4), vec![(4..5, 2), (5..6, 1), (6..7, 1)]);
    /// assert_eq!(wm.top_k_ranges(0..wm.len(), 2..9,        4), vec![(2..4, 3), (4..6, 3), (6..8, 1), (8..16, 1)]);
    /// ```
    pub fn top_k_ranges(&self,
                        pos_range: Range<usize>,
                        val_range: Range<u64>,
                        k: usize)
                        -> Vec<(Range<u64>, usize)> {
        self.value_ranges::<NodeRangeByFrequency>(pos_range, val_range, k)
    }

    /// list the `(value, count)` pairs in descending order.
    /// values are constrained to the `val_range`.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn max_k(&self,
                 pos_range: Range<usize>,
                 val_range: Range<u64>,
                 k: usize)
                 -> Vec<(u64, usize)> {
        self.values::<NodeRangeDescending>(pos_range, val_range, k)
    }

    /// list the `(value, count)` pairs in ascending order.
    /// values are constrained to the `val_range`.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    pub fn min_k(&self,
                 pos_range: Range<usize>,
                 val_range: Range<u64>,
                 k: usize)
                 -> Vec<(u64, usize)> {
        self.values::<NodeRangeAscending>(pos_range, val_range, k)
    }

    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    fn values<N>(&self,
                 pos_range: Range<usize>,
                 val_range: Range<u64>,
                 k: usize)
                 -> Vec<(u64, usize)>
        where N: NodeRange
    {

        let mut res = Vec::new();

        self.validate_pos_range(&pos_range);
        // if pos_range.start > self.len() || pos_range.start >= pos_range.end {
        //     return res;
        // }

        let mut qons: BinaryHeap<N> = BinaryHeap::new();

        qons.push(NodeRange::new(pos_range, 0, 0, self.bit_len()));

        while res.len() < k && !qons.is_empty() {
            let qon = qons.pop().unwrap();
            let qon = qon.inner();
            if qon.depth == self.bit_len {
                res.push((qon.prefix_char, qon.pos_end - qon.pos_start));
            } else {
                let next = self.expand_node(val_range.clone(), &qon);
                for i in 0..next.len() {
                    qons.push(NodeRange::from(&next[i]));
                }
            }
        }
        res
    }

    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    fn value_ranges<N>(&self,
                       pos_range: Range<usize>,
                       val_range: Range<u64>,
                       k: usize)
                       -> Vec<(Range<u64>, usize)>
        where N: NodeRange
    {

        let mut res = Vec::new();

        self.validate_pos_range(&pos_range);
        // if pos_range.start > self.len() || pos_range.start >= pos_range.end {
        //     return res; // return the empty vector
        // }

        let mut qons: BinaryHeap<N> = BinaryHeap::new();

        qons.push(NodeRange::new(pos_range, 0, 0, self.bit_len()));

        while res.len() < k && !qons.is_empty() && qons.len() + res.len() < k {
            let qon = qons.pop().unwrap();
            let qon = qon.inner();
            if qon.depth == self.bit_len {
                res.push((qon.prefix_char..qon.prefix_char + 1, qon.pos_end - qon.pos_start));
            } else {
                let next = self.expand_node(val_range.clone(), &qon);
                for i in 0..next.len() {
                    qons.push(NodeRange::from(&next[i]));
                }
            }
        }
        while let Some(qon) = qons.pop() {
            let qon = qon.inner();
            let shift = self.bit_len - qon.depth;
            res.push((qon.prefix_char << shift..qon.prefix_char + 1 << shift,
                      qon.pos_end - qon.pos_start));
        }
        res
    }

    fn expand_node(&self, val_range: Range<u64>, qon: &QueryOnNode) -> Vec<QueryOnNode> {
        let ba = &self.layers[qon.depth as usize];
        let mut next = Vec::new();

        let bpos_zero = ba.rank(qon.pos_start, false);
        let epos_zero = ba.rank(qon.pos_end, false);

        let boundary = ba.zero_num();
        let bpos_one = ba.rank(qon.pos_start, true) + boundary;
        let epos_one = ba.rank(qon.pos_end, true) + boundary;

        if epos_zero > bpos_zero {
            // child for zero
            let next_prefix = qon.prefix_char << 1;
            if self.check_prefix(next_prefix, qon.depth + 1, val_range.clone()) {
                next.push(QueryOnNode::new(bpos_zero..epos_zero,
                                           next_prefix,
                                           qon.depth + 1,
                                           self.bit_len()));
            }
        }
        if epos_one > bpos_one {
            // child for one
            let next_prefix = (qon.prefix_char << 1) + 1;
            if self.check_prefix(next_prefix, qon.depth + 1, val_range) {
                next.push(QueryOnNode::new(bpos_one..epos_one,
                                           next_prefix,
                                           qon.depth + 1,
                                           self.bit_len()));
            }
        }
        next
    }

    fn check_prefix(&self, prefix: u64, depth: u8, val_range: Range<u64>) -> bool {
        Self::prefix_code(val_range.start, depth, self.bit_len) <= prefix &&
        prefix <= Self::prefix_code(val_range.end - 1, depth, self.bit_len)
    }

    fn prefix_code(x: u64, len: u8, bit_num: u8) -> u64 {
        x >> (bit_num - len)
    }

    /// Approximately calculates the sum of `T[pos_range]` using up to k wavelet tree nodes and
    /// returns `(sum of value, sum of count)` tuple.
    ///
    /// It enumerates the top-k most relevant node ranges using `.top_k_ranges()` function.
    ///
    /// To get the exact result, specfy k = m + 1 where m is the number of values which are unique.
    /// E.g. If you have 7 unique values in the sequence T, specify k = 8 to get the exact result.
    ///
    /// For calculation, this method uses u64 for summing.
    /// To avoid overflow during calculation, we recommend to use this function for < 32 bits values assuming the number of elements is ~ 32 bits.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use wavelet_matrix::WaveletMatrix;
    ///
    /// let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    /// //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    ///
    /// let wm = WaveletMatrix::new(&vec);
    /// // assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 0), (0, 0)); // useless
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 1), (96, 12));
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 2), (56, 12));
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 3), (50, 12));
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 4), (49, 12));
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 5), (47, 12));
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 6), (45, 12));
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 7), (40, 12));
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 8), (36, 12)); // got the exact sum
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 0..wm.dim(), 12), (36, 12));
    ///
    /// assert_eq!(wm.sum_experiment1(0..wm.len(), 6..7, 12), (6, 1));
    /// assert_eq!(wm.sum_experiment1(3..8,        4..7, 12), (15, 3));
    /// ```
    pub fn sum_experiment1(&self,
                           pos_range: Range<usize>,
                           val_range: Range<u64>,
                           k: usize)
                           -> (u64, usize) {
        let ranges = self.top_k_ranges(pos_range, val_range, k);

        let sum: u64 = ranges.iter()
            .map(|&(ref r, count)| {
                (r.start + r.end) / 2 // expected value 
                * (count as u64)
            })
            .sum();

        let count: usize = ranges.iter()
            .map(|&(ref _r, count)| count)
            .sum();

        (sum, count)
    }

    /// Improved version of `.sum_experiment1()`.
    ///
    /// It enumerates the top-k most relevant node ranges using custom node expander instead of `.top_k_ranges()`.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use wavelet_matrix::WaveletMatrix;
    ///
    /// let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    /// //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    ///
    /// let wm = WaveletMatrix::new(&vec);
    /// // assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 0), (0, 0)); // useless
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 1), (96, 12));
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 2), (56, 12));
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 3), (50, 12));
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 4), (49, 12));
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 5), (47, 12));
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 6), (43, 12)); // improved since experiment 1 (45->43)
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 7), (38, 12)); // improved since experiment 1 (40->38)
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 8), (36, 12)); // got the exact sum
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 0..wm.dim(), 12), (36, 12));
    ///
    /// assert_eq!(wm.sum_experiment2(0..wm.len(), 6..7, 12), (6, 1));
    /// assert_eq!(wm.sum_experiment2(3..8,        4..7, 12), (15, 3));
    /// ```
    pub fn sum_experiment2(&self,
                           pos_range: Range<usize>,
                           val_range: Range<u64>,
                           k: usize)
                           -> (u64, usize) {
        let ranges = self.value_ranges::<NodeRangeBySumError>(pos_range, val_range, k);

        let sum: u64 = ranges.iter()
            .map(|&(ref r, count)| {
                (r.start + r.end) / 2 // expected value 
                * (count as u64)
            })
            .sum();

        let count: usize = ranges.iter()
            .map(|&(ref _r, count)| count)
            .sum();

        (sum, count)
    }

    /// Improved version of `.sum_experiment2()`.
    ///
    /// It returns `Range<u64>` value to tell how accurate the computed value is.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use wavelet_matrix::WaveletMatrix;
    ///
    /// let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    /// //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    ///
    /// let wm = WaveletMatrix::new(&vec);
    /// // assert_eq!(wm.sum_experimen3(0..wm.len(), 0..wm.dim(), 0), (0, 0)); // useless
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 1), (0..181, 12)); // 181 / 2 = 90
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 2), (8..93, 12)); // 101 / 2 = 50
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 3), (24..65, 12)); // 89 / 2 = 44
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 4), (30..57, 12)); // 87 / 2 = 43
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 5), (32..51, 12)); // 83 / 2 = 41
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 6), (34..45, 12)); // 79 / 2 = 38
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 7), (35..40, 12)); // 75 / 2 = 37
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 8), (36..37, 12)); // got the exact sum = 36
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 0..wm.dim(), 12), (36..37, 12));
    ///
    /// assert_eq!(wm.sum_experiment3(0..wm.len(), 6..7, 12), (6..7, 1));
    /// assert_eq!(wm.sum_experiment3(3..8,        4..7, 12), (15..16, 3));
    /// ```
    pub fn sum_experiment3(&self,
                           pos_range: Range<usize>,
                           val_range: Range<u64>,
                           k: usize)
                           -> (Range<u64>, usize) {
        let ranges = self.value_ranges::<NodeRangeBySumError>(pos_range, val_range, k);

        let sum_min: u64 = ranges.iter()
            .map(|&(ref r, count)| r.start * (count as u64))
            .sum();

        let sum_max: u64 = ranges.iter()
            .map(|&(ref r, count)| (r.end - 1) * (count as u64))
            .sum();

        let count: usize = ranges.iter()
            .map(|&(ref _r, count)| count)
            .sum();

        (sum_min..sum_max + 1, count)
    }

    /// Approximately calculates the average of `T[pos_range]` using up to k wavelet tree nodes.
    ///
    /// It enumerates the top-k most relevant node ranges using `.top_k_ranges()` function.
    ///
    /// To get the exact result, specfy k = m + 1 where m is the number of values which are unique.
    /// E.g. If you have 256 unique values in the sequence T, specify k = 257 to get the exact result.
    ///
    /// For calculation, this method uses u64 for summing.
    /// To avoid overflow during calculation, we recommend to use this function for < 32 bits values assuming the number of elements is ~ 32 bits.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use wavelet_matrix::WaveletMatrix;
    ///
    /// let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    /// //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    ///
    /// let wm = WaveletMatrix::new(&vec);
    /// assert_eq!(wm.mean_experiment1(0..wm.len(), 0..wm.dim(), 1), 8);
    /// assert_eq!(wm.mean_experiment1(0..wm.len(), 0..wm.dim(), 2), 4);
    /// assert_eq!(wm.mean_experiment1(0..wm.len(), 0..wm.dim(), 3), 4);
    /// assert_eq!(wm.mean_experiment1(0..wm.len(), 0..wm.dim(), 4), 4);
    /// assert_eq!(wm.mean_experiment1(0..wm.len(), 0..wm.dim(), 5), 3); // got the actual average
    /// assert_eq!(wm.mean_experiment1(0..wm.len(), 0..wm.dim(), 12), 3);
    /// ```
    pub fn mean_experiment1(&self,
                            pos_range: Range<usize>,
                            val_range: Range<u64>,
                            k: usize)
                            -> u64 {
        let (sum, count) = self.sum_experiment1(pos_range, val_range, k);

        sum / (count as u64)
    }

    /// Approximately calculates the variance of `T[pos_range]` using up to k wavelet tree nodes.
    ///
    /// It enumerates the top-k most relevant node ranges using `.top_k_ranges()` function.
    ///
    /// To get the exact result, specfy k = m + 1 where m is the number of values which are unique.
    /// E.g. If you have 256 unique values in the sequence T, specify k = 257 to get the exact result.
    ///
    /// For calculation, this method uses u64 for summing.
    /// To avoid overflow during calculation, we recommend to use this function for < 16 bits values assuming the number of elements is ~ 32 bits.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use wavelet_matrix::WaveletMatrix;
    ///
    /// let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    /// //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    ///
    /// let wm = WaveletMatrix::new(&vec);
    /// let mean = wm.mean_experiment1(0..wm.len(), 0..wm.dim(), 8);
    /// assert_eq!(mean, 3); // got the actual average
    /// assert_eq!(wm.variance_experiment1(0..wm.len(), 0..wm.dim(), 8), 6);
    /// assert_eq!(vec.iter()
    ///               .map(|&v| {
    ///                   let diff = if v > mean {v - mean} else {mean - v};
    ///                   diff * diff
    ///               })
    ///               .sum::<u64>() / (vec.len() as u64), 6);
    /// ```
    pub fn variance_experiment1(&self,
                                pos_range: Range<usize>,
                                val_range: Range<u64>,
                                k: usize)
                                -> u64 {
        let ranges = self.top_k_ranges(pos_range, val_range, k);

        let sum: u64 = ranges.iter()
            .map(|&(ref r, count)| {
                (r.start + r.end) / 2 // expected value 
                * (count as u64)
            })
            .sum();

        let count: usize = ranges.iter()
            .map(|&(ref _r, count)| count)
            .sum();

        let mean = sum / count as u64;

        let variance: u64 = ranges.iter()
            .map(|&(ref r, count)| {
                let expected = (r.start + r.end) / 2;
                let diff = if expected > mean {
                    expected - mean
                } else {
                    mean - expected
                };
                diff * diff * (count as u64)
            })
            .sum::<u64>() / count as u64;

        variance
    }

    /// returns the median value from ``T[pos_range]``.
    ///
    /// same as `.quantile(start..end, start + (end - start) / 2)`.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use wavelet_matrix::WaveletMatrix;
    ///
    /// let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    /// //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    ///
    /// let wm = WaveletMatrix::new(&vec);
    /// assert_eq!(wm.median(0..wm.len()), 2);  // median
    /// assert_eq!(wm.median(6..wm.len()), 4);  // median
    ///
    /// // Let's compare with the sorted vector version
    /// let mut sorted = vec.clone();
    /// sorted.sort(); // O(n log n)
    /// assert_eq!(wm.median(0..wm.len()), sorted[wm.len() / 2]);
    /// ```
    pub fn median(&self, pos_range: Range<usize>) -> u64 {
        self.quantile(pos_range.clone(), (pos_range.end - pos_range.start) / 2)
    }

    /// Returns the (k+1)th smallest value from `T[pos_range]`.
    ///
    /// ## Panics
    /// 
    /// It panics when `pos_range` is out of range.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use wavelet_matrix::WaveletMatrix;
    ///
    /// let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
    /// //                       0  1  2  3  4  5  6  7  8  9 10 11 (length = 12)
    ///
    /// let wm = WaveletMatrix::new(&vec);
    /// assert_eq!(wm.quantile(0..wm.len(), 0), 0);  // min
    /// assert_eq!(wm.quantile(0..wm.len(), 3), 1);
    /// assert_eq!(wm.quantile(0..wm.len(), 6), 2);  // median
    /// assert_eq!(wm.quantile(0..wm.len(), 11), 9); // max
    /// // assert_eq!(wm.quantile(0..wm.len(), 12), 9); // out of range
    ///
    /// // Let's compare with the sorted vector
    /// let mut sorted = vec.clone();
    /// sorted.sort(); // O(n log n)
    /// for i in 0..sorted.len() {
    ///     assert_eq!(wm.quantile(0..wm.len(), i), sorted[i]);
    /// }
    /// ```
    pub fn quantile(&self, pos_range: Range<usize>, k: usize) -> u64 {
        self.validate_pos_range(&pos_range);

        let mut bpos = pos_range.start;
        let mut epos = pos_range.end;
        let mut value: u64 = 0;
        let mut k = k;

        for depth in 0..self.bit_len {
            let rsd = &self.layers[depth as usize];
            value = value << 1;

            let zeros_bpos = rsd.rank(bpos, false);
            let zeros_epos = rsd.rank(epos, false);
            let zeros_rank = zeros_epos - zeros_bpos;

            if k < zeros_rank {
                // the value is included in zero's node.
                // take zero's node in the next loop
                bpos = zeros_bpos;
                epos = zeros_epos;
            } else {
                // the value is included in one's node.
                k -= zeros_rank; // the count of zeros are consumed
                value |= 1;

                // take one's node in the next loop
                bpos = (bpos - zeros_bpos) + rsd.zero_num();
                //     ^^^^^^^^^^^^^^^^^^^ rsd.rank(bpos, true)
                epos = (epos - zeros_epos) + rsd.zero_num();
                //     ^^^^^^^^^^^^^^^^^^^ rsd.rank(epos, true)
            }
        }
        value
    }
}

fn get_bit_msb(x: u64, pos: u8, blen: u8) -> bool {
    ((x >> (blen - pos - 1)) & 1) == 1
}

fn get_bit_lsb(x: u64, pos: u8) -> bool {
    ((x >> pos) & 1) == 1
}

impl SpaceUsage for WaveletMatrix {
    fn is_stack_only() -> bool {
        false
    }

    fn heap_bytes(&self) -> usize {
        self.layers.iter().map(|x| x.heap_bytes()).sum()
    }
}


/// Thin builder that builds WaveletMatrix
#[derive(Debug)]
pub struct WaveletMatrixBuilder {
    vals: Vec<u64>,
}

impl WaveletMatrixBuilder {
    /// Create builder.
    pub fn new() -> WaveletMatrixBuilder {
        WaveletMatrixBuilder { vals: Vec::new() }
    }

    /// append to the internal Vec.
    pub fn push(&mut self, val: u64) {
        self.vals.push(val);
    }

    /// Build creates WaveletMatrix from this builder.
    /// It takes self, so the original builder won't be accessible later.
    pub fn build(self) -> WaveletMatrix {
        WaveletMatrix::new(&self.vals)
    }
}

/// Iterator struct used by the WaveletMatrix::search()
#[derive(Debug)]
pub struct WaveletMatrixSearch<'a> {
    inner: &'a WaveletMatrix, // underlying Wavelet Matrix
    range: Range<usize>, // index range to be searched
    rank: usize, // the next rank
    value: u64, // value to be searched
    ignore_bit: u8, // used in prefix search
}

impl<'a> Iterator for WaveletMatrixSearch<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.inner.select_helper(self.rank, self.value, 0, 0, self.ignore_bit);
        self.rank += 1;
        if pos < self.range.end {
            Some(pos)
        } else {
            None
        }
    }
}

// Note:
// If max of vals is 0xffff_ffff_ffff_ffff (max u64),
// return value will overflow u64.
// fn get_dim(vals: &[u64]) -> u64 {
//     let mut dim: u64 = 0;
//     for val in vals.iter() {
//         if *val >= dim {
//             dim = *val + 1;
//         }
//     }
//     dim
// }

fn get_bit_len(val: u64) -> u8 {
    let mut blen: u8 = 0;
    let mut val = val;
    while val > 0 {
        val >>= 1;
        blen += 1;
    }
    blen
}


#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate itertools;

    use super::*;
    use wavelet_matrix::tests::rand::Rng;

    #[test]
    fn example() {
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

        assert_eq!(wm.max_k(0..wm.len(), 0..10, 12),
                   vec![(9, 1), (6, 1), (5, 1), (4, 2), (2, 3), (1, 2), (0, 2)]);
        assert_eq!(wm.max_k(0..wm.len(), 0..10, 4),
                   vec![(9, 1), (6, 1), (5, 1), (4, 2)]);
        assert_eq!(wm.max_k(0..wm.len(), 2..9, 12),
                   vec![(6, 1), (5, 1), (4, 2), (2, 3)]);

        assert_eq!(wm.min_k(0..wm.len(), 0..10, 12),
                   vec![(0, 2), (1, 2), (2, 3), (4, 2), (5, 1), (6, 1), (9, 1)]);
        assert_eq!(wm.min_k(0..wm.len(), 0..10, 4),
                   vec![(0, 2), (1, 2), (2, 3), (4, 2)]);
        assert_eq!(wm.min_k(0..wm.len(), 2..9, 12),
                   vec![(2, 3), (4, 2), (5, 1), (6, 1)]);

        // classic .rank()/.select() API
        assert_eq!(wm.rank(5, 1), 2);
        assert_eq!(wm.select(2, 2), 10);
    }

    #[test]
    fn wavelet_matrix_sanity() {
        let mut wmb = WaveletMatrixBuilder::new();
        wmb.push(1);
        wmb.push(31);
        wmb.push(11);
        wmb.push(10);
        wmb.push(11);

        let wm = wmb.build();
        assert_eq!(wm.lookup(0), 1);
        assert_eq!(wm.lookup(1), 31);
        assert_eq!(wm.lookup(2), 11);
        assert_eq!(wm.lookup(3), 10);
        assert_eq!(wm.lookup(4), 11);
        // assert_eq!(wm.lookup(5), 11);

        assert_eq!(wm.count(0..wm.len(), 11), 2);
        assert_eq!(wm.count(0..wm.len(), 10), 1);
        assert_eq!(wm.count(0..wm.len(), 5), 0);

        assert_eq!(wm.rank(0, 1), 0);
        assert_eq!(wm.rank(1, 1), 1);
        assert_eq!(wm.rank(4, 1), 1);

        assert_eq!(wm.rank(0, 31), 0);
        assert_eq!(wm.rank(1, 31), 0);
        assert_eq!(wm.rank(2, 31), 1);
        assert_eq!(wm.rank(3, 31), 1);
        assert_eq!(wm.rank(4, 31), 1);

        assert_eq!(wm.select(0, 1), 0);
        assert_eq!(wm.select(0, 31), 1);
        assert_eq!(wm.select(0, 11), 2);
        assert_eq!(wm.select(1, 11), 4);
        assert_eq!(wm.select(2, 11), 5);
        assert_eq!(wm.select(3, 11), 5);

        // assert_eq!(wm.total_bytes(), 336); // Only true for now with x64
    }

    fn random_upto(max: u64) -> u64 {
        let mut rng = rand::weak_rng();
        if max != 0 {
            rng.gen_range(0, max)            
        } else {
            rng.gen_range(0, 1)
        }
    }

    fn test_count_all(wm: &WaveletMatrix,
                      vec: &Vec<u64>,
                      val: u64,
                      ignore_bit: u8,
                      range: Range<usize>) {

        assert_eq!(wm.count(range.clone(), val),
                   vec[range.clone()].iter().filter(|x| **x == val).count());

        assert_eq!(wm.count_prefix(range.clone(), val, ignore_bit),
                   vec[range.clone()]
                       .iter()
                       .filter(|x| (**x >> ignore_bit) == (val >> ignore_bit))
                       .count());

        assert_eq!(wm.count_lt(range.clone(), val),
                   vec[range.clone()].iter().filter(|x| **x < val).count());

        assert_eq!(wm.count_gt(range.clone(), val),
                   vec[range.clone()].iter().filter(|x| **x > val).count());
    }

    fn test_search_all(wm: &WaveletMatrix,
                       vec: &Vec<u64>,
                       val: u64,
                       ignore_bit: u8,
                       range: Range<usize>) {

        assert_eq!(wm.search(range.clone(), val).collect::<Vec<usize>>(),
                   vec[range.clone()]
                       .iter()
                       .enumerate()
                       .filter(|x| *x.1 == val)
                       .map(|x| x.0 + range.start)
                       .collect::<Vec<usize>>());

        assert_eq!(wm.search_prefix(range.clone(), val, ignore_bit).collect::<Vec<usize>>(),
                   vec[range.clone()]
                       .iter()
                       .enumerate()
                       .filter(|x| *x.1 >> ignore_bit == val >> ignore_bit)
                       .map(|x| x.0 + range.start)
                       .collect::<Vec<usize>>());
    }

    fn test_statistics_all(wm: &WaveletMatrix,
                       vec: &Vec<u64>,
                       range: Range<usize>) {

        let mut sorted: Vec<u64> = vec[range.clone()].iter().map(|x| *x).collect();
        sorted.sort();

        if sorted.len() > 0 {
            let k = random_upto(sorted.len() as u64) as usize;

            assert_eq!(wm.quantile(range.clone(), k), sorted[k]);
            assert_eq!(wm.median(range.clone()), sorted[sorted.len() / 2]);
        }
    }

    use std::collections::BTreeMap;
    fn value_count(vec: &[u64]) -> BTreeMap<u64, (usize, usize)> {
        let mut map = BTreeMap::new();
        for pos in 0..vec.len() {
            let entry = map.entry(vec[pos]).or_insert((0, pos));
            entry.0 = entry.0 + 1; // count += 1
        }
        map // value => (count, 1st pos)
    }

    use self::itertools::Itertools;
    fn test_ranking(wm: &WaveletMatrix,
                    vec: &Vec<u64>,
                    vals: Range<u64>,
                    range: Range<usize>,
                    len: usize) {

        assert_eq!(wm.min_k(range.clone(), vals.clone(), len),
                   value_count(&vec[range.clone()]).iter()
                       .into_iter()
                       .filter(|&(value, _count)| vals.start <= *value && *value < vals.end)
                       .take(len)
                       .map(|k| (*k.0, (k.1).0)) // value, count
                       .collect::<Vec<_>>());

        assert_eq!(wm.max_k(range.clone(), vals.clone(), len),
                   value_count(&vec[range.clone()]).iter()
                       .into_iter()
                       .filter(|&(value, _count)| vals.start <= *value && *value < vals.end)
                       .rev()
                       .take(len)
                       .map(|k| (*k.0, (k.1).0)) // (value, count)
                       .collect::<Vec<_>>());

        assert_eq!(wm.top_k(range.clone(), vals.clone(), len)
                       .iter()
                       .map(|&(_value, count)| count) // Note: currently only count is tested because value order is not predictable
                       .collect::<Vec<_>>(),
                   value_count(&vec[range.clone()]).iter()
                       .into_iter()
                       .filter(|&(value, _count)| vals.start <= *value && *value < vals.end)
                       .sorted_by(|a, b| a.1.cmp(&b.1).reverse() ) // ordered by (count, first pos)
                       .iter()
                       .take(len)
                       .map(|k| (*k.0, (k.1).0)) // (value, count)
                       .collect::<Vec<_>>()
                       .iter()
                       .map(|&(_value, count)| count) // Note: currently only count is tested because value order is not predictable
                       .collect::<Vec<_>>());
    }

    fn random_test(len: usize, val_max: u64) {
        let mut vec: Vec<u64> = Vec::new();
        for _ in 0..len {
            vec.push(random_upto(val_max));
        }
        let wm = WaveletMatrix::new(&vec);

        assert_eq!(wm.dim, *vec.iter().max().unwrap() + 1);
        assert_eq!(wm.num, len);
        assert_eq!(wm.len(), len);

        for i in 0..100 {
            let idx = random_upto(wm.len() as u64) as usize;
            assert_eq!(wm.lookup(idx), vec[idx]);

            let val = vec[idx];
            let ignore_bit = random_upto(wm.bit_len as u64) as u8;
            let a = random_upto(wm.len() as u64) as usize;
            let b = random_upto(wm.len() as u64) as usize;
            let range = ::std::cmp::min(a, b)..::std::cmp::max(a, b);

            test_count_all(&wm, &vec, val, ignore_bit, range.clone());
            test_count_all(&wm, &vec, val + 1, ignore_bit, range.clone());

            test_statistics_all(&wm, &vec, range.clone());

            if i == 0 {
                test_search_all(&wm, &vec, val, ignore_bit, range.clone());
                test_search_all(&wm, &vec, val + 1, ignore_bit, range.clone());

                let a = random_upto(wm.dim as u64) as u64;
                let b = random_upto(wm.dim as u64) as u64;
                let val_range = ::std::cmp::min(a, b)..::std::cmp::max(a, b);
                test_ranking(&wm, &vec, val_range.clone(), range.clone(), 10);
                test_ranking(&wm, &vec, val_range.clone(), range.clone(), 10000);
            }
        }
    }

    #[test]
    fn layers_64() {
        random_test(1024, ::std::u64::MAX);
        random_test(1023, ::std::u64::MAX - 1);
    }
    #[test]
    fn layers_7() {
        random_test(1024, 128);
        random_test(1023, 127);
    }
    #[test]
    fn layers_4() {
        random_test(10240, 16);
        random_test(10231, 15);
    }
    #[test]
    #[should_panic]
    fn new_panic() {
        let vals: Vec<u64> = vec![::std::u64::MAX];
        let _wm = WaveletMatrix::new(&vals);
    }
    #[test]
    #[should_panic]
    fn lookup_panic() {
        let vals: Vec<u64> = vec![0,1,2];
        let wm = WaveletMatrix::new(&vals);
        let _ = wm.lookup(3);
    }
    #[test]
    #[should_panic]
    fn count_panic_start() {
        let vals: Vec<u64> = vec![0,1,2];
        let wm = WaveletMatrix::new(&vals);
        let _ = wm.count(3..3, 1);
    }
    #[test]
    #[should_panic]
    fn count_panic_end() {
        let vals: Vec<u64> = vec![0,1,2];
        let wm = WaveletMatrix::new(&vals);
        let _ = wm.count(0..4, 1);
    }
}
