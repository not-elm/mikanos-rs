use crate::xhc::transfer::event::segment_table::SegmentTableAddr;
use crate::VolatileAccessible;

pub trait RingSegmentTableField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, SegmentTableAddr>,
{
    fn new(address: SegmentTableAddr) -> Self;
}

impl<T, VolatileType> RingSegmentTableField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, SegmentTableAddr>,
{
    fn new(address: SegmentTableAddr) -> Self {
        T::new_uncheck(address.addr())
    }
}
