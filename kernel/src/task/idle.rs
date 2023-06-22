use common_lib::assembly::hlt;

pub extern "sysv64" fn idle(_id: u64, _data: u64) {
    loop {
        hlt();
    }
}
