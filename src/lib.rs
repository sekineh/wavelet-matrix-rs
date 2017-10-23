extern crate succinct;

use succinct::*;
use succinct::rank::RankSupport;
use succinct::select::SelectSupport;
use succinct::SpaceUsage;

// #[derive(Debug)]
struct RsDic {
    select_support: BinSearchSelect<JacobsonRank<BitVector>>
}

impl RsDic {
    fn access(&self, pos: u64) -> bool {
        self.select_support.inner().inner().get_bit(pos)
    }
    fn zero_num(&self) -> u64 {
        self.rank(self.limit(), false)
    }
    fn one_num(&self) -> u64 {
        self.rank(self.limit(), true)
    }
}

impl RankSupport for RsDic {
    type Over = bool;
    fn limit(&self) -> u64 {
        self.select_support.inner().limit()        
    }
    fn rank(&self, pos: u64, value: bool) ->u64 {
        // self.select_support.inner().rank(pos, value)
        if pos > 0 {
            self.select_support.inner().rank(pos - 1, value)
        } else {
            0
        }
    }
}

impl SelectSupport for RsDic {
    type Over = bool;
    fn select(&self, index: u64, value: bool) ->Option<u64> {
        self.select_support.select(index, value)
    }
}

struct RsDicBuilder {
    bv: BitVector,
}

impl RsDicBuilder {
    fn new() -> Self {
        RsDicBuilder {
            bv: BitVector::new()
        }
    }
    fn push_bit(&mut self, bit: bool) {
        self.bv.push_bit(bit);
    }
    fn build(self) -> RsDic {
        let rank = JacobsonRank::new(self.bv);
        let select = BinSearchSelect::new(rank);
        RsDic {
            select_support: select,
        }
    }
}

impl SpaceUsage for RsDic {
    fn is_stack_only() -> bool { 
        false 
    }
    fn heap_bytes(&self) -> usize {
        self.select_support.heap_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsdic_sanity() {
        let mut rsb = RsDicBuilder::new();
        for _ in  0..1_000_000 {
            rsb.push_bit(true);
        }
        let rs = rsb.build();

        // space usage
        assert!(rs.total_bytes() > 1_000_000 / 8); // > orig
        assert!(rs.total_bytes() < 1_000_000 / 8 * 13 / 10); // < orig * 1.3
        assert_eq!(RsDic::stack_bytes(), 120);

        // .access()
        assert_eq!(rs.access(0), true);
        assert_eq!(rs.access(999_999), true);
        // assert_eq!(rs.access(1_000_000), true); // Panics

        // .rank(_, true)
        assert_eq!(rs.rank(0, true), 0);
        assert_eq!(rs.rank(999_999, true), 999_999);
        assert_eq!(rs.rank(1_000_000, true), 1_000_000); // Panics

        // .select(_, true)
        assert_eq!(rs.select(0, true), Some(0));
        assert_eq!(rs.select(999_999, true), Some(999_999));
        assert_eq!(rs.select(1_000_000, true), None);

        // .rank(_, false)
        assert_eq!(rs.rank(0, false), 0);
        assert_eq!(rs.rank(999_999, false), 0);
        assert_eq!(rs.rank(1_000_000, false), 0);

        // .select(_, false)
        assert_eq!(rs.select(0, false), None);
        assert_eq!(rs.select(999_999, false), None);
        assert_eq!(rs.select(1_000_000, false), None);
    }
}

// #[derive(Debug)]
struct WaveletMatrix {
    layers: Vec<RsDic>,
    dim: u64,
    num: u64,
    blen: u64, // = layers.len()
}

impl WaveletMatrix {
    fn lookup_and_rank(&self, pos: u64) -> (u64, u64) {
        let mut val: u64 = 0;
        let mut bpos: u64 = 0;
        let mut epos: u64 = pos;

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

    fn lookup(&self, pos: u64) -> u64 {
        let mut val: u64 = 0;
        let mut epos: u64 = pos;

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

    fn rank(&self, pos: u64, val: u64) -> u64 {
        let mut bpos = 0;
        let mut epos = pos;
        for depth in 0..self.blen as usize {
            let rsd = &self.layers[depth];
            let bit = get_msb(val, depth as u64, self.blen);
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

fn get_msb(x: u64, pos: u64, blen: u64) -> bool {
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

#[derive(Debug)]
struct WaveletMatrixBuilder {
    vals: Vec<u64>,
}

impl WaveletMatrixBuilder {
    fn new() -> WaveletMatrixBuilder {
        WaveletMatrixBuilder {
            vals: Vec::new(),
        }
    }

    fn push(&mut self, val: u64) {
        self.vals.push(val);
    }

    fn build(self) -> WaveletMatrix {
        let dim = get_dim(&self.vals);
        let blen = get_binary_len(dim);
        let num = self.vals.len();
        let mut zeros: Vec<u64> = self.vals;
        let mut ones: Vec<u64> = Vec::new();
        let mut layers: Vec<RsDic> = Vec::new();
        
        for depth in 0..blen {
            let mut next_zeros: Vec<u64> = Vec::new();
            let mut next_ones: Vec<u64> = Vec::new();
            let mut rsd_ = RsDicBuilder::new();
            filter(&zeros, blen-depth-1, &mut next_zeros, &mut next_ones, &mut rsd_);
            filter(&ones, blen-depth-1, &mut next_zeros, &mut next_ones, &mut rsd_);
            zeros = next_zeros;
            ones = next_ones;
            layers.push(rsd_.build());
        }

        WaveletMatrix {
            layers: layers,
            dim: dim,
            num: num as u64,
            blen: blen,
        }

    }
}

fn filter(vals: &[u64], depth: u64, next_zeros: &mut Vec<u64>, next_ones: &mut Vec<u64>, rsd: &mut RsDicBuilder) {
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

fn get_dim(vals: &[u64]) -> u64 {
    let mut dim = 0;
    for val in vals.iter() {
        if *val >= dim {
            dim = val + 1;
        }
    }
    dim
}

fn get_binary_len(val: u64) -> u64 {
    let mut blen = 0;
    let mut val = val;
    while val > 0 {
        val >>= 1;
        blen += 1;
    }
    blen
}

#[cfg(test)]
mod testss {
    use super::*;

    #[test]
    fn wavelet_matrix_sanity() {
        let mut wmb = WaveletMatrixBuilder::new();
        wmb.push(1);
        wmb.push(31);
        wmb.push(11);
        wmb.push(10);
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
    }

}
