use std::env;

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
    for repo in repos {
        println!("- Repo {} with url {}", repo.user_name, repo.url);
    }
}
