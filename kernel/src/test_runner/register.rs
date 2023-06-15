use kernel_lib::register::read::read_cr3;

#[test_case]
fn it_read_cr3() {
    read_cr3();
}


macro_rules! test_write {
    ($register: ident) => {
        paste::paste! {
            #[test_case]
            fn [<it_write_ $register>]() {
                unsafe{kernel_lib::register::write::[<write_ $register>](0x10)};
                assert_eq!(kernel_lib::register::read::[<read_ $register>](), 0x10);
            }
        }
    };
}


test_write!(rax);
test_write!(rbx);
test_write!(rcx);
test_write!(rdx);
test_write!(rsi);
test_write!(rdi);
test_write!(r8);
test_write!(r9);
test_write!(r10);
test_write!(r11);
test_write!(r12);
test_write!(r13);
test_write!(r14);
test_write!(r15);
