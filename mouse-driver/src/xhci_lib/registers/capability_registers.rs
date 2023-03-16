use macros::Volatile;

//
pub struct CapabilityRegisters {
    pub cap_length: CapabilityLength,
    // _r1: Capability,
    //
    // hci_version: u16,
    // hcs_params1: u32,
    // #[bits(4)]
    // hcs_params2: usize,
    // #[bits(4)]
    // hcs_params3: usize,
    // #[bits(4)]
    // hcc_params1: usize,
    // #[bits(4)]
    // pub doorbell_offset: usize,
    // #[bits(4)]
    // pub runtime_register_space_offset: usize,
    // #[bits(4)]
    // pub hcc_params2: usize,
}

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u8)]
pub struct CapabilityLength(usize);

impl CapabilityRegisters {
    pub fn new(mmio_base_addr: usize) -> Self {
        Self {
            cap_length: CapabilityLength::new(mmio_base_addr),
        }
    }
}
