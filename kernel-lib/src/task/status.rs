#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Status {
    Sleep = 0,

    Pending = 1,

    Running = 2,
}


impl Status {
    #[inline(always)]
    pub const fn is_running(&self) -> bool {
        matches!(self, Status::Running)
    }


    #[inline(always)]
    pub const fn is_pending(&self) -> bool {
        matches!(self, Status::Pending)
    }


    #[inline(always)]
    pub const fn is_sleep(&self) -> bool {
        matches!(self, Status::Sleep)
    }
}


