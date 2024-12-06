use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_uint, c_void},
    ptr::null_mut,
};

pub use callbacks::DownloadProgress;
use zypp_agama_sys::{
    ProgressCallback, ProgressData, Status, Status_STATE_STATE_SUCCEED, ZyppProgressCallback,
};

pub mod errors;
pub use errors::ZyppError;

mod helpers;
use helpers::string_from_ptr;

mod callbacks;

pub struct Repository {
    pub url: String,
    pub alias: String,
    pub user_name: String,
}

// TODO: is there better way how to use type from ProgressCallback binding type?
unsafe extern "C" fn zypp_progress_callback<F>(
    zypp_data: ProgressData,
    user_data: *mut c_void,
) -> c_int
where
    F: FnMut(i64, String) -> bool,
{
    let user_data = &mut *(user_data as *mut F);
    let res = user_data(zypp_data.value, string_from_ptr(zypp_data.name));
    if res {
        1 as c_int
    } else {
        0 as c_int
    }
}

fn get_zypp_progress_callback<F>(_closure: &F) -> ZyppProgressCallback
where
    F: FnMut(i64, String) -> bool,
{
    Some(zypp_progress_callback::<F>)
}

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
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _ as *mut Status;
        zypp_agama_sys::init_target(
            c_root.as_ptr(),
            status_ptr,
            cb,
            &mut closure as *mut _ as *mut c_void,
        );
        return helpers::status_to_result_void(status);
    }
}

pub fn list_repositories() -> Result<Vec<Repository>, ZyppError> {
    let mut repos_v = vec![];

    unsafe {
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _ as *mut Status;

        let mut repos = zypp_agama_sys::list_repositories(status_ptr);
        // unwrap is ok as it will crash only on less then 32b archs,so safe for agama
        let size_usize: usize = repos.size.try_into().unwrap();
        for i in 0..size_usize {
            let c_repo = *(repos.repos.add(i));
            let r_repo = Repository {
                url: string_from_ptr(c_repo.url),
                alias: string_from_ptr(c_repo.alias),
                user_name: string_from_ptr(c_repo.userName),
            };
            repos_v.push(r_repo);
        }
        let repos_rawp = &mut repos;
        zypp_agama_sys::free_repository_list(
            repos_rawp as *mut _ as *mut zypp_agama_sys::RepositoryList,
        );

        let res = if status.state == zypp_agama_sys::Status_STATE_STATE_SUCCEED {
            Ok(repos_v)
        } else {
            Err(crate::ZyppError::new(
                string_from_ptr(status.error).as_str(),
            ))
        };
        zypp_agama_sys::free_status(status_ptr);

        res
    }
}

pub fn refresh_repository(alias: &str, progress: &impl DownloadProgress) -> Result<(), ZyppError> {
    unsafe {
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        let mut refresh_fn = |mut callbacks| {
            zypp_agama_sys::refresh_repository(c_alias.as_ptr(), status_ptr, &mut callbacks)
        };
        callbacks::with_c_download_callbacks(progress, &mut refresh_fn);
        return helpers::status_to_result_void(status);
    }
}

pub fn add_repository<F>(alias: &str, url: &str, progress: F) -> Result<(), ZyppError>
where
    F: FnMut(i64, String) -> bool,
{
    unsafe {
        let mut closure = progress;
        let cb = get_zypp_progress_callback(&closure);
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        let c_url = CString::new(url).unwrap();
        zypp_agama_sys::add_repository(
            c_alias.as_ptr(),
            c_url.as_ptr(),
            status_ptr,
            cb,
            &mut closure as *mut _ as *mut c_void,
        );
        return helpers::status_to_result_void(status);
    }
}

pub fn remove_repository<F>(alias: &str, progress: F) -> Result<(), ZyppError>
where
    F: FnMut(i64, String) -> bool,
{
    unsafe {
        let mut closure = progress;
        let cb = get_zypp_progress_callback(&closure);
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        zypp_agama_sys::remove_repository(
            c_alias.as_ptr(),
            status_ptr,
            cb,
            &mut closure as *mut _ as *mut c_void,
        );
        return helpers::status_to_result_void(status);
    }
}

pub fn create_repo_cache<F>(alias: &str, progress: F) -> Result<(), ZyppError>
where
    F: FnMut(i64, String) -> bool,
{
    unsafe {
        let mut closure = progress;
        let cb = get_zypp_progress_callback(&closure);
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        zypp_agama_sys::build_repository_cache(
            c_alias.as_ptr(),
            status_ptr,
            cb,
            &mut closure as *mut _ as *mut c_void,
        );
        return helpers::status_to_result_void(status);
    }
}

pub fn load_repo_cache(alias: &str) -> Result<(), ZyppError>
{
    unsafe {
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        zypp_agama_sys::load_repository_cache(
            c_alias.as_ptr(),
            status_ptr,
        );
        return helpers::status_to_result_void(status);
    }
}

pub enum ResolvableKind {
    Package,
    Pattern,
    SrcPackage,
    Patch,
    Product,
}

impl Into<zypp_agama_sys::RESOLVABLE_KIND> for ResolvableKind {
    fn into(self) -> zypp_agama_sys::RESOLVABLE_KIND {
        match self {
            Self::Package => zypp_agama_sys::RESOLVABLE_KIND_RESOLVABLE_PACKAGE,
            Self::SrcPackage => zypp_agama_sys::RESOLVABLE_KIND_RESOLVABLE_SRCPACKAGE,
            Self::Patch => zypp_agama_sys::RESOLVABLE_KIND_RESOLVABLE_PATCH,
            Self::Product => zypp_agama_sys::RESOLVABLE_KIND_RESOLVABLE_PRODUCT,
            Self::Pattern => zypp_agama_sys::RESOLVABLE_KIND_RESOLVABLE_PATTERN,
        }
    }
}

pub fn select_resolvable(name: &str, kind: ResolvableKind) -> Result<(), ZyppError> {
    unsafe {
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _;
        let c_name = CString::new(name).unwrap();
        let c_kind = kind.into();
        zypp_agama_sys::resolvable_select(c_name.as_ptr(), c_kind, status_ptr);
        return helpers::status_to_result_void(status);
    }
}

pub fn unselect_resolvable(name: &str, kind: ResolvableKind) -> Result<(), ZyppError> {
    unsafe {
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _;
        let c_name = CString::new(name).unwrap();
        let c_kind = kind.into();
        zypp_agama_sys::resolvable_unselect(c_name.as_ptr(), c_kind, status_ptr);
        return helpers::status_to_result_void(status);
    }
}

pub fn run_solver() -> Result<bool, ZyppError> {
    unsafe {
        let mut status: Status = Status {
            state: Status_STATE_STATE_SUCCEED,
            error: null_mut(),
        };
        let status_ptr = &mut status as *mut _;
        let c_res = zypp_agama_sys::run_solver(status_ptr);
        let r_res = c_res != 0;
        let result = helpers::status_to_result_void(status);
        result.and(Ok(r_res))
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
        let repos = list_repositories().unwrap();
        println!("{} repos", repos.len());
        assert!(repos.len() > 10); // FIXME: just my quick validation
    }
}
