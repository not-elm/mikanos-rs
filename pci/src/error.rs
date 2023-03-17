pub type PciResult<T = ()> = Result<T, PciError>;

#[derive(Debug)]
pub enum PciError {
    NotSingleFunction,
    NotGeneralHeader,
}
