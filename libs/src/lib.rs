#![feature(pointer_byte_offsets)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod elf;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
