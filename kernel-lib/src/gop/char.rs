use crate::gop::char::char_writable::CharWritable;

pub mod ascii;
pub mod ascii_char_writer;
pub mod char_writable;

pub fn new_char_writer() -> impl CharWritable {
    ascii_char_writer::AscIICharWriter::new()
}
