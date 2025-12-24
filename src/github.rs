use serde::Deserialize;

#[derive(Deserialize)]
pub struct GitHubUser {
    pub login: String,
}

#[derive(Deserialize, Clone)]
pub struct RepoOwner {
    pub login: String,
}

#[derive(Deserialize, Clone)]
pub struct Repo {
    pub name: String,

    #[allow(dead_code)]
    pub html_url: String,

    pub owner: RepoOwner,
}

use reqwest::Client;
use std::env;

pub async fn fetch_starred_owned_repos() -> Result<Vec<Repo>, String> {
    let token = env::var("GITHUB_TOKEN").map_err(|_| "GITHUB_TOKEN not set".to_string())?;

    let client = Client::new();

    // 1. Fetch authenticated user
    let user: GitHubUser = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "raunak-tui")
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())?;

    // 2. Fetch starred repos
    let repos: Vec<Repo> = client
        .get("https://api.github.com/user/starred?visibility=public")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "raunak-tui")
        .send().await
        .map_err(|e| e.to_string())?
        .json().await
        .map_err(|e| e.to_string())?;

    // 3. Filter repos owned by user
    let owned = repos
        .into_iter()
        .filter(|repo| repo.owner.login == user.login)
        .collect();

    Ok(owned)
}
