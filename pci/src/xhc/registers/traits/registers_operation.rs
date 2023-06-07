use crate::error::PciResult;

pub trait RegistersOperation {
    fn reset(&mut self) -> PciResult;
    fn run(&mut self) -> PciResult;
}
