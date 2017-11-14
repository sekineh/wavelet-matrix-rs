use std::ops::Range;
use succinct::SpaceUsage;
use rsdic_simple::*;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Operator {
    Equal,
    LessThan,
    GreaterThan,
}

/// WaveletMatrix supports various near-O(1) queries on the sequence of integers.
#[derive(Debug)]
pub struct WaveletMatrix {
    layers: Vec<RsDic>,
    dim: u64, // = max vals + 1
    num: usize, // = layers[0].len()
    bit_len: u8, // = layers.len()
}

impl WaveletMatrix {
    /// Create a new WaveletMatrix struct from a input Vec<u64>.
    pub fn new(vals: &Vec<u64>) -> WaveletMatrix {
        let dim = get_dim(&vals);
        let bit_len = get_bit_len(dim);
        let num = vals.len();
        let mut zeros: Vec<u64> = vals.clone();
        let mut ones: Vec<u64> = Vec::new();
        let mut layers: Vec<RsDic> = Vec::new();

        for depth in 0..bit_len {
            let mut next_zeros: Vec<u64> = Vec::new();
            let mut next_ones: Vec<u64> = Vec::new();
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

    /// Returns the length of T
    #[inline]
    pub fn len(&self) -> usize {
        self.num
    }

    /// Returns the value T[pos]
    pub fn lookup(&self, pos: usize) -> u64 {
        let mut val: u64 = 0;
        let mut pos: usize = pos;

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

    /// Returns the number of the element which satisfies `e == value` included in A[pos_range]
    pub fn count(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Operator::Equal)
    }

    /// Returns the number of the element which satisfies `e < value` included in A[pos_range]
    pub fn count_lt(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Operator::LessThan)
    }

    /// Returns the number of the element which satisfies `e > value` included in A[pos_range]
    pub fn count_gt(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Operator::GreaterThan)
    }

    /// Returns the number of the element which satisfies `(e >> ignore_bit) == (val >> ignore_bit)` included in A[pos_range]
    pub fn count_prefix(&self, pos_range: Range<usize>, value: u64, ignore_bit: u8) -> usize {
        self.prefix_rank_op(pos_range, value, ignore_bit, Operator::Equal)
    }

    /// Returns the number of the element which satisfies `val_range.start <= e < val_range.end` included in A[pos_range]
    pub fn count_range(&self, pos_range: Range<usize>, val_range: Range<u64>) -> usize {
        self.count_lt(pos_range.clone(), val_range.end) - self.count_lt(pos_range, val_range.start)
    }

    /// Find the first index of the element which satisfies `e == value` included in `A[start..end]`
    fn find1st(&self, pos_range: Range<usize>, value: u64) -> Option<usize> {
        self.search(pos_range, value).next()
    }

    /// Returns the iterator that generates indexes that satisfies the condition `e == value`.
    pub fn search(&self, pos_range: Range<usize>, value: u64) -> WaveletMatrixSearch {
        self.search_prefix(pos_range, value, 0)
    }

    /// Returns the iterator that generates indexes that satisfies the condition `e >> ignore_bit == value >> ignore_bit`.
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

    /// Returns the number of val found in T[0..pos].
    ///
    /// The range specified is half open, i.e. [0, pos).
    pub fn rank(&self, pos: usize, val: u64) -> usize {
        self.prefix_rank_op(0..pos, val, 0, Operator::Equal)
    }

    /// .rank() with:
    /// - range support bpos..epos
    /// - prefix search support (ignore_bit)
    /// - operator support
    #[inline]
    fn prefix_rank_op(&self,
                      pos_range: Range<usize>,
                      val: u64,
                      ignore_bit: u8,
                      operator: Operator)
                      -> usize {
        let mut bpos = pos_range.start;
        let mut epos = pos_range.end;
        let mut rank = 0;

        if self.bit_len > ignore_bit {
            for depth in 0..self.bit_len - ignore_bit {
                let rsd = &self.layers[depth as usize];
                let bit = get_bit_msb(val, depth, self.bit_len);
                if bit {
                    if let Operator::LessThan = operator {
                        rank += rsd.rank(epos, false) - rsd.rank(bpos, false);
                    }
                    bpos = rsd.rank(bpos, bit) + rsd.zero_num();
                    epos = rsd.rank(epos, bit) + rsd.zero_num();
                } else {
                    if let Operator::GreaterThan = operator {
                        rank += rsd.rank(epos, true) - rsd.rank(bpos, true);
                    }
                    bpos = rsd.rank(bpos, bit);
                    epos = rsd.rank(epos, bit);
                }
            }
        }
        match operator {
            Operator::Equal => epos - bpos,
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
            pos = rsd.rank(pos, bit) + rsd.zero_num();
            rank = self.select_helper(rank, val, pos, depth + 1, ignore_bit) - rsd.zero_num();
        } else {
            pos = rsd.rank(pos, bit);
            rank = self.select_helper(rank, val, pos, depth + 1, ignore_bit);
        }
        rsd.select(rank, bit)
    }

    fn list_values(&self,
                   pos_range: Range<usize>,
                   val_range: Range<u64>,
                   k: usize)
                   -> Vec<ValueCount> {

        let min_c = val_range.start;
        let max_c = val_range.end;
        let beg_pos = pos_range.start;
        let end_pos = pos_range.end;
        let mut res: Vec<ValueCount> = Vec::new();

        if end_pos > self.len() || beg_pos >= end_pos {
            return res;
        }

        let mut qons = BinaryHeap::new();

        qons.push(QueryOnNode::new(0, self.len(), beg_pos, end_pos, 0, 0));

        while res.len() < self.num && !qons.is_empty() {
            let qon = qons.pop().unwrap();
            if qon.depth >= self.bit_len {
                res.push(ValueCount::new(qon.prefix_char, qon.end_pos - qon.beg_pos));
            } else {
                let next = self.expand_node(min_c, max_c, &qon);
                for i in 0..next.len() {
                    qons.push(next[i].clone());
                }
            }
        }
        res
    }

