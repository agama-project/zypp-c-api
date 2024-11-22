use std::{cell::RefCell, env};
use zypp_agama::{add_repository, refresh_repository, DownloadProgress};

struct ExampleProgress {
    // We need to use RefCell here because libzypp is limited to single thread
    // and trait for progress cannot use `&mut self` as it passed multiple callbacks
    // at once to libzypp and `&mut` is exclusive.
    progress_bar: RefCell<indicatif::ProgressBar>,
}

impl ExampleProgress {
    fn new() -> Self {
        let bar = indicatif::ProgressBar::hidden();
        Self {
            progress_bar: RefCell::new(bar)
        }
    }
}

impl DownloadProgress for ExampleProgress {
    fn start(&self, url: &str, localfile: &str) {
        self.progress_bar.replace(indicatif::ProgressBar::new(100));
        let bar = self.progress_bar.borrow_mut();
        bar.set_message(format!("Downloading {}", url));
        bar.println(format!("local path: {}", localfile));
    }

    fn progress(&self, value: i32, _url: &str, _bps_avg: f64, _bps_current: f64) -> bool {
        // unwrap is ok here as we know that libzypp send percentage in value..do not ask me why it uses int instead of unsigned
        self.progress_bar.borrow_mut().set_position(value.try_into().unwrap());
        true
    }

    // skip definition of problem as we are ok with default action.

    fn finish(&self, url: &str, error_id: i32, reason: &str) {
        let bar = self.progress_bar.borrow_mut();
        // well, by checking libzypp sources I know that 0 means no error :) C API do not remap yeat enum for it.
        if error_id == 0 {
            bar.println(format!("{} downloaded.", url));
        } else {
            bar.println(format!("{} failed to download due to: {}.", url, reason));
        }
        bar.finish_and_clear();
    }
}

fn main() {
    println!("Usage: main [ROOT]");
    println!("  ROOT defaults to /");

    let args: Vec<String> = env::args().collect();
    let default_root = "/".to_owned();
    let root = args.get(1).unwrap_or(&default_root);

    println!("List of repositories:");
    let result = zypp_agama::init_target(root, |text, step, total| {
        println!("Initializing target: {}/{} - {}", step, total, text)
    });
    if let Err(error) = result {
        println!("Failed to initialize target: {}", error);
        return;
    };
    let repos = zypp_agama::list_repositories();
    let progress = ExampleProgress::new();
    for repo in repos {
        println!("- Repo {} with url {}", repo.user_name, repo.url);
        println!("Refreshing...");
        let result = refresh_repository(&repo.alias, &progress);
        if let Err(error) = result {
            println!("Failed to refresh repo {}: {}", repo.user_name, error);
            return;
        };
        println!("Refresh done.")
    }

    println!("Adding new repo agama:");
    let result = add_repository("agama", "https://download.opensuse.org/repositories/systemsmanagement:/Agama:/Devel/openSUSE_Tumbleweed/", |value, text| { 
        println!("{}:{}%", text, value);
        true // no abort of operation
    });
    if let Err(error) = result {
        println!("Failed to add repo: {}", error);
        return;
    };
    println!("Refreshing...");
    let result = refresh_repository("agama", &progress);
    if let Err(error) = result {
        println!("Failed to refresh repo: {}", error);
        return;
    };
}
