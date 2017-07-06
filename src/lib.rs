#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use self::bindings::*;

mod bindings;

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn can_create_mcu_by_name() {
        let mcu_name = CString::new("atmega328p").unwrap();

        unsafe {
            let mcu = avr_make_mcu_by_name(mcu_name.as_ptr());
        }
    }
}
