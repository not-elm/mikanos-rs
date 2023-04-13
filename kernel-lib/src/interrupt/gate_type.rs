
use modular_bitfield::BitfieldSpecifier;

#[derive(Debug, BitfieldSpecifier)]
#[bits = 4]
pub enum GateType {
    InterruptGate = 0x0E,
    TrapGate = 0x0F
}