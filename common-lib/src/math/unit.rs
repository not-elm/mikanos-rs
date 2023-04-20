pub const fn kib(bytes: usize) -> usize {
    1024 * bytes
}

pub const fn mib(bytes: usize) -> usize {
    bytes * 1024 * kib(1)
}


pub const fn gib(bytes: usize) -> usize {
    bytes * 1024 * mib(1)
}
