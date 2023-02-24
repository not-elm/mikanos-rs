// use crate::elf::ehdr::Ehdr;
//
// pub struct EhdrIter {
//     ehdr_ptr: *mut Ehdr,
//     current: Ehdr,
// }
//
// impl EhdrIter {
//     pub fn from_file_buff(file_buff: &mut [u8]) -> Self {
//         let buff_ptr = file_buff.as_mut_ptr();
//         let ehdr_ptr = (buff_ptr) as *mut Ehdr;
//         Self::from_ptr(ehdr_ptr)
//     }
//     pub fn from_ptr(edhr_ptr: *mut Ehdr) -> Self {
//         Self {
//             ehdr_ptr: edhr_ptr,
//             current: unsafe { *edhr_ptr },
//         }
//     }
//
//
//     fn dref(&self) -> Ehdr {
//         unsafe { *(self.ehdr_ptr) }
//     }
//
//     fn step_ehdr(&mut self) {
//         unsafe {
//             self.ehdr_ptr = self.ehdr_ptr.add(1);
//         }
//     }
// }
//
// impl Iterator for EhdrIter {
//     type Item = Ehdr;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let current: Ehdr = self.dref();
//         self.
//         self.step_ehdr();
//         self.current = self.dref();
//         Some(current)
//     }
// }
//
//
// #[cfg(test)]
// pub mod tests {
//     use std::io::Read;
//
//     use crate::elf::ehdr_iter::EhdrIter;
//
//
// }