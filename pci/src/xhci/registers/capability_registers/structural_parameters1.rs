use macros::Volatile;

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u32)]
pub struct StructuralParameters1(usize);
