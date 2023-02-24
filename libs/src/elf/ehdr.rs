use crate::elf::{ELF64_Addr, ELF64_Off};

const EI_NIDENT: usize = 16;

#[repr(C)]
pub struct Ehdr {
    pub e_indent: [u8; EI_NIDENT],
    
    /// オブジェクトファイルの種類を表します。
    pub e_type: EType,

    /// 個々のファイル
    /// FIXME アーキテクチャの種類を表す列挙体を定義
    pub e_machine: u16,

    /// ELF仕様のバージョン番号
    pub e_version: u32,

    /// エントリーポイントのアドレス
    pub e_entry: ELF64_Addr,

    /// プログラムヘッダのオフセット
    /// 存在しない場合は0になります。
    pub e_phoff: ELF64_Off,

    /// セクションヘッダのオフセット
    /// 存在しない場合は、0になります。
    pub e_shoff: ELF64_Off,

    /// ファイルに関連付けられたプロセッサ固有のフラグ
    /// 現在のところはこのフラグは定義されていない。
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_ephentsize: u16,
    /// プログラムヘッダの数
    pub e_phnum: u16,

    ///　セクションヘッダーのサイズを表します。
    pub e_shentsize: u16,

    /// セクションヘッダーテーブルにあるエントリの数
    pub e_shnum: u16,

    /// セクション名テーブルのエントリのインデックス
    pub e_shstrndx: u16,
}

#[repr(u16)]
pub enum EType {
    /// No File Type
    ETNone = 0,

    /// 再配置可能ファイル
    EtRel = 1,

    ///実行可能ファイル
    EtExec = 2,

    /// 共有オブジェクトファイル
    ETDyn = 3,

    /// コアファイル
    EtCore = 4,

    /// プロセッサに固有
    EtLoproc = 0xff00,

    /// プロセッサに固有
    EiHiproc = 0xffff,
}


#[cfg(test)]
mod tests {
    use crate::elf::ehdr::Ehdr;

    #[test]
    fn it_size() {
        let size = core::mem::size_of::<Ehdr>();
        assert_eq!(size, 0x40);
    }
}
