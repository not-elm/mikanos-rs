use crate::gop::char::char_writable::CharWritable;

pub mod ascii_char_writer;
pub mod char_writable;
pub mod mock_char_writer;

pub fn new_char_writer() -> impl CharWritable {
    return ascii_char_writer::AscIICharWriter::new();
}
