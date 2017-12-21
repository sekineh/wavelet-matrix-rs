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

/// Comparator trait
pub trait NodeRange: Ord {
    fn new(pos_range: Range<usize>, depth: u8, prefix_char: u64, bit_len: u8) -> Self;
    fn inner(&self) -> &QueryOnNode;
    fn from(qon: &QueryOnNode) -> Self;
}

/// QueryOnNode orderd by frequency
#[derive(Debug, Clone, Eq)]
pub struct NodeRangeByFrequency(QueryOnNode);

impl NodeRange for NodeRangeByFrequency {
    fn new(pos_range: Range<usize>, depth: u8, prefix_char: u64, bit_len: u8) -> Self {
        NodeRangeByFrequency(QueryOnNode::new(pos_range, prefix_char, depth, bit_len))
    }
    fn inner(&self) -> &QueryOnNode {
        &self.0
    }
    fn from(qon: &QueryOnNode) -> Self {
        NodeRangeByFrequency(qon.clone())
    }
}

impl Ord for NodeRangeByFrequency {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0.count() != other.0.count() {
            self.0.count().cmp(&other.0.count()) // wide node first
        } else if self.0.depth != other.0.depth {
            self.0.depth.cmp(&other.0.depth) // deepest node first
        } else {
            self.0.pos_start.cmp(&other.0.pos_start) // just to make the order more predictable
        }
    }
}

impl PartialOrd for NodeRangeByFrequency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for NodeRangeByFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

/// QueryOnNode orderd by frequency
#[derive(Debug, Clone, Eq)]
pub struct NodeRangeBySumError(QueryOnNode);

impl NodeRange for NodeRangeBySumError {
    fn new(pos_range: Range<usize>, depth: u8, prefix_char: u64, bit_len: u8) -> Self {
        NodeRangeBySumError(QueryOnNode::new(pos_range, prefix_char, depth, bit_len))
    }
    fn inner(&self) -> &QueryOnNode {
        &self.0
    }
    fn from(qon: &QueryOnNode) -> Self {
        NodeRangeBySumError(qon.clone())
    }
}

impl Ord for NodeRangeBySumError {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.sum_error().cmp(&other.0.sum_error())
    }
}

impl PartialOrd for NodeRangeBySumError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for NodeRangeBySumError {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

/// QueryOnNode in Descending order
#[derive(Debug, Clone, Eq)]
pub struct NodeRangeDescending(QueryOnNode);

impl NodeRange for NodeRangeDescending {
    fn new(pos_range: Range<usize>, depth: u8, prefix_char: u64, bit_len: u8) -> Self {
        NodeRangeDescending(QueryOnNode::new(pos_range, prefix_char, depth, bit_len))
    }
    fn inner(&self) -> &QueryOnNode {
        &self.0
    }
    fn from(qon: &QueryOnNode) -> Self {
        NodeRangeDescending(qon.clone())
    }
}

impl Ord for NodeRangeDescending {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.prefix_char.cmp(&other.0.prefix_char)
    }
}

impl PartialOrd for NodeRangeDescending {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for NodeRangeDescending {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

/// QueryOnNode in Ascending order
#[derive(Debug, Clone, Eq)]
pub struct NodeRangeAscending(QueryOnNode);

impl NodeRange for NodeRangeAscending {
    fn new(pos_range: Range<usize>, depth: u8, prefix_char: u64, bit_len: u8) -> Self {
        NodeRangeAscending(QueryOnNode::new(pos_range, prefix_char, depth, bit_len))
    }
    fn inner(&self) -> &QueryOnNode {
        &self.0
    }
    fn from(qon: &QueryOnNode) -> Self {
        NodeRangeAscending(qon.clone())
    }
}

impl Ord for NodeRangeAscending {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.prefix_char.cmp(&self.0.prefix_char)
    }
}

impl PartialOrd for NodeRangeAscending {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for NodeRangeAscending {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

