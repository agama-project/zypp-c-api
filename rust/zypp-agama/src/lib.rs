use std::{
    ffi::CString, os::raw::{c_char, c_int, c_uint, c_void}, ptr::null_mut
};

use zypp_agama_sys::{ DownloadProgressCallbacks, ProgressCallback, Status, Status_STATE_STATE_SUCCEED, ZyppDownloadFinishCallback, ZyppDownloadProblemCallback, ZyppDownloadProgressCallback, ZyppDownloadStartCallback, PROBLEM_RESPONSE, PROBLEM_RESPONSE_PROBLEM_ABORT, PROBLEM_RESPONSE_PROBLEM_IGNORE, PROBLEM_RESPONSE_PROBLEM_RETRY};

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

// Default progress that do nothing
pub struct EmptyDownloadProgress;
impl DownloadProgress for EmptyDownloadProgress {}

unsafe extern "C" fn download_progress_start<F>(
    url: *const c_char,
    localfile: *const c_char,
    user_data: *mut c_void,
) where
    F: FnMut(String, String),
{
    let user_data = &mut *(user_data as *mut F);
    user_data(string_from_ptr(url), string_from_ptr(localfile));
}

fn get_download_progress_start<F>(_closure: &F) -> ZyppDownloadStartCallback
where
    F: FnMut(String, String),
{
    Some(download_progress_start::<F>)
}

unsafe extern "C" fn download_progress_progress<F>(
    value: c_int,
    url: *const c_char,
    bps_avg: f64,
    bps_current: f64,
    user_data: *mut c_void,
) -> c_int
where
    F: FnMut(i32, String, f64, f64) -> bool,
{
    let user_data = &mut *(user_data as *mut F);
    let res = user_data(value.into(), string_from_ptr(url), bps_avg, bps_current);
    // C type boolean
    if res {
        1 as c_int
      } else {
        0 as c_int
      }
}

fn get_download_progress_progress<F>(_closure: &F) -> ZyppDownloadProgressCallback
where
F: FnMut(i32, String, f64, f64) -> bool,
{
    Some(download_progress_progress::<F>)
}

unsafe extern "C" fn download_progress_problem<F>(
    url: *const c_char,
    error: c_int,
    description: *const c_char,
    user_data: *mut c_void,
) -> PROBLEM_RESPONSE
where
    F: FnMut(String, c_int, String) -> ProblemResponse,
{
    let user_data = &mut *(user_data as *mut F);
    let res = user_data(string_from_ptr(url), error.into(), string_from_ptr(description));
    match res {
      ProblemResponse::ABORT => PROBLEM_RESPONSE_PROBLEM_ABORT,
      ProblemResponse::IGNORE => PROBLEM_RESPONSE_PROBLEM_IGNORE,
      ProblemResponse::RETRY => PROBLEM_RESPONSE_PROBLEM_RETRY,
    }
}

fn get_download_progress_problem<F>(_closure: &F) -> ZyppDownloadProblemCallback
where
F: FnMut(String, c_int, String) -> ProblemResponse,
{
    Some(download_progress_problem::<F>)
}

unsafe extern "C" fn download_progress_finish<F>(
    url: *const c_char,
    error: c_int,
    reason: *const c_char,
    user_data: *mut c_void,
)
where
    F: FnMut(String, c_int, String),
{
    let user_data = &mut *(user_data as *mut F);
    user_data(string_from_ptr(url), error.into(), string_from_ptr(reason));    
}

fn get_download_progress_finish<F>(_closure: &F) -> ZyppDownloadFinishCallback
where
F: FnMut(String, c_int, String),
{
    Some(download_progress_finish::<F>)
}

fn with_c_download_callbacks<R, F>(callbacks: &impl DownloadProgress, block: &mut F) -> R
where 
    F: FnMut(DownloadProgressCallbacks) -> R
{
    let mut start_call = | url: String, localfile: String| callbacks.start(&url, &localfile);
    let cb_start = get_download_progress_start(&start_call);
    let mut progress_call = | value, url: String, bps_avg, bps_current| callbacks.progress(value, &url, bps_avg, bps_current);
    let cb_progress = get_download_progress_progress(&progress_call);
    let mut problem_call = | url: String, error, description: String| callbacks.problem(&url, error, &description);
    let cb_problem = get_download_progress_problem(&problem_call);
    let mut finish_call = | url: String, error, description: String| callbacks.finish(&url, error, &description);
    let cb_finish = get_download_progress_finish(&finish_call);

    let callbacks = DownloadProgressCallbacks {
        start: cb_start,
        start_data: &mut start_call as *mut _ as *mut c_void,
        progress: cb_progress,
        progress_data: &mut progress_call as *mut _ as *mut c_void,
        problem: cb_problem,
        problem_data: &mut problem_call as *mut _ as *mut c_void,
        finish: cb_finish,
        finish_data: &mut finish_call as *mut _ as *mut c_void,
    };
    block(callbacks)
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

pub fn init_target<F>(root: &str, progress: F) -> Result<(), ZyppError>
where
    // cannot be FnOnce, the whole point of progress callbacks is
    // to provide feedback multiple times
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

pub fn refresh_repository(alias: &str, progress: &impl DownloadProgress) -> Result<(), ZyppError> {
    unsafe {
        let mut status: Status = Status { state: Status_STATE_STATE_SUCCEED, error: null_mut() };
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        let mut refresh_fn = |mut callbacks| zypp_agama_sys::refresh_repository(c_alias.as_ptr(), status_ptr, &mut callbacks);
        with_c_download_callbacks(progress, &mut refresh_fn);
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
        let repos = list_repositories();
        println!("{} repos", repos.len());
        assert!(repos.len() > 10); // FIXME: just my quick validation
    }
}
