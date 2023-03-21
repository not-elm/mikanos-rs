use core::ffi::c_char;

extern "C" {
    fn get_font(c: c_char) -> *mut u8;
}

pub fn convert_to_ascii(mut c: char) -> Option<char> {
    if !c.is_ascii() {
        return None;
    }

    if c.is_ascii_lowercase() {
        c.make_ascii_lowercase();
    } else {
        c.make_ascii_uppercase();
    }

    Some(c)
}

pub fn get_font_from(c: char) -> Option<*mut u8> {
    let char_ptr = unsafe { get_font(convert_to_ascii(c)? as c_char) };
    if char_ptr.is_null() {
        None
    } else {
        Some(char_ptr)
    }
}

// #[cfg(test)]
// mod test_runner {
//     use crate::gop::font::get_font_from;
//
//     /// 印字可能文字をすべて取得できるかのテスト
//     /// Asciiの文字コード表はMikanOSの書籍の付録に記載されています。
//     #[test]
//     fn it_get_printable_ascii_codes() {
//         let get_all_printable_ascii_codes =
//             (0x20..=0x7Eu8).all(|code| get_font_from(char::from(code)).is_some());
//
//         assert!(get_all_printable_ascii_codes);
//     }
//
//     #[test]
//     fn it_failed_get_char() {
//         assert!(get_font_from('�').is_none());
//     }
//
//     #[test]
//     fn it_failed_over_ascii_range() {
//         assert!(get_font_from(char::from(0x80)).is_none());
//     }
// }
