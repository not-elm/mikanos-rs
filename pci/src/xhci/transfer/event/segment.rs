use macros::Address;

pub struct Segment {}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Address)]
#[repr(transparent)]
pub struct RingSegmentsBaseAddr(usize);
