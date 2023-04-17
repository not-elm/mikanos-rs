use common_lib::physical_address::PhysicalAddress;

#[derive(Debug, Clone)]
pub struct MemoryMapFrame {
    frame_id: usize,
    base_phys_addr: PhysicalAddress,
    end_phys_addr: PhysicalAddress,
}


impl MemoryMapFrame {
    pub fn new(
        frame_id: usize,
        base_phys_addr: PhysicalAddress,
        end_phys_addr: PhysicalAddress) -> MemoryMapFrame {
        Self {
            frame_id,
            base_phys_addr,
            end_phys_addr,
        }
    }
    pub fn id(&self) -> usize {
        self.frame_id
    }

    pub fn contains(&self, phys_addr: PhysicalAddress) -> bool {
        self.base_phys_addr <= phys_addr && phys_addr <= self.end_phys_addr
    }

    pub fn base_phys_addr(&self) -> PhysicalAddress{
        self.base_phys_addr
    }
}

pub trait MemoryMapFrameIterable: Iterator<Item=MemoryMapFrame> {
    fn last_id(&self) -> Option<usize>;
    fn frame_at(&mut self, frame_id: usize) -> Option<MemoryMapFrame>;
    fn frame_contains_address(&mut self, phys_addr: PhysicalAddress) -> Option<MemoryMapFrame>;
}


