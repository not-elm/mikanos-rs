use crate::error::OldPciResult;

pub trait RegistersOperation {
    fn reset(&mut self) -> OldPciResult;
    fn run(&mut self) -> OldPciResult;
}
