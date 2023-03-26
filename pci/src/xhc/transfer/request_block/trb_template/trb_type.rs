use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhci::transfer::request_block::trb_template::TrbAddr;

/// TRB Types
///
///
///
/// [Xhci Document] : 511 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(Debug)]
pub enum TrbType {
    Reserved,
    Normal,
    SetupStage,
    DataStage,
    StatusStage,
    Isoch,
    Link,
    EventData,
    NoOp,
    EnableSlotCommand,
    DisableSlotCommand,
    AddressDeviceCommand,
    ConfigureEndpointCommand,
}

impl TrbType {
    #[allow(dead_code)]
    pub fn new(trb_addr: TrbAddr) -> PciResult<TrbType> {
        let trb_type = TrbTypeBit::new_uncheck(trb_addr.addr()).read_volatile();
        match trb_type {
            1 => Ok(TrbType::Normal),
            2 => Ok(TrbType::SetupStage),
            3 => Ok(TrbType::DataStage),
            4 => Ok(TrbType::StatusStage),
            5 => Ok(TrbType::Isoch),
            6 => Ok(TrbType::Link),
            7 => Ok(TrbType::EventData),
            8 => Ok(TrbType::NoOp),
            9 => Ok(TrbType::EnableSlotCommand),
            10 => Ok(TrbType::DisableSlotCommand),
            11 => Ok(TrbType::AddressDeviceCommand),
            12 => Ok(TrbType::ConfigureEndpointCommand),
            _ => Err(PciError::InvalidRegister(
                InvalidRegisterReason::InvalidAddress {
                    specified_address: trb_addr.addr(),
                },
            )),
        }
    }
}

///
///
#[derive(VolatileBits)]
#[bits(1)]
#[add_addr_bytes(0x0C)]
struct TrbTypeBit(usize, PhantomData<TrbAddr>);
