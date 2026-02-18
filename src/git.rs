use std::path::Path;
use std::process::Command;

pub struct GitStatus {
    pub branch: String,
    pub modified: usize,
    pub untracked: usize,
    pub staged: usize,
}

impl GitStatus {
    pub fn status_text(&self) -> String {
        if self.is_clean() {
            return "clean".to_string();
        }
        let mut parts = Vec::new();
        if self.modified > 0 {
            parts.push(format!("{} modified", self.modified));
        }
        if self.staged > 0 {
            parts.push(format!("{} staged", self.staged));
        }
        if self.untracked > 0 {
            parts.push(format!("{} untracked", self.untracked));
        }
        parts.join(", ")
    }

    pub fn is_clean(&self) -> bool {
        self.modified == 0 && self.untracked == 0 && self.staged == 0
    }

    pub fn is_available(&self) -> bool {
        self.is_clean() && self.branch == "develop"
    }
}

pub fn get_status(path: &str) -> Option<GitStatus> {
    if !Path::new(path).exists() {
        return None;
    }

    let branch = Command::new("git")
        .args(["-C", path, "branch", "--show-current"])
        .output()
        .ok()?;
    let branch = String::from_utf8_lossy(&branch.stdout).trim().to_string();

    let status = Command::new("git")
        .args(["-C", path, "status", "--porcelain"])
        .output()
        .ok()?;
    let status_text = String::from_utf8_lossy(&status.stdout);

    let mut modified = 0;
    let mut untracked = 0;
    let mut staged = 0;

    for line in status_text.lines() {
        if line.len() < 2 {
            continue;
        }
        let index = line.as_bytes()[0];
        let worktree = line.as_bytes()[1];

        if line.starts_with("??") {
            untracked += 1;
        } else {
            if index != b' ' && index != b'?' {
                staged += 1;
            }
            if worktree != b' ' && worktree != b'?' {
                modified += 1;
            }
        }
    }

    Some(GitStatus {
        branch,
        modified,
        untracked,
        staged,
    })
}

pub fn checkout_develop(path: &str) -> Result<(), String> {
    let fetch = Command::new("git")
        .args(["-C", path, "fetch", "origin"])
        .output()
        .map_err(|e| format!("git fetch failed: {}", e))?;
    if !fetch.status.success() {
        return Err(format!(
            "git fetch failed: {}",
            String::from_utf8_lossy(&fetch.stderr)
        ));
    }

    let checkout = Command::new("git")
        .args(["-C", path, "checkout", "develop"])
        .output()
        .map_err(|e| format!("git checkout failed: {}", e))?;
    if !checkout.status.success() {
        return Err(format!(
            "git checkout failed: {}",
            String::from_utf8_lossy(&checkout.stderr)
        ));
    }

    let pull = Command::new("git")
        .args(["-C", path, "pull"])
        .output()
        .map_err(|e| format!("git pull failed: {}", e))?;
    if !pull.status.success() {
        return Err(format!(
            "git pull failed: {}",
            String::from_utf8_lossy(&pull.stderr)
        ));
    }

    Ok(())
}
