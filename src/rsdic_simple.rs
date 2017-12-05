use succinct::*;
use succinct::rank::RankSupport;
use succinct::select::SelectSupport;
use succinct::SpaceUsage;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

pub struct RsDic {
    select_support: BinSearchSelect<JacobsonRank<BitVector>>,
}

impl Debug for RsDic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "RsDic: {{ bit_len(): {} }}",
               self.select_support.inner().inner().bit_len())
    }
}

impl RsDic {
    pub fn access(&self, pos: usize) -> bool {
        self.select_support.inner().inner().get_bit(pos as u64)
    }

    pub fn zero_num(&self) -> usize {
        self.rank(self.len(), false)
    }

    // pub fn one_num(&self) -> usize {
    //     self.rank(self.len(), true)
    // }

    /// Return the length of bits
    pub fn len(&self) -> usize {
        self.select_support.inner().limit() as usize
    }

    /// Return the occurrences of value within the range [0, pos)
    ///
    /// Always return 0 for rank(0, value) for any value.
    /// Note: It is different from rank() definition used in succinct::rank::RankSupport
    pub fn rank(&self, pos: usize, value: bool) -> usize {
        // self.select_support.inner().rank(pos, value)
        if pos > 0 {
            self.select_support.inner().rank(pos as u64 - 1, value) as usize
        } else {
            0
        }
    }

    /// Return the position of the index'th occurrence of the value
    ///
    /// If found nothing, it returns the length of the vector.
    pub fn select(&self, index: usize, value: bool) -> usize {
        match self.select_support.select(index as u64, value) {
            Some(x) => x as usize,
            None => self.len(),
        }
    }
}

pub struct RsDicBuilder {
    bv: BitVector,
}

impl RsDicBuilder {
    pub fn new() -> Self {
        RsDicBuilder { bv: BitVector::new() }
    }
    pub fn push_bit(&mut self, bit: bool) {
        self.bv.push_bit(bit);
    }
    pub fn build(self) -> RsDic {
        let rank = JacobsonRank::new(self.bv);
        let select = BinSearchSelect::new(rank);
        RsDic { select_support: select }
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
        for _ in 0..1_000_000 {
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
        assert_eq!(rs.select(0, true), 0);
        assert_eq!(rs.select(999_999, true), 999_999);
        assert_eq!(rs.select(1_000_000, true), 1_000_000);

        // .rank(_, false)
        assert_eq!(rs.rank(0, false), 0);
        assert_eq!(rs.rank(999_999, false), 0);
        assert_eq!(rs.rank(1_000_000, false), 0);

        // .select(_, false)
        assert_eq!(rs.select(0, false), 1_000_000);
        assert_eq!(rs.select(999_999, false), 1_000_000);
        assert_eq!(rs.select(1_000_000, false), 1_000_000);

        assert_eq!(rs.zero_num(), 0);
        // assert_eq!(rs.one_num(), 1_000_000);
    }
}
