
use core::ffi::{c_char};

extern "C" {
     fn get_font(c: c_char) -> *mut u8;
}


pub fn get_font_from(mut c: char) -> Option<*mut u8>{
    if !c.is_ascii(){
        return None;
    }

    if c.is_ascii_lowercase(){
        c.make_ascii_lowercase();
    }else{
        c.make_ascii_uppercase();
    }

    let char_ptr = unsafe{get_font(c as c_char)};
    if char_ptr == core::ptr::null_mut(){
        return None;
    }

    Some(char_ptr)
}


#[cfg(test)]
mod tests {
    use crate::gop::font::get_font_from;


    #[test]
    fn it_get_pritable_ascii_codes() {
        let get_all_pritable_ascii_codes = (0x20..0x7Eu8).all(|code| get_font_from(char::from(code)).is_some());

        assert!(get_all_pritable_ascii_codes);
    }


    #[test]
    fn it_failed_get_char() {
        assert!(get_font_from('�').is_none());
    }

    #[test]
    fn it_failed_over_ascii_range() {
        assert!( get_font_from(char::from(0x80)).is_none());
    }
}