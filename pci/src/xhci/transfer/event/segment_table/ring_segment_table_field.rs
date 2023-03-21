use crate::xhci::transfer::event::segment_table::SegmentTableAddr;
use crate::VolatileAccessible;

pub trait RingSegmentTableField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, SegmentTableAddr>,
{
    fn new(address: SegmentTableAddr) -> T;
}

impl<T, VolatileType> RingSegmentTableField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, SegmentTableAddr>,
{
    fn new(address: SegmentTableAddr) -> T {
        T::new_uncheck(address.addr())
    }
}
