pub mod ehdr;
pub mod phdr;
pub mod ehdr_iter;

/// プログラムアドレス(64bitアーキテクチャ)
pub type Elf64Addr = u64;


/// ファイルオフセット(64bitアーキテクチャ)
pub type Elf64Off = u64;