    fn expand_node(&self, min_c: u64, max_c: u64, qon: &QueryOnNode) -> Vec<QueryOnNode> {
        let ba = &self.layers[qon.depth as usize];
        let mut next = Vec::new();

        let beg_node_zero = ba.rank(qon.beg_node, false);
        let end_node_zero = ba.rank(qon.end_node, false);
        let beg_node_one = qon.beg_node - beg_node_zero;
        let beg_zero = ba.rank(qon.beg_pos, false);
        let end_zero = ba.rank(qon.end_pos, false);
        let beg_one = qon.beg_pos - beg_zero;
        let end_one = qon.end_pos - end_zero;
        let boundary = qon.beg_node + end_node_zero - beg_node_zero;

        if end_zero - beg_zero > 0 {
            // child for zero
            let next_prefix = qon.prefix_char << 1;
            if self.check_prefix(next_prefix, qon.depth + 1, min_c, max_c) {
                next.push(QueryOnNode::new(qon.beg_node,
                                           boundary,
                                           qon.beg_node + beg_zero - beg_node_zero,
                                           qon.beg_node + end_zero - beg_node_zero,
                                           qon.depth + 1,
                                           next_prefix));
            }
        }
        if end_one - beg_one > 0 {
            // child for one
            let next_prefix = (qon.prefix_char << 1) + 1;
            if self.check_prefix(next_prefix, qon.depth + 1, min_c, max_c) {
                next.push(QueryOnNode::new(boundary,
                                           qon.end_node,
                                           boundary + beg_one - beg_node_one,
                                           boundary + end_one - beg_node_one,
                                           qon.depth + 1,
                                           next_prefix));
            }
        }
        next
    }

    fn check_prefix(&self, prefix: u64, depth: u8, min_c: u64, max_c: u64) -> bool {
        Self::prefix_code(min_c, depth, self.bit_len) <= prefix &&
        Self::prefix_code(max_c - 1, depth, self.bit_len) >= prefix
    }

    fn prefix_code(x: u64, len: u8, bit_num: u8) -> u64 {
        x >> (bit_num - len)
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

#[derive(Debug, Clone, Eq)]
struct QueryOnNode {
    beg_node: usize,
    end_node: usize,
    beg_pos: usize,
    end_pos: usize,
    depth: u8,
    prefix_char: u64,
}

impl QueryOnNode {
    fn new(beg_node: usize,
           end_node: usize,
           beg_pos: usize,
           end_pos: usize,
           depth: u8,
           prefix_char: u64)
           -> Self {
        QueryOnNode {
            beg_node: beg_node,
            end_node: end_node,
            beg_pos: beg_pos,
            end_pos: end_pos,
            depth: depth,
            prefix_char: prefix_char,
        }
    }
}

impl Ord for QueryOnNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.end_pos - self.beg_pos != other.end_pos - other.beg_pos) {
            (self.end_pos - self.beg_pos).cmp(&(other.end_pos - other.beg_pos))
        } else if (self.depth != other.depth) {
            self.depth.cmp(&other.depth)
        } else {
            other.beg_pos.cmp(&self.beg_pos)
        }
    }
}

impl PartialOrd for QueryOnNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for QueryOnNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ValueCount {
    value: u64,
    count: usize,
}

impl ValueCount {
    fn new(value: u64, count: usize) -> Self {
        ValueCount {
            value: value,
            count: count,
        }
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

// Iterator struct used for the WaveletMatrix::search()
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
fn get_dim(vals: &[u64]) -> u64 {
    let mut dim: u64 = 0;
    for val in vals.iter() {
        if *val >= dim {
            dim = val + 1;
        }
    }
    dim
}

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
    extern crate num;

    use self::rand::distributions::range::SampleRange;
    use super::*;
    use self::rand::distributions;
    use wavelet_matrix::tests::rand::distributions::IndependentSample;
    // use super::std;
    // use self::rand::distributions;
    // use self::rand::Rng;

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

        // searching
        assert_eq!(wm.search(0..wm.len(), 4).collect::<Vec<usize>>(),
                   vec![2, 6]);
        assert_eq!(wm.search(3..wm.len(), 4).collect::<Vec<usize>>(), vec![6]);
        assert_eq!(wm.search(0..wm.len(), 7).collect::<Vec<usize>>(), vec![]);

        // statistics
        assert_eq!(wm.list_values(0..wm.len(), 0..wm.dim, 12), vec![]);

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

    const LEN: usize = 1_000;

    fn random_upto(max: u64) -> u64 {
        let range = distributions::range::Range::new(0, max);
        let mut rng = rand::thread_rng();
        range.ind_sample(&mut rng)
    }

    // fn random_upto<T>(max: T) -> T
    // where T: num::Unsigned
    // {
    //     let range = distributions::range::Range::new(0, max);
    //     let mut rng = rand::thread_rng();
    //     range.ind_sample(&mut rng)
    // }

    fn count_all(wm: &WaveletMatrix,
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

    fn search_all(wm: &WaveletMatrix,
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

            count_all(&wm, &vec, val, ignore_bit, range.clone());
            count_all(&wm, &vec, val + 1, ignore_bit, range.clone());

            if i == 0 {
                search_all(&wm, &vec, val, ignore_bit, range.clone());
                search_all(&wm, &vec, val + 1, ignore_bit, range.clone());
            }
        }
    }

    #[test]
    fn layers_64() {
        random_test(1024, -1i64 as u64);
        random_test(1023, -1i64 as u64);
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
}
