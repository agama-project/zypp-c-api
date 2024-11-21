use std::env;
use zypp_agama::{refresh_repository, DownloadProgress};

struct ExampleProgress{

}

impl DownloadProgress for ExampleProgress {
    fn progress(&self, value: i32, url: &str, _bps_avg: f64, _bps_current: f64) -> bool {
        println!("Donwloading {} - {}%", url, value);
        true
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
    let progress = ExampleProgress{};
    for repo in repos {
        println!("- Repo {} with url {}", repo.user_name, repo.url);
        println!("Refreshing...");
        let result = refresh_repository(&repo.alias, &progress);
        if let Err(error) = result {
            println!("Failed to refresh repo {}: {}", repo.user_name, error);
            return;
        };
    }
}
