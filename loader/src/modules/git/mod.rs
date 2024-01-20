pub mod repo {
    use git2;
    use serde_json;

    use crate::modules::loader::settings;

    pub async fn clone(url: &str) -> serde_json::Value {
        match git2::Repository::clone(
            url,
            format!(
                "{}repositories\\git.{}",
                settings::PATH,
                url.split("/").last().unwrap()
            ),
        ) {
            Ok(_) => serde_json::json!({
                "status": "success",
                "message": "Successfully cloned",
            }),
            Err(error) => serde_json::json!({
                "status": "error",
                "message": error.to_string()
            }),
        }
    }

    pub async fn update(path: &str) -> serde_json::Value {
        let repo = match git2::Repository::open(&path) {
            Ok(repo) => repo,
            Err(error) => {
                return serde_json::json!({
                    "status": "error",
                    "message": error.to_string()
                });
            }
        };

        let mut remote = match repo.find_remote("origin") {
            Ok(remote) => remote,
            Err(error) => {
                return serde_json::json!({
                    "status": "error",
                    "message": error.to_string()
                });
            }
        };

        let callbacks = git2::RemoteCallbacks::new();

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        if let Err(error) = remote.fetch::<String>(&[], Some(&mut fetch_options), None) {
            return serde_json::json!({
                "status": "error",
                "message": error.to_string()
            });
        }

        let head = match repo.head() {
            Ok(head) => head,
            Err(error) => {
                return serde_json::json!({
                    "status": "error",
                    "message": error.to_string()
                });
            }
        };

        let branch_name = head.shorthand().unwrap_or_default();

        let refname = format!("refs/remotes/origin/{}", branch_name);
        let remote_branch = match repo.find_reference(&refname) {
            Ok(reference) => reference.peel_to_commit(),
            Err(error) => {
                return serde_json::json!({
                    "status": "error",
                    "message": error.to_string()
                });
            }
        };

        if let Err(error) = repo.set_head(&format!("refs/heads/{}", branch_name)) {
            return serde_json::json!({
                "status": "error",
                "message": error.to_string()
            });
        }

        if let Err(error) = repo.reset(
            remote_branch.unwrap().as_object(),
            git2::ResetType::Hard,
            None,
        ) {
            return serde_json::json!({
                "status": "error",
                "message": error.to_string()
            });
        }

        if let Err(error) = repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force())) {
            return serde_json::json!({
                "status": "error",
                "message": error.to_string()
            });
        }

        serde_json::json!({
            "status": "success",
            "message": "Successfully updated",
        })
    }
}

pub mod utils {
    use git2;

    use super::repo;
    use crate::modules::{
        cheat::{self, parser},
        loader::settings,
    };

    pub async fn add(url: &str) -> serde_json::Value {
        repo::clone(url).await;

        let mut repos = settings::get("repositories").await;

        repos.as_array_mut().unwrap().push(serde_json::json!(url));

        settings::replace("repositories", repos).await;

        serde_json::json!({
            "status": "success",
            "message": "Repository successfully added",
        })
    }

    pub async fn remove(name: &str) -> serde_json::Value {
        let path = format!("{}repositories\\{}", settings::PATH, name);

        if let Some(repo) = git2::Repository::open(&path).ok() {
            if let Some(remote) = repo.find_remote("origin").ok() {
                let mut repos = settings::get("repositories").await;

                repos
                    .as_array_mut()
                    .unwrap()
                    .retain(|repo| repo.as_str().unwrap() != remote.url().unwrap());

                settings::replace("repositories", repos).await;
            }
        }

        match std::fs::remove_dir_all(path) {
            Ok(_) => serde_json::json!({
                "status": "success",
                "message": "Repository successfully removed",
            }),
            Err(error) => serde_json::json!({
                "status": "error",
                "message": error.to_string()
            }),
        }
    }

    pub async fn update_all() -> serde_json::Value {
        match std::fs::read_dir(format!("{}repositories", settings::PATH)) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                            && entry.file_name().to_str().unwrap().starts_with("git.")
                        {
                            repo::update(entry.path().to_str().unwrap()).await;
                        }
                    }
                }

                serde_json::json!({
                    "status": "success",
                    "message": "Successfully updated all repositories"
                })
            }
            Err(error) => {
                serde_json::json!({
                    "status": "error",
                    "message": error.to_string()
                })
            }
        }
    }
}
