// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::fs;
use std::path::{Path, PathBuf};
use sw_install_core::{NormalOutput, Result};

pub fn find_shell_config(home: &Path) -> PathBuf {
    [".zshrc", ".bashrc", ".bash_profile", ".profile"]
        .iter()
        .map(|f| home.join(f))
        .find(|p| p.exists())
        .unwrap_or_else(|| home.join(".bashrc"))
}

#[rustfmt::skip]
pub fn write_path_config(cfg: &Path, dir: &Path, dry_run: bool, out: &NormalOutput) -> Result<PathBuf> {
    let path_line = format!("export PATH=\"{}:$PATH\"", dir.display());
    if dry_run {
        out.info(&format!("Would add to {}: {}", cfg.display(), path_line));
        return Ok(cfg.to_path_buf());
    }
    let content = fs::read_to_string(cfg).unwrap_or_default();
    if content.contains(&path_line) {
        out.info("PATH already configured in shell config");
        return Ok(cfg.to_path_buf());
    }
    let sep = if content.is_empty() || content.ends_with('\n') { "" } else { "\n" };
    fs::write(cfg, format!("{}{}\n# Added by sw-install\n{}\n", content, sep, path_line))?;
    Ok(cfg.to_path_buf())
}
