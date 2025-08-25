use git2::build::RepoBuilder;
use std::path::Path;

use std::process::Command;

pub fn git_checkout(repo_url: &str, target_path: &Path) -> Result<(), String> {
    println!("Cloning {} into {:?}...", repo_url, target_path);

    let repo = match RepoBuilder::new().clone(repo_url, target_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Failed to clone repository: {}", e)),
    };

    println!("Repository cloned. Initializing and updating submodules...");

    let mut options = git2::SubmoduleUpdateOptions::new();

    for mut submodule in repo.submodules().map_err(|e| format!("Failed to get submodules: {}", e))? {
        let submodule_path = submodule.path().to_path_buf();
        match submodule.update(true, Some(&mut options)) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Warning: Failed to update submodule {:?}: {}. Attempting manual update.", submodule_path, e);
                // Attempt a manual update
                let submodule_repo_path = target_path.join(&submodule_path);
                if submodule_repo_path.exists() {
                    println!("Attempting git reset --hard and git pull in {:?}...", submodule_repo_path);
                    let output = Command::new("git")
                        .arg("-C")
                        .arg(&submodule_repo_path)
                        .arg("reset")
                        .arg("--hard")
                        .arg("HEAD")
                        .output()
                        .expect("Failed to execute git reset");
                    eprintln!("git reset stdout: {}", String::from_utf8_lossy(&output.stdout));
                    eprintln!("git reset stderr: {}", String::from_utf8_lossy(&output.stderr));

                    let output = Command::new("git")
                        .arg("-C")
                        .arg(&submodule_repo_path)
                        .arg("pull")
                        .output()
                        .expect("Failed to execute git pull");
                    eprintln!("git pull stdout: {}", String::from_utf8_lossy(&output.stdout));
                    eprintln!("git pull stderr: {}", String::from_utf8_lossy(&output.stderr));
                } else {
                    eprintln!("Submodule directory {:?} does not exist for manual update.", submodule_repo_path);
                }
            }
        }
    }

    println!("Submodules updated.");

    Ok(())
}
