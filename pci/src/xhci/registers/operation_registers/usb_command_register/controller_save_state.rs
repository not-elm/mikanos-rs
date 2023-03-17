use macros::VolatileFlag;

#[derive(VolatileFlag)]
pub struct ControllerSaveState(usize);
