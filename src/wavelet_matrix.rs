use rsdic_simple::*;

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
    pub fn new(vals: Vec<u64>) -> WaveletMatrix {
        let dim = get_dim(&vals);
        let bit_len = get_bit_len(dim);
        let num = vals.len();
        let mut zeros: Vec<u64> = vals;
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

    /// Returns the number of val found in T[0..pos].
    ///
    /// The range specified is half open, i.e. [0, pos).
    pub fn rank(&self, pos: usize, val: u64) -> usize {
        let mut bpos = 0;
        let mut epos = pos;

        for depth in 0..self.bit_len {
            let rsd = &self.layers[depth as usize];
            let bit = get_bit_msb(val, depth, self.bit_len);
            bpos = rsd.rank(bpos, bit);
            epos = rsd.rank(epos, bit);
            if bit {
                bpos += rsd.zero_num();
                epos += rsd.zero_num();
            }
        }
        epos - bpos
    }

    /// Return the position of (rank+1)-th val in T.
    /// 
    /// If no match has been found, it returns the length of self.
    pub fn select(&self, rank: usize, val: u64) -> usize {
        self.select_helper(rank, val, 0, 0)
    }

    fn select_helper(&self, rank: usize, val: u64, pos: usize, depth: u8) -> usize {
        if depth == self.bit_len {
            return pos + rank;
        }
        let mut pos = pos;
        let mut rank = rank;

        let bit = get_bit_msb(val, depth, self.bit_len);
        let rsd = &self.layers[depth as usize];
        if !bit {
            pos = rsd.rank(pos, bit);
            rank = self.select_helper(rank, val, pos, depth + 1);
        } else {
            pos = rsd.zero_num() + rsd.rank(pos, bit);
            rank = self.select_helper(rank, val, pos, depth + 1) - rsd.zero_num();
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
        WaveletMatrix::new(self.vals)
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
    use super::*;

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
    }
}
