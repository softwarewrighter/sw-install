use std::process::Command;

fn main() {
    // Get the hostname (build host)
    let hostname = Command::new("hostname")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=BUILD_HOST={}", hostname);

    // Get the git commit SHA
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Get the current timestamp in ISO 8601 format
    let timestamp = Command::new("date")
        .args(["+%Y-%m-%dT%H:%M:%S%z"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Rerun if .git/HEAD changes (new commit)
    println!("cargo:rerun-if-changed=.git/HEAD");
}
