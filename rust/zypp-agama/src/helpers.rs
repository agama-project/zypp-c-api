use std::ptr::null_mut;


// Safety requirements: inherited from https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr
pub(crate) unsafe fn string_from_ptr(c_ptr: *const i8) -> String {
    String::from_utf8_lossy(std::ffi::CStr::from_ptr(c_ptr).to_bytes()).into_owned()
}

// Safety requirements: ...
pub(crate) unsafe fn status_to_result_void(mut status: zypp_agama_sys::Status) -> Result<(), crate::ZyppError> {
    let res = if status.state == zypp_agama_sys::Status_STATE_STATE_SUCCEED {
        Ok(())
    } else {
        Err(crate::ZyppError::new(string_from_ptr(status.error).as_str()))
    };
    let status_ptr = &mut status;
    zypp_agama_sys::free_status(status_ptr as *mut _);
    return res;
}

pub(crate) unsafe fn create_status_with_pointer() -> (zypp_agama_sys::Status, *mut zypp_agama_sys::Status) {
    let mut status: zypp_agama_sys::Status = zypp_agama_sys::Status {
        state: zypp_agama_sys::Status_STATE_STATE_SUCCEED,
        error: null_mut(),
    };
    let status_ptr = &mut status as *mut _;
    (status, status_ptr)
}
