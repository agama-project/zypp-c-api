fn main() {
    println!("Example failure of initialize:");
    let result = zypp_agama::init_target("/bla", |_,_,_| {
    });
    if let Err(error) = result {
        println!("Failed to initialize target: {}", error);
        println!("Lets try some real work");
        println!("");
    };
    println!("List of repositories:");
    let result = zypp_agama::init_target("/", |text, step, total| {
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
