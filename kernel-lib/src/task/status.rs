#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Status {
    Running,

    Pending,

    Sleep,
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


