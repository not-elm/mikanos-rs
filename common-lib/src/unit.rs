pub const fn kib(bytes: usize) -> usize {
    1024 * bytes
}

pub const fn mib(bytes: usize) -> usize {
    1024 * kib(bytes)
}


pub const fn gib(bytes: usize) -> usize {
    1024 * mib(bytes)
}
