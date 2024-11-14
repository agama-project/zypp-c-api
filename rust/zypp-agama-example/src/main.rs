fn main() {
    println!("List of repositories:");
    zypp_agama::init_target("/");
    let repos = zypp_agama::list_repositories();
    for repo in repos {
        println!("- Repo {} with url {}", repo.user_name, repo.url);
    }
}
