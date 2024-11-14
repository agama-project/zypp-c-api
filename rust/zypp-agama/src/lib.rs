use std::{ffi::CString, ptr};

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

// TODO: use result
pub fn list_repositories() -> Vec<Repository> {
    let mut res = vec![];

    unsafe {
        let mut repos = zypp_agama_sys::list_repositories();
        // unwrap is ok as it will crash only on less then 32 archs,so safe for agama
        let size_usize: usize = repos.size.try_into().unwrap();
        for i in 0..size_usize {
            let c_repo = *(repos.repos.add(i));
            // TODO some error reporting when it is not utf-8 would be nice
            let r_repo = Repository {
                url: String::from_utf8_lossy(CString::from_raw(c_repo.url).as_bytes()).into_owned(),
                alias: String::from_utf8_lossy(CString::from_raw(c_repo.alias).as_bytes()).into_owned(),
                user_name: String::from_utf8_lossy(CString::from_raw(c_repo.userName).as_bytes()).into_owned(),
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
        assert_eq!(result.len(), 17); // TODO just my quick validation
    }
}
