use crate::xhci::registers::runtime_registers::interrupter_register_set::interrupter_management_register::interrupt_enable::InterruptEnable;
use crate::xhci::registers::runtime_registers::interrupter_register_set::interrupter_management_register::interrupt_pending::InterruptPending;
use crate::xhci::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;
use crate::xhci::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;

pub mod interrupt_enable;
pub mod interrupt_pending;

/// IMAN
///
/// # Offset
///
/// RuntimeRegistersOffset + 0x20 Bytes
///
/// # Size
///
/// 32 Bits
///
/// # Description
///
/// xHCの割り込みの有効または、無効化の操作(IE)と、
/// 割り込みの発生状態の検知(IP)ができます。
///
/// [Xhci Document] : 425 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(Debug)]
pub struct InterrupterManagementRegister {
    /// Offset: 0
    ip: InterruptPending,
    /// Offset: 1 Bit
    ie: InterruptEnable,
}

impl InterrupterManagementRegister {
    pub fn new(offset: InterrupterRegisterSetOffset) -> Self {
        Self {
            ip: InterruptPending::new(offset),
            ie: InterruptEnable::new(offset),
        }
    }

    pub fn ip(&self) -> &InterruptPending {
        &self.ip
    }

    pub fn ie(&self) -> &InterruptEnable {
        &self.ie
    }
}
