use std::env;
use zypp_agama::ResolvableKind;

/*
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

*/

fn main() -> Result<(), zypp_agama::ZyppError> {
    println!("Usage: main [ROOT]");
    println!("  ROOT defaults to /");

    let args: Vec<String> = env::args().collect();
    let default_root = "/".to_owned();
    let root = args.get(1).unwrap_or(&default_root);

    let zypp = zypp_agama::Zypp::init_target(root, |text, step, total| {
        println!("Initializing target: {}/{} - {}", step, total, text)
    })?;

    println!("List of existing repositories:");
    let repos = zypp.list_repositories()?;
    for repo in repos {
        println!("- Repo {} with url {}", repo.user_name, repo.url);
    }

    zypp.load_source(|percent, text| {
        println!("{}%: {}", percent, text);
        true
    })?;
    // intentionally create conflict
    zypp.select_resolvable(
        "ftp",
        ResolvableKind::Package,
        zypp_agama::ResolvableSelected::User,
    )?;
    zypp.select_resolvable(
        "tnftp",
        ResolvableKind::Package,
        zypp_agama::ResolvableSelected::User,
    )?;
    let res = zypp.run_solver()?;
    println!("Conflict case. Solver returns {}", res);

    zypp.unselect_resolvable(
        "ftp",
        ResolvableKind::Package,
        zypp_agama::ResolvableSelected::User,
    )?;
    let res = zypp.run_solver()?;
    println!("Non conflicting case. Solver returns {}", res);

    let names = vec!["base", "minimal_base"];
    let res = zypp.patterns_info(names)?;
    println!("patterns info: {:#?}", res);

    Ok(())
}
