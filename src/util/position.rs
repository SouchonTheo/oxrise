use std::os::raw::c_void;

use core_foundation::{
    base::{CFTypeRef, TCFType},
    dictionary::CFDictionaryGetValue,
    number::CFNumber,
    string::CFString,
};

pub fn point_in_rect(
    mouse_x: f64,
    mouse_y: f64,
    rect_x: f64,
    rect_y: f64,
    rect_width: f64,
    rect_height: f64,
) -> bool {
    mouse_x >= rect_x
        && mouse_x <= (rect_x + rect_width)
        && mouse_y >= rect_y
        && mouse_y <= (rect_y + rect_height)
}

pub unsafe fn get_double_from_cf_dict(
    dict: &core_foundation::dictionary::CFDictionary<CFString, CFTypeRef>,
    key: &str,
) -> f64 {
    let key_cf = CFString::new(key);
    let value_ptr = CFDictionaryGetValue(
        dict.as_concrete_TypeRef(),
        key_cf.as_concrete_TypeRef() as *const c_void,
    );
    if !value_ptr.is_null() {
        let cf_number: CFNumber = TCFType::wrap_under_get_rule(value_ptr as *const _);
        if let Some(num) = cf_number.to_f64() {
            return num;
        }
    }
    0.0
}
