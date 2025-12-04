// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Install and uninstall operations for sw-install.

mod install;
mod paths;
mod uninstall;

pub use install::Installer;
pub use uninstall::Uninstaller;
