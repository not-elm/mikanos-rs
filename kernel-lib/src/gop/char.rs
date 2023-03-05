use crate::gop::char::char_writable::CharWritable;

pub mod ascii_char_writer;
pub mod char_writable;
pub mod mock_char_writer;

pub fn new_char_writer() -> impl CharWritable {
    #[cfg(not(test))]
    return crate::gop::char::ascii_char_writer::AscIICharWriter::new();
    #[cfg(test)]
    return crate::gop::char::mock_char_writer::MockCharWriter::new();
}
