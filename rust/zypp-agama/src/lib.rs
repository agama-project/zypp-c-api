use std::{
    ffi::CString, os::raw::{c_char, c_uint, c_void}, ptr::null_mut
};

use zypp_agama_sys::{set_zypp_download_callbacks, DownloadProgressCallbacks, ProgressCallback, Status, Status_STATE_STATE_SUCCEED};

pub mod errors;
pub use errors::ZyppError;

mod helpers;
use helpers::string_from_ptr;
// TODO: split file

pub enum ProblemResponse {
    RETRY,
    ABORT,
    IGNORE,
}

// generic trait to 
pub trait DownloadProgress {
    // callback when download start
    fn start(&self, _url: &str, _localfile: &str) {}
    // callback when download is in progress
    fn progress(&self, _value: i32, _url: &str, _bps_avg: f64, _bps_current: f64) -> bool {
        true
    }
    // callback when problem occurs
    fn problem(&self, _url: &str, _error_id: i32, _description: &str) -> ProblemResponse {
        ProblemResponse::ABORT
    }
    // callback when download finishes either successfully or with error
    fn finish(&self, _url: &str, _error_id: i32, _reason: &str) {}
}

pub struct EmptyDownloadProgress;
impl DownloadProgress for EmptyDownloadProgress {}

unsafe extern "C" fn download_callback_start(url: *const c_char, localfile: *const c_char, user_data: *mut c_void) {
    let handler = user_data as *mut _ as *mut Box<dyn DownloadProgress>;
    let r_handler: Box<Box<dyn DownloadProgress>> = Box::from_raw(handler);
    r_handler.start(&string_from_ptr(url), &string_from_ptr(localfile));
    // leak box pointer as it will be needed later in other callbacks and destructed properly at the end
    Box::leak(r_handler);
}

pub fn set_download_callbacks(callbacks: impl DownloadProgress) {
    let box_callbacks = Box::new(callbacks);
    unsafe {
        // we need double box to ensure that fat pointer caused by dyn is enclused to thin pointer
        // maybe there is better way?
        let double_box = Box::new(box_callbacks);
        let user_data = Box::into_raw(double_box);
        let c_data = user_data as *mut c_void;
        let c_callbacks = DownloadProgressCallbacks {
            start: Some(download_callback_start),
            progress: None,
            problem: None,
            finish: None,
            user_data: c_data,
        };
        set_zypp_download_callbacks(c_callbacks);
    }
}

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
        return helpers::status_to_result_void(status);
    }
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
        return helpers::status_to_result_void(status);
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
