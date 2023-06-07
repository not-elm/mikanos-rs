use common_lib::nums::{FlagConvertible, FlagNumConvertible};

use crate::configuration_space::msi::msi_capability_register::structs::capability_id::CapabilityId;
use crate::configuration_space::msi::msi_capability_register::structs::from_u32::TryFromU32;
use crate::error::PciResult;

#[derive(Debug, Clone)]
pub struct Control {
    capability_id: CapabilityId,
    next_cap_ptr: u8,
    msi_enable: bool,
    multiple_msg_capable: u8,
    multiple_msg_enable: u8,
    is_64bit_addr_capable: bool,
    per_vector_masking_capable: bool,
}


impl Control {
    pub fn capability_id(&self) -> CapabilityId {
        self.capability_id
    }


    pub fn next_cap_ptr(&self) -> u8 {
        self.next_cap_ptr
    }


    pub fn msi_enable(&self) -> bool {
        self.msi_enable
    }


    pub fn is_64bit_addr_capable(&self) -> bool {
        self.is_64bit_addr_capable
    }

    pub fn set_msi_enable(&mut self) {
        self.msi_enable = true;
    }


    pub fn clear_msi_enable(&mut self) {
        self.msi_enable = false;
    }


    pub fn multiple_msg_enable(&mut self) -> u8 {
        self.multiple_msg_enable
    }

    pub fn set_multiple_msg_enable(&mut self, multiple_msg_enable: u8) {
        self.multiple_msg_enable = multiple_msg_enable & 0b111;
    }


    pub fn multiple_msg_capable(&mut self) -> u8 {
        self.multiple_msg_capable
    }


    pub fn raw(&self) -> u32 {
        let left = |v: u32, shift: u32| v << shift;
        let left_u8 = |v: u8, shift: u32| left(v as u32, shift);
        left(
            self.per_vector_masking_capable
                .into_bit(),
            24,
        ) | left(
            self.is_64bit_addr_capable
                .into_bit(),
            23,
        ) | left_u8(self.multiple_msg_enable, 20)
            | left_u8(self.multiple_msg_capable, 17)
            | left(self.msi_enable.into_bit(), 16)
            | left_u8(self.next_cap_ptr, 8)
            | self.capability_id as u32
    }
}


impl TryFromU32<Control> for Control {
    fn try_from_u32(raw: u32) -> PciResult<Control> {
        Ok(Self {
            capability_id: capability_id(raw)?,
            next_cap_ptr: next_cap_ptr(raw),
            msi_enable: msi_enable(raw),
            multiple_msg_capable: multiple_msg_capable(raw),
            multiple_msg_enable: multiple_msg_enable(raw),
            is_64bit_addr_capable: is_64bit_addr_capable(raw),
            per_vector_masking_capable: per_vector_masking_capable(raw),
        })
    }
}


fn capability_id(raw: u32) -> PciResult<CapabilityId> {
    CapabilityId::try_from_u8((raw & 0xFF) as u8)
}


fn next_cap_ptr(raw: u32) -> u8 {
    ((raw >> 8) & 0xFF) as u8
}


fn msi_enable(raw: u32) -> bool {
    flag(raw, 16)
}

fn multiple_msg_capable(raw: u32) -> u8 {
    ((raw >> 17) & 0b111) as u8
}

fn multiple_msg_enable(raw: u32) -> u8 {
    ((raw >> 20) & 0b111) as u8
}

fn is_64bit_addr_capable(raw: u32) -> bool {
    flag(raw, 23)
}

fn per_vector_masking_capable(raw: u32) -> bool {
    flag(raw, 24)
}


fn flag(raw: u32, right_shift: usize) -> bool {
    let v = (raw >> right_shift) & 0b1;
    v.is_true()
}


#[cfg(test)]
mod tests {
    use crate::configuration_space::msi::msi_capability_register::structs::capability_id::CapabilityId;
    use crate::configuration_space::msi::msi_capability_register::structs::control::Control;
    use crate::configuration_space::msi::msi_capability_register::structs::from_u32::TryFromU32;
    use crate::error::PciResult;

    #[test]
    fn it_new_control_is_ok() {
        assert!(control_register().is_ok())
    }


    #[test]
    fn it_read_capability_id() {
        assert_eq!(
            control_register()
                .unwrap()
                .capability_id(),
            CapabilityId::Msi
        )
    }


    #[test]
    fn it_read_next_cap_ptr() {
        assert_eq!(
            control_register()
                .unwrap()
                .next_cap_ptr(),
            0xFF
        )
    }

    #[test]
    fn it_read_multiple_msg_enable() {
        assert_eq!(
            control_register()
                .unwrap()
                .multiple_msg_enable(),
            0b101
        )
    }


    #[test]
    fn it_read_is_64bit_addressing() {
        assert!(!control_register()
            .unwrap()
            .is_64bit_addr_capable())
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn it_when_update_get_correct_value() {
        let mut control = Control::try_from_u32(0b0_101_111_0_11111111_00000101).unwrap();
        control.set_msi_enable();
        control.set_multiple_msg_enable(0b001);

        assert_eq!(control.raw(), 0b0_001_111_1_11111111_00000101);
    }

    #[allow(clippy::unusual_byte_groupings)]
    fn control_register() -> PciResult<Control> {
        Control::try_from_u32(0b0_101_111_1_11111111_00000101)
    }
}
