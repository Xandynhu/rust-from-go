use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// function to free the memory allocated
#[no_mangle]
pub extern "C" fn free_string(msg: *const c_char) {
    if msg.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(msg as *mut c_char);
    }
}

#[no_mangle]
pub extern "C" fn print(msg: *const libc::c_char) {
    let message_cstr = unsafe { CStr::from_ptr(msg) };
    let msg_str = message_cstr.to_str().unwrap();

    if msg_str == "!" {
        return;
    }

    println!("({})", msg_str);
}

#[no_mangle]
pub extern "C" fn process_json(json: *const libc::c_char) -> *const libc::c_char {
    let json_str = unsafe { CStr::from_ptr(json) };
    // if empty, return "is empty", else, return "is not empty !!"
    if json_str.to_str().unwrap().is_empty() {
        return CString::new("is empty").unwrap().into_raw();
    } else {
        return CString::new("is not empty !!").unwrap().into_raw();
    }
}