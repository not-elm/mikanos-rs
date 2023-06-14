macro_rules! write_register {
    ($register: ident) => {
        paste::paste!{
            #[inline(always)]
            pub unsafe fn [<write_ $register>](v: u64)  {
               core::arch::asm!(
                        concat!("mov ", stringify!($register), ", {}"),
                        in(reg) v,
                        options(nostack,  preserves_flags)
               );
            }
        }
    };
}


write_register!(rax);



#[cfg(test)]
mod tests {
    use crate::register::read::{read_rax, };
    use crate::register::write::write_rax;

    #[test]
    fn it_write_rax() {
        unsafe{write_rax(0x31)};
        assert_eq!(read_rax(), 0x31);
    }
}