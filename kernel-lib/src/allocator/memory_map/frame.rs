use common_lib::physical_address::PhysicalAddress;

#[derive(Debug, Clone)]
pub struct Frame {
    frame_id: usize,
    base_phys_addr: PhysicalAddress,
    end_phys_addr: PhysicalAddress,
}


impl Frame {
    pub fn new(
        frame_id: usize,
        base_phys_addr: PhysicalAddress,
        end_phys_addr: PhysicalAddress,
    ) -> Frame {
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

    pub fn base_phys_addr(&self) -> PhysicalAddress {
        self.base_phys_addr
    }

    pub fn end_phys_addr(&self) -> PhysicalAddress {
        self.end_phys_addr
    }
}
