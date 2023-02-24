// #[cfg(test)]
// pub mod tests {
//     #[test]
//     fn test() {
//         #[cfg(test)]
//         pub mod tests {
//             use std::assert_eq;
//             use crate::elf::phdr::Phdr;
//
//             #[test]
//             fn it_equal_struct_size() {
//                 let phdr_size = core::mem::size_of::<Phdr>();
//                 assert_eq!(phdr_size, 56);
//             }
//         }
//     }
// }