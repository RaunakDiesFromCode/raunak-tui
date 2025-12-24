use serde::Deserialize;
use reqwest::Client;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
}

pub async fn fetch_repos() -> Result<Vec<Repo>, String> {
    let client = Client::new();

    let mut repos: Vec<Repo> = client
        .get("https://api.github.com/users/RaunakDiesFromCode/repos")
        .header("User-Agent", "raunak-tui")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    // 🔀 Randomize order
    repos.shuffle(&mut thread_rng());

    // 🎯 Optional: limit how many you show
    // Change 6 to whatever feels right
    repos.truncate(6);

    Ok(repos)
}
