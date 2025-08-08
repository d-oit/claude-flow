use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn get_diff_files(diff_spec: &str) -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .args(&["diff", "--name-only", diff_spec])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Git diff command failed"));
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .collect();

    Ok(files)
}

pub fn get_staged_files() -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .args(&["diff", "--cached", "--name-only"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Git staged files command failed"));
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .collect();

    Ok(files)
}

pub fn get_current_commit_hash() -> Result<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get current commit hash"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}