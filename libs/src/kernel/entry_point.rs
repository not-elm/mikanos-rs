use core::fmt::LowerHex;

#[derive(Debug)]
pub struct EntryPoint(u64);

impl EntryPoint {
    pub fn new(entry_point_addr: u64) -> Self {
        Self(entry_point_addr)
    }
    pub fn execute(&self) {
        let entry_point_ptr =  self.0 as *const ();
        let entry_point: extern "C" fn() -> () = unsafe { core::mem::transmute(entry_point_ptr) };
        entry_point();
    }
}

impl LowerHex for EntryPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val = self.0;

        core::fmt::LowerHex::fmt(&val, f)
    }
}
