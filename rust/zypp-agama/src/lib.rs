use std::{
    error::Error, ffi::{CStr, CString}, fmt, os::raw::{c_char, c_uint, c_void}, ptr::null_mut
};

use zypp_agama_sys::{free_status, ProgressCallback, Status, Status_STATE_STATE_SUCCEED};

pub struct Repository {
    pub url: String,
    pub alias: String,
    pub user_name: String,
}

// TODO: is there better way how to use type from ProgressCallback binding type?
unsafe extern "C" fn progress_callback<F>(
    text: *const c_char,
    stage: c_uint,
    total: c_uint,
    user_data: *mut c_void,
) where
    F: FnMut(String, u32, u32),
{
    let user_data = &mut *(user_data as *mut F);
    user_data(string_from_ptr(text), stage.into(), total.into());
}

fn get_progress_callback<F>(_closure: &F) -> ProgressCallback
where
    F: FnMut(String, u32, u32),
{
    Some(progress_callback::<F>)
}

#[derive(Debug)]
pub struct ZyppError {
    details: String
}

impl ZyppError {
    fn new(msg: &str) -> ZyppError {
        ZyppError{details: msg.to_string()}
    }
}

impl fmt::Display for ZyppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ZyppError {
    fn description(&self) -> &str {
        &self.details
    }
}

unsafe fn status_to_result_void(status: Status) -> Result<(), ZyppError> {
    let res = if status.state == Status_STATE_STATE_SUCCEED {
        Ok(())
    } else {
        Err(ZyppError::new(string_from_ptr(status.error).as_str()))
    };
    free_status(status);
    return res;
} 

// TODO: use result
pub fn init_target<F>(root: &str, progress: F) -> Result<(), ZyppError>
where
    F: FnMut(String, u32, u32),
{
    unsafe {
        let mut closure = progress;
        let cb = get_progress_callback(&closure);
        let c_root = CString::new(root).unwrap();
        let mut status: Status = Status { state: Status_STATE_STATE_SUCCEED, error: null_mut() };
        let status_ptr = &mut status as *mut _ as *mut Status;
        zypp_agama_sys::init_target(c_root.as_ptr(), status_ptr, cb, &mut closure as *mut _ as *mut c_void);
        return status_to_result_void(status);
    }
}

unsafe fn string_from_ptr(c_ptr: *const i8) -> String {
    String::from_utf8_lossy(CStr::from_ptr(c_ptr).to_bytes()).into_owned()
}

// TODO: use result
pub fn list_repositories() -> Vec<Repository> {
    let mut res = vec![];

    unsafe {
        let mut repos = zypp_agama_sys::list_repositories();
        // unwrap is ok as it will crash only on less then 32b archs,so safe for agama
        let size_usize: usize = repos.size.try_into().unwrap();
        for i in 0..size_usize {
            let c_repo = *(repos.repos.add(i));
            let r_repo = Repository {
                url: string_from_ptr(c_repo.url),
                alias: string_from_ptr(c_repo.alias),
                user_name: string_from_ptr(c_repo.userName),
            };
            res.push(r_repo);
        }
        let repos_rawp = &mut repos;
        zypp_agama_sys::free_repository_list(
            repos_rawp as *mut _ as *mut zypp_agama_sys::RepositoryList,
        );
    }

    res
}

pub fn refresh_repository(alias: &str) -> Result<(), ZyppError> {
    unsafe {
        let mut status: Status = Status { state: Status_STATE_STATE_SUCCEED, error: null_mut() };
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        zypp_agama_sys::refresh_repository(c_alias.as_ptr(), status_ptr);
        return status_to_result_void(status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn progress_cb(_text: String, _stage: u32, _total: u32) {
        // for test do nothing..maybe some check of callbacks?
    }

    #[test]
    fn it_works() {
        init_target("/", progress_cb).unwrap();
        let result = list_repositories();
        assert_eq!(result.len(), 24); // FIXME: just my quick validation
    }
}
