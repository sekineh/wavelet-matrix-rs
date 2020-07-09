use std::cmp::Ordering;
use std::ops::Range;

/// Stores the node position during Wavelet Matrix traversing.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct QueryOnNode {
    pub pos_start: usize,
    pub pos_end: usize,
    pub prefix_char: u64,
    pub depth: u8,
    pub bit_len: u8,
}

impl QueryOnNode {
    pub fn new(pos_range: Range<usize>, prefix_char: u64, depth: u8, bit_len: u8) -> Self {
        QueryOnNode {
            pos_start: pos_range.start,
            pos_end: pos_range.end,
            prefix_char: prefix_char,
            depth: depth,
            bit_len: bit_len,
        }
    }
    #[inline]
    fn count(&self) -> usize {
        self.pos_end - self.pos_start
    }
    #[inline]
    fn value_range(&self) -> Range<u64> {
        let shift = self.bit_len - self.depth;
        self.prefix_char << shift..(self.prefix_char + 1) << shift
    }
    #[inline]
    fn sum_error(&self) -> u64 {
        let vr = self.value_range();
        (vr.end - vr.start - 1) * (self.count() as u64)
    }
    // #[inline]
    // fn sum_expected(&self) -> u64 {
    //     let vr = self.value_range();
    //     (vr.end + vr.start) / 2 * (self.count() as u64)
    // }
}

pub fn cmp_by_frequency(self_: &QueryOnNode, other: &QueryOnNode) -> Ordering {
    if self_.count() != other.count() {
        self_.count().cmp(&other.count()) // wide node first
    } else if self_.depth != other.depth {
        self_.depth.cmp(&other.depth) // deepest node first
    } else {
        self_.pos_start.cmp(&other.pos_start) // just to make the order more predictable
    }
}

pub fn cmp_by_sum_error(self_: &QueryOnNode, other: &QueryOnNode) -> Ordering {
    self_.sum_error().cmp(&other.sum_error())
}

pub fn cmp_by_descending(self_: &QueryOnNode, other: &QueryOnNode) -> Ordering {
    self_.prefix_char.cmp(&other.prefix_char)
}

pub fn cmp_by_ascending(self_: &QueryOnNode, other: &QueryOnNode) -> Ordering {
    other.prefix_char.cmp(&self_.prefix_char)
}
