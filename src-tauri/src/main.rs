// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use octocrab::{self, Octocrab, models::{Author, Repository}, params::users::repos::Type};

#[derive(serde::Serialize, serde::Deserialize)]
struct RepositoryForDelete {
  name: String,
  description: Option<String>,
  checked: bool
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![search_repository, delete_repositories])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn search_repository(token: String) -> Result<Vec<Repository>, ()> {
  if let Ok(client) = Octocrab::builder().personal_token(token).build() {
    let user: Author = match client.get("/user", None::<&()>).await {
      Ok(user) => user,
      Err(_) => return Err(())
    };
    let userId = &user.login;
    let user = client.users(userId);
    let mut pages = match user.repos().r#type(Type::All).per_page(100).send().await {
      Ok(pages) => pages,
      Err(_) => return Err(())
    };
    let repos = pages.take_items();
    return Ok(repos);
  }

  return Err(());
}

#[tauri::command]
async fn delete_repositories(token: String, repositories: Vec<RepositoryForDelete>) -> Result<(),()> {
  if let Ok(client) = Octocrab::builder().personal_token(token).build() {
    for repo in repositories {
      let splitted: Vec<&str> = repo.name.split("/").collect();
      let owner = splitted[0];
      let name = splitted[1];
      client.repos(owner, name).delete().await;
    }
    return Ok(());
  }
  Err(())
}