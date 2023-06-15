macro_rules! write_register {
    ($register: ident) => {
        paste::paste! {
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
write_register!(rbx);
write_register!(rcx);
write_register!(rdx);
write_register!(rsi);
write_register!(rbp);
write_register!(rdi);
write_register!(rsp);
write_register!(r8);
write_register!(r9);
write_register!(r10);
write_register!(r11);
write_register!(r12);
write_register!(r13);
write_register!(r14);
write_register!(r15);
