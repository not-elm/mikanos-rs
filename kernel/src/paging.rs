use kernel_lib::paging::setup_identity_page_table;

pub fn init_paging_table() {
    unsafe { setup_identity_page_table() };
}
