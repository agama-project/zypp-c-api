/* automatically generated by rust-bindgen 0.70.1 */

pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProgressData {
    pub value: ::std::os::raw::c_longlong,
    pub name: *const ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ProgressData"][::std::mem::size_of::<ProgressData>() - 16usize];
    ["Alignment of ProgressData"][::std::mem::align_of::<ProgressData>() - 8usize];
    ["Offset of field: ProgressData::value"][::std::mem::offset_of!(ProgressData, value) - 0usize];
    ["Offset of field: ProgressData::name"][::std::mem::offset_of!(ProgressData, name) - 8usize];
};
#[doc = " @return true to continue, false to abort. Can be ignored"]
pub type ZyppProgressCallback = ::std::option::Option<
    unsafe extern "C" fn(zypp_data: ProgressData, user_data: *mut ::std::os::raw::c_void) -> bool,
>;
pub const PROBLEM_RESPONSE_PROBLEM_RETRY: PROBLEM_RESPONSE = 0;
pub const PROBLEM_RESPONSE_PROBLEM_ABORT: PROBLEM_RESPONSE = 1;
pub const PROBLEM_RESPONSE_PROBLEM_IGNORE: PROBLEM_RESPONSE = 2;
pub type PROBLEM_RESPONSE = ::std::os::raw::c_uint;
pub type ZyppDownloadStartCallback = ::std::option::Option<
    unsafe extern "C" fn(
        url: *const ::std::os::raw::c_char,
        localfile: *const ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
pub type ZyppDownloadProgressCallback = ::std::option::Option<
    unsafe extern "C" fn(
        value: ::std::os::raw::c_int,
        url: *const ::std::os::raw::c_char,
        bps_avg: f64,
        bps_current: f64,
        user_data: *mut ::std::os::raw::c_void,
    ) -> bool,
>;
pub type ZyppDownloadProblemCallback = ::std::option::Option<
    unsafe extern "C" fn(
        url: *const ::std::os::raw::c_char,
        error: ::std::os::raw::c_int,
        description: *const ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    ) -> PROBLEM_RESPONSE,
>;
pub type ZyppDownloadFinishCallback = ::std::option::Option<
    unsafe extern "C" fn(
        url: *const ::std::os::raw::c_char,
        error: ::std::os::raw::c_int,
        reason: *const ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DownloadProgressCallbacks {
    pub start: ZyppDownloadStartCallback,
    pub start_data: *mut ::std::os::raw::c_void,
    pub progress: ZyppDownloadProgressCallback,
    pub progress_data: *mut ::std::os::raw::c_void,
    pub problem: ZyppDownloadProblemCallback,
    pub problem_data: *mut ::std::os::raw::c_void,
    pub finish: ZyppDownloadFinishCallback,
    pub finish_data: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DownloadProgressCallbacks"]
        [::std::mem::size_of::<DownloadProgressCallbacks>() - 64usize];
    ["Alignment of DownloadProgressCallbacks"]
        [::std::mem::align_of::<DownloadProgressCallbacks>() - 8usize];
    ["Offset of field: DownloadProgressCallbacks::start"]
        [::std::mem::offset_of!(DownloadProgressCallbacks, start) - 0usize];
    ["Offset of field: DownloadProgressCallbacks::start_data"]
        [::std::mem::offset_of!(DownloadProgressCallbacks, start_data) - 8usize];
    ["Offset of field: DownloadProgressCallbacks::progress"]
        [::std::mem::offset_of!(DownloadProgressCallbacks, progress) - 16usize];
    ["Offset of field: DownloadProgressCallbacks::progress_data"]
        [::std::mem::offset_of!(DownloadProgressCallbacks, progress_data) - 24usize];
    ["Offset of field: DownloadProgressCallbacks::problem"]
        [::std::mem::offset_of!(DownloadProgressCallbacks, problem) - 32usize];
    ["Offset of field: DownloadProgressCallbacks::problem_data"]
        [::std::mem::offset_of!(DownloadProgressCallbacks, problem_data) - 40usize];
    ["Offset of field: DownloadProgressCallbacks::finish"]
        [::std::mem::offset_of!(DownloadProgressCallbacks, finish) - 48usize];
    ["Offset of field: DownloadProgressCallbacks::finish_data"]
        [::std::mem::offset_of!(DownloadProgressCallbacks, finish_data) - 56usize];
};
#[doc = " status struct to pass and obtain from calls that can fail.\n After usage free with \\ref free_status function.\n\n Most functions act as *constructors* for this, taking a pointer\n to it as an output parameter, disregarding the struct current contents\n and filling it in. Thus, if you reuse a `Status` without \\ref free_status\n in between, `error` will leak."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Status {
    pub state: Status_STATE,
    #[doc = "< owned"]
    pub error: *mut ::std::os::raw::c_char,
}
pub const Status_STATE_STATE_SUCCEED: Status_STATE = 0;
pub const Status_STATE_STATE_FAILED: Status_STATE = 1;
pub type Status_STATE = ::std::os::raw::c_uint;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Status"][::std::mem::size_of::<Status>() - 16usize];
    ["Alignment of Status"][::std::mem::align_of::<Status>() - 8usize];
    ["Offset of field: Status::state"][::std::mem::offset_of!(Status, state) - 0usize];
    ["Offset of field: Status::error"][::std::mem::offset_of!(Status, error) - 8usize];
};
#[doc = " Progress reporting callback used by methods that takes longer.\n @param text  text for user describing what is happening now\n @param stage current stage number starting with 0\n @param total count of stages. It should not change during single call of method.\n @param user_data is never touched by method and is used only to pass local data for callback\n @todo Do we want to support response for callback that allows early exit of execution?"]
pub type ProgressCallback = ::std::option::Option<
    unsafe extern "C" fn(
        text: *const ::std::os::raw::c_char,
        stage: ::std::os::raw::c_uint,
        total: ::std::os::raw::c_uint,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
pub const RESOLVABLE_KIND_RESOLVABLE_PRODUCT: RESOLVABLE_KIND = 0;
pub const RESOLVABLE_KIND_RESOLVABLE_PATCH: RESOLVABLE_KIND = 1;
pub const RESOLVABLE_KIND_RESOLVABLE_PACKAGE: RESOLVABLE_KIND = 2;
pub const RESOLVABLE_KIND_RESOLVABLE_SRCPACKAGE: RESOLVABLE_KIND = 3;
pub const RESOLVABLE_KIND_RESOLVABLE_PATTERN: RESOLVABLE_KIND = 4;
pub type RESOLVABLE_KIND = ::std::os::raw::c_uint;
pub const RESOLVABLE_SELECTED_NOT_SELECTED: RESOLVABLE_SELECTED = 0;
pub const RESOLVABLE_SELECTED_USER_SELECTED: RESOLVABLE_SELECTED = 1;
pub const RESOLVABLE_SELECTED_INSTALLATION_SELECTED: RESOLVABLE_SELECTED = 2;
pub const RESOLVABLE_SELECTED_SOLVER_SELECTED: RESOLVABLE_SELECTED = 3;
pub type RESOLVABLE_SELECTED = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PatternNames {
    pub names: *const *const ::std::os::raw::c_char,
    pub size: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PatternNames"][::std::mem::size_of::<PatternNames>() - 16usize];
    ["Alignment of PatternNames"][::std::mem::align_of::<PatternNames>() - 8usize];
    ["Offset of field: PatternNames::names"][::std::mem::offset_of!(PatternNames, names) - 0usize];
    ["Offset of field: PatternNames::size"][::std::mem::offset_of!(PatternNames, size) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PatternInfo {
    #[doc = "< owned"]
    pub name: *mut ::std::os::raw::c_char,
    #[doc = "< owned"]
    pub category: *mut ::std::os::raw::c_char,
    #[doc = "< owned"]
    pub icon: *mut ::std::os::raw::c_char,
    #[doc = "< owned"]
    pub description: *mut ::std::os::raw::c_char,
    #[doc = "< owned"]
    pub summary: *mut ::std::os::raw::c_char,
    #[doc = "< owned"]
    pub order: *mut ::std::os::raw::c_char,
    pub selected: RESOLVABLE_SELECTED,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PatternInfo"][::std::mem::size_of::<PatternInfo>() - 56usize];
    ["Alignment of PatternInfo"][::std::mem::align_of::<PatternInfo>() - 8usize];
    ["Offset of field: PatternInfo::name"][::std::mem::offset_of!(PatternInfo, name) - 0usize];
    ["Offset of field: PatternInfo::category"]
        [::std::mem::offset_of!(PatternInfo, category) - 8usize];
    ["Offset of field: PatternInfo::icon"][::std::mem::offset_of!(PatternInfo, icon) - 16usize];
    ["Offset of field: PatternInfo::description"]
        [::std::mem::offset_of!(PatternInfo, description) - 24usize];
    ["Offset of field: PatternInfo::summary"]
        [::std::mem::offset_of!(PatternInfo, summary) - 32usize];
    ["Offset of field: PatternInfo::order"][::std::mem::offset_of!(PatternInfo, order) - 40usize];
    ["Offset of field: PatternInfo::selected"]
        [::std::mem::offset_of!(PatternInfo, selected) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PatternInfos {
    #[doc = "< owned, *size* items"]
    pub infos: *mut PatternInfo,
    pub size: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PatternInfos"][::std::mem::size_of::<PatternInfos>() - 16usize];
    ["Alignment of PatternInfos"][::std::mem::align_of::<PatternInfos>() - 8usize];
    ["Offset of field: PatternInfos::infos"][::std::mem::offset_of!(PatternInfos, infos) - 0usize];
    ["Offset of field: PatternInfos::size"][::std::mem::offset_of!(PatternInfos, size) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Repository {
    #[doc = "<"]
    pub enabled: bool,
    #[doc = "< owned"]
    pub url: *mut ::std::os::raw::c_char,
    #[doc = "< owned"]
    pub alias: *mut ::std::os::raw::c_char,
    #[doc = "< owned"]
    pub userName: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Repository"][::std::mem::size_of::<Repository>() - 32usize];
    ["Alignment of Repository"][::std::mem::align_of::<Repository>() - 8usize];
    ["Offset of field: Repository::enabled"][::std::mem::offset_of!(Repository, enabled) - 0usize];
    ["Offset of field: Repository::url"][::std::mem::offset_of!(Repository, url) - 8usize];
    ["Offset of field: Repository::alias"][::std::mem::offset_of!(Repository, alias) - 16usize];
    ["Offset of field: Repository::userName"]
        [::std::mem::offset_of!(Repository, userName) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RepositoryList {
    pub size: ::std::os::raw::c_uint,
    #[doc = "< owned, *size* items"]
    pub repos: *mut Repository,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RepositoryList"][::std::mem::size_of::<RepositoryList>() - 16usize];
    ["Alignment of RepositoryList"][::std::mem::align_of::<RepositoryList>() - 8usize];
    ["Offset of field: RepositoryList::size"]
        [::std::mem::offset_of!(RepositoryList, size) - 0usize];
    ["Offset of field: RepositoryList::repos"]
        [::std::mem::offset_of!(RepositoryList, repos) - 8usize];
};
extern "C" {
    pub fn set_zypp_progress_callback(
        progress: ZyppProgressCallback,
        user_data: *mut ::std::os::raw::c_void,
    );
    pub fn free_status(s: *mut Status);
    #[doc = " Initialize Zypp target (where to install packages to)\n @param root\n @param[out] status\n @param progress\n @param user_data"]
    pub fn init_target(
        root: *const ::std::os::raw::c_char,
        status: *mut Status,
        progress: ProgressCallback,
        user_data: *mut ::std::os::raw::c_void,
    );
    #[doc = " Marks resolvable for installation\n @param name resolvable name\n @param kind kind of resolvable\n @param[out] status (will overwrite existing contents)"]
    pub fn resolvable_select(
        name: *const ::std::os::raw::c_char,
        kind: RESOLVABLE_KIND,
        status: *mut Status,
    );
    #[doc = " Unselect resolvable for installation. It can still be installed as dependency.\n @param name resolvable name\n @param kind kind of resolvable\n @param[out] status (will overwrite existing contents)"]
    pub fn resolvable_unselect(
        name: *const ::std::os::raw::c_char,
        kind: RESOLVABLE_KIND,
        status: *mut Status,
    );
    pub fn get_patterns_info(names: PatternNames, status: *mut Status) -> PatternInfos;
    pub fn free_pattern_infos(infos: *const PatternInfos);
    #[doc = " Runs solver\n @param[out] status (will overwrite existing contents)\n @return true if solver pass and false if it found some dependency issues"]
    pub fn run_solver(status: *mut Status) -> bool;
    pub fn free_zypp();
    #[doc = " repository array in list.\n when no longer needed, use \\ref free_repository_list to release memory\n @param[out] status (will overwrite existing contents)"]
    pub fn list_repositories(status: *mut Status) -> RepositoryList;
    pub fn free_repository_list(repo_list: *mut RepositoryList);
    #[doc = " Adds repository to repo manager\n @param alias have to be unique\n @param url can contain repo variables\n @param[out] status (will overwrite existing contents)\n @param callback pointer to function with callback or NULL\n @param user_data"]
    pub fn add_repository(
        alias: *const ::std::os::raw::c_char,
        url: *const ::std::os::raw::c_char,
        status: *mut Status,
        callback: ZyppProgressCallback,
        user_data: *mut ::std::os::raw::c_void,
    );
    #[doc = " Removes repository from repo manager\n @param alias have to be unique\n @param[out] status (will overwrite existing contents)\n @param callback pointer to function with callback or NULL\n @param user_data"]
    pub fn remove_repository(
        alias: *const ::std::os::raw::c_char,
        status: *mut Status,
        callback: ZyppProgressCallback,
        user_data: *mut ::std::os::raw::c_void,
    );
    #[doc = "\n @param alias alias of repository to refresh\n @param[out] status (will overwrite existing contents)\n @param callbacks pointer to struct with callbacks or NULL if no progress is needed"]
    pub fn refresh_repository(
        alias: *const ::std::os::raw::c_char,
        status: *mut Status,
        callbacks: *mut DownloadProgressCallbacks,
    );
    pub fn build_repository_cache(
        alias: *const ::std::os::raw::c_char,
        status: *mut Status,
        callback: ZyppProgressCallback,
        user_data: *mut ::std::os::raw::c_void,
    );
    pub fn load_repository_cache(alias: *const ::std::os::raw::c_char, status: *mut Status);
}
