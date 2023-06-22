#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Status {
    Sleep,

    Pending,

    Running,
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


