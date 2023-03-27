use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::PciResult;
use crate::wait_update_64bits_register_for;
use crate::xhc::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;

/// ERSTBA
///
/// # Offset
///
/// InterrupterRegisterSetOffset + 0x10 Bytes
///
/// # Size
///
/// 64 Bits
///
/// # Attribute
/// RW
///
/// # Description
///
/// EventRingSegmentTableの先頭アドレスを保持します。
///
/// [Xhci Document] : 428 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u64)]
#[add_addr_bytes(0x18)]
#[offset_bit(4)]
pub struct EventRingDequeuePointer(usize, PhantomData<InterrupterRegisterSetOffset>);

impl EventRingDequeuePointer {
    pub(crate) fn update_deque_pointer(&self, deque_ptr_addr: u64) -> PciResult {
        let addr = deque_ptr_addr >> 4;
        self.write_volatile(addr);
        wait_update_64bits_register_for(10, addr, self)
    }

    pub fn read_deque_pointer(&self) -> u64 {
        (self.read_volatile() << 4)
    }
}
