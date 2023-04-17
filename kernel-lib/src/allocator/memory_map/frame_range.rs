use core::ops::{Bound, Range, RangeBounds};

use common_lib::physical_address::PhysicalAddress;

use crate::allocator::memory_map::frame::Frame;

#[derive(Debug, Clone)]
pub struct FrameRange {
    base: Frame,
    end: Frame,
}


impl FrameRange {
    pub fn new(base: Frame, end: Frame) -> Self {
        Self { base, end }
    }


    pub fn base(&self) -> &Frame {
        &self.base
    }

    pub fn end(&self) -> &Frame {
        &self.end
    }

    pub fn address_range(&self) -> Range<PhysicalAddress> {
        self.base.base_phys_addr()..self.end.end_phys_addr()
    }

    pub fn is_contain_address(&self, phys_addr: PhysicalAddress) -> bool {
        self.address_range()
            .contains(&phys_addr)
    }
}


impl RangeBounds<Frame> for FrameRange {
    fn start_bound(&self) -> Bound<&Frame> {
        Bound::Included(self.base())
    }

    fn end_bound(&self) -> Bound<&Frame> {
        Bound::Included(self.end())
    }
}


#[cfg(test)]
mod tests {
    use common_lib::physical_address::PhysicalAddress;

    use crate::allocator::memory_map::frame::Frame;
    use crate::allocator::memory_map::frame_range::FrameRange;

    #[test]
    fn it_range() {
        let range = FrameRange::new(
            Frame::new(0, PhysicalAddress::new(0), PhysicalAddress::new(1)),
            Frame::new(1, PhysicalAddress::new(30), PhysicalAddress::new(60)),
        );

        assert_eq!(
            range
                .address_range()
                .start
                .raw(),
            0
        );
        assert_eq!(
            range
                .address_range()
                .end
                .raw(),
            60
        );
    }
}
