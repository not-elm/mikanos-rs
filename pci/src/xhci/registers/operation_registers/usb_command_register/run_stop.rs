use macros::VolatileFlag;

#[derive(VolatileFlag)]
pub struct RunStop(usize);
