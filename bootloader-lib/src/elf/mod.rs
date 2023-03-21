pub mod ehdr;
pub mod phdr;

/// プログラムアドレス(64bitアーキテクチャ)
pub type Elf64Addr = u64;

/// ファイルオフセット(64bitアーキテクチャ)
pub type Elf64Off = u64;

#[cfg(test)]
pub(crate) fn load_ehdr() -> *mut crate::elf::ehdr::elf_header::ElfHeader {
    use std::io::Read;
    let path = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/resources/test/kernel.elf", path);

    let mut kernel_file = ::std::fs::File::open(path).unwrap();

    let mut buff = Vec::<u8>::new();
    kernel_file.read_to_end(&mut buff).unwrap();

    crate::elf::ehdr::elf_header::ElfHeader::from_file_buff(&mut buff)
}
