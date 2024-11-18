fn main() {
    println!("List of repositories:");
    zypp_agama::init_target("/", |text, step, total| {
        println!("Initializing target: {}/{} - {}", step, total, text)
    });
    let repos = zypp_agama::list_repositories();
    for repo in repos {
        println!("- Repo {} with url {}", repo.user_name, repo.url);
    }
}
