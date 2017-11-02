use std::ops::Range;
use succinct::SpaceUsage;
use rsdic_simple::*;

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
    dim: u64,    // = max vals + 1
    num: usize,  // = layers[0].len()
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
            Self::filter(
                &zeros,
                bit_len - depth - 1,
                &mut next_zeros,
                &mut next_ones,
                &mut rsd_,
            );
            Self::filter(
                &ones,
                bit_len - depth - 1,
                &mut next_zeros,
                &mut next_ones,
                &mut rsd_,
            );
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

    fn filter(
        vals: &Vec<u64>,
        shift: u8,
        next_zeros: &mut Vec<u64>,
        next_ones: &mut Vec<u64>,
        rsd: &mut RsDicBuilder,
    ) {
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
    fn count(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Operator::Equal)
    }

    /// Returns the number of the element which satisfies `e < value` included in A[pos_range]
    fn count_lt(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Operator::LessThan)
    }

    /// Returns the number of the element which satisfies `e > value` included in A[pos_range]
    fn count_gt(&self, pos_range: Range<usize>, value: u64) -> usize {
        self.prefix_rank_op(pos_range, value, 0, Operator::GreaterThan)
    }

    /// Returns the number of the element which satisfies `val_range.start <= e < val_range.end` included in A[pos_range]
    fn count_range(&self, pos_range: Range<usize>, val_range: Range<u64>) -> usize {
        self.count_lt(pos_range.clone(), val_range.end) - self.count_lt(pos_range, val_range.start)
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
    fn prefix_rank_op(
        &self,
        pos_range: Range<usize>,
        val: u64,
        ignore_bit: u8,
        operator: Operator,
    ) -> usize {
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
        if self.bit_len < ignore_bit {
            return pos + rank; // may overflow the len()?
        }
        if depth == self.bit_len - ignore_bit {
            return pos + rank;
        }
        let mut pos = pos;
        let mut rank = rank;

        let bit = get_bit_msb(val, depth, self.bit_len);
        let rsd = &self.layers[depth as usize];
        if !bit {
            pos = rsd.rank(pos, bit);
            rank = self.select_helper(rank, val, pos, depth + 1, ignore_bit);
        } else {
            pos = rsd.zero_num() + rsd.rank(pos, bit);
            rank = self.select_helper(rank, val, pos, depth + 1, ignore_bit) - rsd.zero_num();
        }
        rsd.select(rank, bit)
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
    #[cfg(test)]
    extern crate rand;

    use super::*;
    // use super::rand;
    #[test]
    fn example() {
        let vec: Vec<u64> = vec![1, 2, 4, 5, 1, 0, 4, 6, 2, 9, 2, 0];
        //                       0  1  2  3  4  5  6  7  8  9 10 11
        let wm = WaveletMatrix::new(&vec);

        assert_eq!(wm.len(), 12);
        assert_eq!(wm.lookup(7), 6);

        // Counting
        assert_eq!(wm.count(0..wm.len(), 2), 3);
        assert_eq!(wm.count(0..wm.len(), 4), 2);
        assert_eq!(wm.count(0..wm.len(), 5), 1);
        assert_eq!(wm.count(0..wm.len(), 7), 0);
        assert_eq!(wm.count(0..wm.len(), 39), 0);

        assert_eq!(wm.count_lt(0..wm.len(), 2), 4);
        assert_eq!(wm.count_lt(0..wm.len(), 7), 11);

        assert_eq!(wm.count_gt(0..wm.len(), 2), 5);
        assert_eq!(wm.count_gt(0..wm.len(), 7), 1);

        assert_eq!(wm.count_range(0..wm.len(), 0..10), 12);
        assert_eq!(wm.count_range(0..wm.len(), 4..6), 3);

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

    #[test]
    fn random_1000_lookup() {
        let mut vec: Vec<u64> = Vec::new();
        for _ in 0..LEN {
            vec.push(rand::random());
        }
        let wm = WaveletMatrix::new(&vec);
        assert_eq!(wm.len(), vec.len());
        for i in 0..LEN {
            assert_eq!(wm.lookup(i), vec[i]);
        }
    }
}
