use core::fmt::Debug;

#[derive(Debug)]
pub enum Error{
    /// エラーの中身が()の場合
    Void,
    /// UEFI-rsによってファイルに書き込む際のエラー
    SfsFileWrite(usize)
}








