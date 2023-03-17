use macros::VolatileFlag;

#[derive(VolatileFlag)]
pub struct InterrupterEnable(usize);
