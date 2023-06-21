/// Task priority
///
/// Higher-level tasks take precedence over lower-levels.
#[derive(Default, Debug, Clone, Copy,  Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct PriorityLevel(usize);




impl PriorityLevel{
    #[inline(always)]
    pub const fn new(level: usize) -> Self{
        Self(level)
    }




}
