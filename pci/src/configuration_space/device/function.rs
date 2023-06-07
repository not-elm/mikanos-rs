use multiple_function_device::MultipleFunctionDevice;

use crate::configuration_space::device::function::single_function_device::SingleFunctionDevice;
use crate::error::PciResult;
use crate::pci_bail;

pub mod multiple_function_device;
pub mod single_function_device;


#[derive(Debug)]
pub enum Function {
    Single(SingleFunctionDevice),
    Multiple(MultipleFunctionDevice),
}


impl Function {
    pub fn expect_single(self) -> PciResult<SingleFunctionDevice> {
        if let Function::Single(single) = self {
            Ok(single)
        } else {
            pci_bail!("Not Single function device")
        }
    }
}
