use std::{ffi::{CStr, CString}, ptr};

pub struct Repository {
    pub url: String,
    pub alias: String,
    pub user_name: String, 
}

// TODO: use result
pub fn init_target(root: &str) {
    unsafe {
        let c_root = CString::new(root).unwrap();
        zypp_agama_sys::init_target(c_root.as_ptr(), None, ptr::null_mut());
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
        zypp_agama_sys::free_repository_list(repos_rawp as *mut _ as *mut zypp_agama_sys::RepositoryList);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        init_target("/");
        let result = list_repositories();
        assert_eq!(result.len(), 24); // FIXME: just my quick validation
    }
}
