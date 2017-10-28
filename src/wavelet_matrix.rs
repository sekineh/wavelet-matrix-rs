use rsdic_simple::*;

///
#[derive(Debug)]
pub struct WaveletMatrix {
    layers: Vec<RsDic>,
    dim: u64,   // max value + 1
    num: usize, // = layers[0].len()
    blen: u8,   // = layers.len()
}

impl WaveletMatrix {
    /// Create a new WaveletMatrix struct from a input Vec<u64>.
    pub fn new(vals: Vec<u64>) -> WaveletMatrix {
        let dim = get_dim(&vals);
        let blen = get_binary_len(dim);
        let num = vals.len();
        let mut zeros: Vec<u64> = vals;
        let mut ones: Vec<u64> = Vec::new();
        let mut layers: Vec<RsDic> = Vec::new();

        for depth in 0..blen {
            let mut next_zeros: Vec<u64> = Vec::new();
            let mut next_ones: Vec<u64> = Vec::new();
            let mut rsd_ = RsDicBuilder::new();
            Self::filter(
                &zeros,
                blen - depth - 1,
                &mut next_zeros,
                &mut next_ones,
                &mut rsd_,
            );
            Self::filter(
                &ones,
                blen - depth - 1,
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
            blen: blen,
        }
    }

    fn filter(
        vals: &Vec<u64>,
        depth: u8,
        next_zeros: &mut Vec<u64>,
        next_ones: &mut Vec<u64>,
        rsd: &mut RsDicBuilder,
    ) {
        for val in vals {
            let bit = ((val >> depth) & 1) == 1;
            rsd.push_bit(bit);
            if bit {
                next_ones.push(*val);
            } else {
                next_zeros.push(*val);
            }
        }
    }

    fn lookup_and_rank(&self, pos: usize) -> (u64, usize) {
        let mut val: u64 = 0;
        let mut bpos: usize = 0;
        let mut epos: usize = pos;

        for depth in 0..self.blen as usize {
            let rsd = &self.layers[depth];
            let bit = rsd.access(epos);
            bpos = rsd.rank(bpos, bit);
            epos = rsd.rank(epos, bit);
            val <<= 1;
            if bit {
                bpos += rsd.zero_num();
                epos += rsd.zero_num();
                val |= 1;
            }
        }
        (val, epos - bpos)
    }

    pub fn lookup(&self, pos: usize) -> u64 {
        let mut val: u64 = 0;
        let mut epos: usize = pos;

        for depth in 0..self.blen as usize {
            let rsd = &self.layers[depth];
            let bit = rsd.access(epos);
            epos = rsd.rank(epos, bit);
            val <<= 1;
            if bit {
                epos += rsd.zero_num();
                val |= 1;
            }
        }
        val
    }

    pub fn rank(&self, pos: usize, val: u64) -> usize {
        let mut bpos = 0;
        let mut epos = pos;
        for depth in 0..self.blen {
            let rsd = &self.layers[depth as usize];
            let bit = get_msb(val, depth, self.blen);
            bpos = rsd.rank(bpos, bit);
            epos = rsd.rank(epos, bit);
            if bit {
                bpos += rsd.zero_num();
                epos += rsd.zero_num();
            }
        }
        epos - bpos
    }
}

fn get_msb(x: u64, pos: u8, blen: u8) -> bool {
    ((x >> (blen - pos - 1)) & 1) == 1
}

// impl std::ops::Index<u64> for WaveletMatrix {
//     type Output = u64;
//     fn index(&self, pos: u64) -> u64 {
//         let (val, _) = self.lookup_and_rank(pos);
//         val
//     }
// }

// impl RankSupport for WaveletMatrix {
//     type Over = u64;
//     fn limit(&self) -> u64 {
//         self.num
//     }
//     fn rank(&self, pos: u64, value: u64) -> u64 {

//     }
// }

/// Thin builder that builds WaveletMatrix
#[derive(Debug)]
pub struct WaveletMatrixBuilder {
    vals: Vec<u64>,
}

impl WaveletMatrixBuilder {
    pub fn new() -> WaveletMatrixBuilder {
        WaveletMatrixBuilder { vals: Vec::new() }
    }

    pub fn push(&mut self, val: u64) {
        self.vals.push(val);
    }

    pub fn build(self) -> WaveletMatrix {
        WaveletMatrix::new(self.vals)
    }
}

// Possible bug:
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

fn get_binary_len(val: u64) -> u8 {
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

        assert_eq!(wm.rank(0, 1), 0);
        assert_eq!(wm.rank(1, 1), 1);
        assert_eq!(wm.rank(4, 1), 1);

        assert_eq!(wm.rank(0, 31), 0);
        assert_eq!(wm.rank(1, 31), 0);
        assert_eq!(wm.rank(2, 31), 1);
        assert_eq!(wm.rank(3, 31), 1);
        assert_eq!(wm.rank(4, 31), 1);

        assert_eq!(wm.lookup_and_rank(4), (11, 1));
    }
}
