use multiple_function_device::MultipleFunctionDevice;

use crate::pci::configuration_space::device::function::single_function_device::SingleFunctionDevice;

pub mod multiple_function_device;
pub mod single_function_device;

#[derive(Debug)]
pub enum Function {
    Single(SingleFunctionDevice),
    Multiple(MultipleFunctionDevice),
}
