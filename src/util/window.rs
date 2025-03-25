use core_foundation::{
    array::CFArray,
    base::{CFTypeRef, TCFType},
    dictionary::CFDictionaryGetValue,
    number::CFNumber,
    string::CFString,
};
use core_graphics::display::{
    kCGNullWindowID, kCGWindowListOptionOnScreenOnly, CGWindowListCopyWindowInfo,
};

use std::{os::raw::c_void, process::Command};

use crate::model::WindowInfo;

use super::{get_double_from_cf_dict, get_mouse_position, point_in_rect};

pub unsafe fn find_window_under_mouse(mouse_x: f64, mouse_y: f64) -> Option<WindowInfo> {
    let window_info_ptr =
        CGWindowListCopyWindowInfo(kCGWindowListOptionOnScreenOnly, kCGNullWindowID);
    let window_info: CFArray<CFTypeRef> = CFArray::wrap_under_get_rule(window_info_ptr);

    for i in 0..window_info.len() {
        let item = window_info.get(i).expect("L'élément doit exister");
        let dict_ptr: *const c_void = *item;
        let dict =
            core_foundation::dictionary::CFDictionary::<CFString, CFTypeRef>::wrap_under_get_rule(
                dict_ptr as *const _,
            );

        // Filtre sur le layer.
        let key_layer = CFString::new("kCGWindowLayer");
        let layer_ptr = CFDictionaryGetValue(
            dict.as_concrete_TypeRef(),
            key_layer.as_concrete_TypeRef() as *const c_void,
        );
        if !layer_ptr.is_null() {
            let layer_cf: CFNumber = TCFType::wrap_under_get_rule(layer_ptr as *const _);
            if let Some(layer) = layer_cf.to_f64() {
                if layer != 0.0 {
                    continue;
                }
            }
        }

        // Récupération des bounds.
        let key_bounds = CFString::new("kCGWindowBounds");
        let bounds_ptr = CFDictionaryGetValue(
            dict.as_concrete_TypeRef(),
            key_bounds.as_concrete_TypeRef() as *const c_void,
        );
        if bounds_ptr.is_null() {
            continue;
        }
        let bounds_dict =
            core_foundation::dictionary::CFDictionary::<CFString, CFTypeRef>::wrap_under_get_rule(
                bounds_ptr as *const _,
            );
        let x = get_double_from_cf_dict(&bounds_dict, "X");
        let y = get_double_from_cf_dict(&bounds_dict, "Y");
        let width = get_double_from_cf_dict(&bounds_dict, "Width");
        let height = get_double_from_cf_dict(&bounds_dict, "Height");

        if point_in_rect(mouse_x, mouse_y, x, y, width, height) {
            // Récupération du nom du propriétaire.
            let key_owner = CFString::new("kCGWindowOwnerName");
            let owner_ptr = CFDictionaryGetValue(
                dict.as_concrete_TypeRef(),
                key_owner.as_concrete_TypeRef() as *const c_void,
            );
            let owner = if !owner_ptr.is_null() {
                let owner_cf: CFString = TCFType::wrap_under_get_rule(owner_ptr as *const _);
                owner_cf.to_string()
            } else {
                "Unknown".to_string()
            };

            return Some(WindowInfo {
                owner,
                x,
                y,
                width,
                height,
            });
        }
    }
    None
}

pub fn focus_app(app_name: &str) {
    let mouse_pos = get_mouse_position();
    let script = format!(
        r#"
set targetX to {mouse_x}
set targetY to {mouse_y}
tell application "System Events"
    tell application process "{app_name}"
        set bestWindow to missing value
        set bestDist to 1e10
        repeat with w in windows
            try
                set pos to position of w
                set siz to size of w
                set centerX to (item 1 of pos) + ((item 1 of siz) / 2)
                set centerY to (item 2 of pos) + ((item 2 of siz) / 2)
                set dx to centerX - targetX
                set dy to centerY - targetY
                set dist to (dx * dx) + (dy * dy)
                if dist < bestDist then
                    set bestDist to dist
                    set bestWindow to w
                end if
            end try
        end repeat
        if bestWindow is not missing value then
            tell bestWindow to perform action "AXRaise"
        end if
        set frontmost to true
    end tell
end tell
"#,
        mouse_x = mouse_pos.x,
        mouse_y = mouse_pos.y,
        app_name = app_name
    );
    let output = Command::new("osascript").arg("-e").arg(&script).output();
    match output {
        Ok(output) if output.status.success() => {
            println!("Focus AppleScript réussi pour \"{}\".", app_name);
        }
        Ok(output) => {
            eprintln!(
                "Focus AppleScript échoué pour \"{}\": {}",
                app_name,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            eprintln!("Échec d'exécution d'osascript: {}", e);
        }
    }
}
