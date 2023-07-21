use kernel_lib::control_registers::read_cr3;
use kernel_lib::paging::{clean_page_maps, setup_page_maps};
use kernel_lib::paging::linear_address::LinearAddress;

#[test_case]
fn it_setup_page() {
    let addr = 0xFFFF800000000000;
    setup_page_maps(LinearAddress::new(addr), 1);
    unsafe {
        let ptr = addr as *mut u8;
        ptr.write(3);
        assert_eq!(ptr.read_volatile(), 3);
        ptr.drop_in_place();

        clean_page_maps(LinearAddress::new(addr));
        let table = read_cr3() as *mut u64;
        assert_eq!(table.add(LinearAddress::new(addr).part(4)).read(), 0);
    };
}