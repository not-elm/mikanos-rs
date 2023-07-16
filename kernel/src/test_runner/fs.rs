use kernel_lib::fs;
use kernel_lib::simple_fat::dir::entry::short::ShortDirEntryReadable;

#[test_case]
fn it_read_root_dir() {
    let mut dir = fs::root_dir().unwrap();

    dir.next();

    let hello_txt_file = dir
        .next()
        .unwrap()
        .into_regular_file()
        .unwrap()
        .name()
        .unwrap();

    assert_eq!(
        hello_txt_file
            .to_str()
            .unwrap(),
        "HELLO.TXT"
    );
}


#[test_case]
fn it_exists_hlt_elf() {
    let hlt = fs::open_file("HLT.ELF")
        .unwrap()
        .name()
        .unwrap();

    let name = hlt.to_str().unwrap();
    assert_eq!(name, "HLT.ELF");
}
