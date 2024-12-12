use std::{
    ffi::CString,
    os::raw::{c_char, c_uint, c_void},
};

pub use callbacks::DownloadProgress;
use errors::ZyppResult;
use zypp_agama_sys::{
    get_patterns_info, PatternNames, ProgressCallback, ProgressData, Status, ZyppProgressCallback,
};

pub mod errors;
pub use errors::ZyppError;

mod helpers;
use helpers::{status_to_result_void, string_from_ptr};

mod callbacks;

pub struct Repository {
    pub enabled: bool,
    pub url: String,
    pub alias: String,
    pub user_name: String,
}

// TODO: is there better way how to use type from ProgressCallback binding type?
unsafe extern "C" fn zypp_progress_callback<F>(
    zypp_data: ProgressData,
    user_data: *mut c_void,
) -> bool
where
    F: FnMut(i64, String) -> bool,
{
    let user_data = &mut *(user_data as *mut F);
    let res = user_data(zypp_data.value, string_from_ptr(zypp_data.name));
    res
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

pub fn init_target<F>(root: &str, progress: F) -> ZyppResult<()>
where
    // cannot be FnOnce, the whole point of progress callbacks is
    // to provide feedback multiple times
    F: FnMut(String, u32, u32),
{
    unsafe {
        let mut closure = progress;
        let cb = get_progress_callback(&closure);
        let c_root = CString::new(root).unwrap();
        let mut status: Status = Status::default();
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

pub fn list_repositories() -> ZyppResult<Vec<Repository>> {
    let mut repos_v = vec![];

    unsafe {
        let mut status: Status = Status::default();
        let status_ptr = &mut status as *mut _ as *mut Status;

        let mut repos = zypp_agama_sys::list_repositories(status_ptr);
        // unwrap is ok as it will crash only on less then 32b archs,so safe for agama
        let size_usize: usize = repos.size.try_into().unwrap();
        for i in 0..size_usize {
            let c_repo = *(repos.repos.add(i));
            let r_repo = Repository {
                enabled: c_repo.enabled,
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

        helpers::status_to_result_void(status).and(Ok(repos_v))
    }
}

pub fn refresh_repository(alias: &str, progress: &impl DownloadProgress) -> ZyppResult<()> {
    unsafe {
        let mut status: Status = Status::default();
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        let mut refresh_fn = |mut callbacks| {
            zypp_agama_sys::refresh_repository(c_alias.as_ptr(), status_ptr, &mut callbacks)
        };
        callbacks::with_c_download_callbacks(progress, &mut refresh_fn);
        return helpers::status_to_result_void(status);
    }
}

pub fn add_repository<F>(alias: &str, url: &str, progress: F) -> ZyppResult<()>
where
    F: FnMut(i64, String) -> bool,
{
    unsafe {
        let mut closure = progress;
        let cb = get_zypp_progress_callback(&closure);
        let mut status: Status = Status::default();
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

pub fn remove_repository<F>(alias: &str, progress: F) -> ZyppResult<()>
where
    F: FnMut(i64, String) -> bool,
{
    unsafe {
        let mut closure = progress;
        let cb = get_zypp_progress_callback(&closure);
        let mut status: Status = Status::default();
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

pub fn create_repo_cache<F>(alias: &str, progress: F) -> ZyppResult<()>
where
    F: FnMut(i64, String) -> bool,
{
    unsafe {
        let mut closure = progress;
        let cb = get_zypp_progress_callback(&closure);
        let mut status: Status = Status::default();
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

pub fn load_repo_cache(alias: &str) -> ZyppResult<()> {
    unsafe {
        let mut status: Status = Status::default();
        let status_ptr = &mut status as *mut _ as *mut Status;
        let c_alias = CString::new(alias).unwrap();
        zypp_agama_sys::load_repository_cache(c_alias.as_ptr(), status_ptr);
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

pub fn select_resolvable(name: &str, kind: ResolvableKind, who: ResolvableSelected) -> ZyppResult<()> {
    unsafe {
        let mut status: Status = Status::default();
        let status_ptr = &mut status as *mut _;
        let c_name = CString::new(name).unwrap();
        let c_kind = kind.into();
        zypp_agama_sys::resolvable_select(c_name.as_ptr(), c_kind, who.into(), status_ptr);
        return helpers::status_to_result_void(status);
    }
}

pub fn unselect_resolvable(name: &str, kind: ResolvableKind, who: ResolvableSelected) -> ZyppResult<()> {
    unsafe {
        let mut status: Status = Status::default();
        let status_ptr = &mut status as *mut _;
        let c_name = CString::new(name).unwrap();
        let c_kind = kind.into();
        zypp_agama_sys::resolvable_unselect(c_name.as_ptr(), c_kind, who.into(), status_ptr);
        return helpers::status_to_result_void(status);
    }
}

#[derive(Debug)]
pub enum ResolvableSelected {
    Not,
    User,
    Installation,
    Solver,
}

impl From<zypp_agama_sys::RESOLVABLE_SELECTED> for ResolvableSelected {
    fn from(value: zypp_agama_sys::RESOLVABLE_SELECTED) -> Self {
        match value {
            zypp_agama_sys::RESOLVABLE_SELECTED_NOT_SELECTED => Self::Not,
            zypp_agama_sys::RESOLVABLE_SELECTED_USER_SELECTED => Self::User,
            zypp_agama_sys::RESOLVABLE_SELECTED_APPLICATION_SELECTED => Self::Installation,
            zypp_agama_sys::RESOLVABLE_SELECTED_SOLVER_SELECTED => Self::Solver,
            _ => panic!("Unknown value for resolvable_selected {}", value),
        }
    }
}

impl Into<zypp_agama_sys::RESOLVABLE_SELECTED> for ResolvableSelected {
    fn into(self) -> zypp_agama_sys::RESOLVABLE_SELECTED {
        match self {
            Self::Not => zypp_agama_sys::RESOLVABLE_SELECTED_NOT_SELECTED,
            Self::User => zypp_agama_sys::RESOLVABLE_SELECTED_USER_SELECTED,
            Self::Installation => zypp_agama_sys::RESOLVABLE_SELECTED_APPLICATION_SELECTED,
            Self::Solver => zypp_agama_sys::RESOLVABLE_SELECTED_SOLVER_SELECTED,
        }
    }
}

// TODO: should we add also e.g. serd serializers here?
#[derive(Debug)]
pub struct PatternInfo {
    pub name: String,
    pub category: String,
    pub icon: String,
    pub description: String,
    pub summary: String,
    pub order: String,
    pub selected: ResolvableSelected,
}

pub fn patterns_info(names: Vec<&str>) -> ZyppResult<Vec<PatternInfo>> {
    unsafe {
        let mut status: Status = Status::default();
        let status_ptr = &mut status as *mut _;
        let c_names: Vec<CString> = names
            .iter()
            .map(|s| CString::new(*s).expect("CString must not contain internal NUL"))
            .collect();
        let c_ptr_names: Vec<*const i8> = c_names.iter().map(|c| c.as_c_str().as_ptr()).collect();
        let pattern_names = PatternNames {
            size: names.len() as u32,
            names: c_ptr_names.as_ptr(),
        };
        let infos = get_patterns_info(pattern_names, status_ptr);
        helpers::status_to_result_void(status)?;

        let mut r_infos = Vec::with_capacity(infos.size as usize);
        for i in 0..infos.size as usize {
            let c_info = *(infos.infos.add(i));
            let r_info = PatternInfo {
                name: string_from_ptr(c_info.name),
                category: string_from_ptr(c_info.category),
                icon: string_from_ptr(c_info.icon),
                description: string_from_ptr(c_info.description),
                summary: string_from_ptr(c_info.summary),
                order: string_from_ptr(c_info.order),
                selected: c_info.selected.into(),
            };
            r_infos.push(r_info);
        }
        zypp_agama_sys::free_pattern_infos(&infos);
        Ok(r_infos)
    }
}

pub fn run_solver() -> ZyppResult<bool> {
    unsafe {
        let mut status: Status = Status::default();
        let status_ptr = &mut status as *mut _;
        let r_res = zypp_agama_sys::run_solver(status_ptr);
        let result = helpers::status_to_result_void(status);
        result.and(Ok(r_res))
    }
}

// high level method to load source
pub fn load_source<F>(progress: F) -> ZyppResult<()>
where
    F: Fn(i64, String) -> bool,
{
    let repos = list_repositories()?;
    let enabled_repos: Vec<&Repository> = repos.iter().filter(|r| r.enabled).collect();
    // TODO: this step logic for progress can be enclosed to own struct
    let mut percent: f64 = 0.0;
    let percent_step: f64 = 100.0 / (enabled_repos.len() as f64 * 3.0); // 3 substeps
    let abort_err = Err(ZyppError::new("Operation aborted"));
    let mut cont: bool;
    for i in enabled_repos {
        cont = progress(
            percent.floor() as i64,
            format!("Refreshing repository {}", &i.alias).to_string(),
        );
        if !cont {
            return abort_err;
        }
        refresh_repository(&i.alias, &callbacks::EmptyDownloadProgress)?;
        percent += percent_step;
        cont = progress(
            percent.floor() as i64,
            format!("Creating repository cache for {}", &i.alias).to_string(),
        );
        if !cont {
            return abort_err;
        }
        create_repo_cache(&i.alias, callbacks::empty_progress)?;
        percent += percent_step;
        cont = progress(
            percent.floor() as i64,
            format!("Loading repository cache for {}", &i.alias).to_string(),
        );
        if !cont {
            return abort_err;
        }
        load_repo_cache(&i.alias)?;
        percent += percent_step;
    }
    progress(100, "Loading repositories finished".to_string());
    Ok(())
}

pub fn import_gpg_key(file_path: &str) -> ZyppResult<()> {
    unsafe {
        let mut status: Status = Status::default();
        let status_ptr = &mut status as *mut _;
        let c_path = CString::new(file_path).expect("CString must not contain internal NUL");
        zypp_agama_sys::import_gpg_key(c_path.as_ptr(), status_ptr);
        status_to_result_void(status)
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
