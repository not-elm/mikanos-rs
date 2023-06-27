use kernel_lib::register::read::read_cr3;

mod rflags;

#[test_case]
fn it_read_cr3() {
    read_cr3();
}
