use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::PciResult;
use crate::wait_update_32bits_register_for;
use crate::xhci::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;

/// ERSTS
///
/// # Offset
///
/// InterrupterRegisterSetOffset + 0x08 Bytes
///
/// # Size
///
/// 32 Bits
///
/// # Attribute
/// RW
///
/// # Default
///
/// 0
///
/// # Description
///
/// EventRingSegmentTableでサポートされるセグメントの数を表します。
///
/// 設定できる最大の数はHcsParams2のErstMaxに設定されています。
///
/// # Notes
///
/// * SecondaryInterrupterの場合、このフィールドを0に設定するとEventRingが動作を停止します。
/// (そのEventRingを対象にしたイベントは未定義の動作を起こします。)
///
/// * PrimaryInterrupterの場合、EventRingの動作は停止できません。EventRingは未定義の動作を起こします。
///
/// [Xhci Document] : 427 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u32)]
#[offset_bit(64)]
pub struct EventRingSegmentTableSize(usize, PhantomData<InterrupterRegisterSetOffset>);

impl EventRingSegmentTableSize {
    pub fn event_ring_segment_table_size(&self) -> u16 {
        mask_16_bits(self.read_volatile())
    }

    pub fn update_event_ring_segment_table_size(&self, set_size: u16) -> PciResult {
        // // TODO テーブルの要素数を設定する
        // let max = erst_max.max_entries();
        // if max < set_size as u32 {
        //     return Err(PciError::FailedOperateToRegister(
        //         OperationReason::ExceedsEventRingSegmentTableMax {
        //             max,
        //             value: set_size,
        //         },
        //     ));
        // }

        self.write_volatile((set_size & 0xFF_FF) as u32);

        wait_update_32bits_register_for(10, set_size as u32, self)
    }
}

fn mask_16_bits(addr: u32) -> u16 {
    (addr & 0xFF_FF) as u16
}

#[cfg(test)]
mod tests {
    use crate::xhci::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_size::mask_16_bits;

    #[test]
    fn it_mask_16_bits() {
        let addr: u32 = 0b1000_0000_0000_0000_1000_1111_0000_1111;
        assert_eq!(mask_16_bits(addr), 0b1000_1111_0000_1111);
    }
}
