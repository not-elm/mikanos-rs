use kernel_lib::error::{KernelError, KernelResult};
use multiple_function_device::MultipleFunctionDevice;

use crate::configuration_space::device::function::single_function_device::SingleFunctionDevice;

pub mod multiple_function_device;
pub mod single_function_device;

#[derive(Debug)]
pub enum Function {
    Single(SingleFunctionDevice),
    Multiple(MultipleFunctionDevice),
}

impl Function {
    pub fn expect_single(self) -> KernelResult<SingleFunctionDevice> {
        if let Function::Single(single) = self {
            Ok(single)
        } else {
            Err(KernelError::NotSingleFunction)
        }
    }
}
