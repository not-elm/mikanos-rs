use alloc::string::String;

use crate::error::KernelResult;
use crate::kernel_bail;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct CharBuff<const N: usize>(u64);


impl<const N: usize> CharBuff<N> {
    pub const fn new(addr: u64) -> CharBuff<N> {
        Self(addr)
    }

    pub fn new_with_check(addr: u64, s: &str) -> KernelResult<Self> {
        let me = Self::new(addr);
        if me.equal(s) {
            Ok(me)
        } else {
            kernel_bail!(
                "Invalid Chars Expect = '{s}' But Actual = '{}'",
                me.as_string()
            )
        }
    }

    pub fn equal(&self, str: &str) -> bool {
        self.as_string().as_str() == str
    }


    pub fn as_string(&self) -> String {
        let chars = unsafe { core::slice::from_raw_parts(self.0 as *const u8, N) };
        chars
            .iter()
            .map(|c| char::from(*c))
            .take_while(|c| *c != '\0')
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::acpi::volatile_chars::CharBuff;

    #[test]
    fn it_as_string() {
        let buff: &[u8; 5] = b"RUST\0";
        let chars = CharBuff::<10>::new(buff.as_ptr() as u64);
        assert_eq!(chars.as_string().as_str(), "RUST");
    }
}
