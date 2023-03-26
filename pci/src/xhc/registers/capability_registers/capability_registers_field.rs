use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::VolatileAccessible;

pub trait CapabilityRegistersField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, MemoryMappedAddr>,
{
    fn new(offset: MemoryMappedAddr) -> T;
}

impl<T, VolatileType> CapabilityRegistersField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, MemoryMappedAddr>,
{
    fn new(offset: MemoryMappedAddr) -> T {
        T::new_uncheck(offset.addr())
    }
}
