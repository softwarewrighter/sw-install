// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Operations module containing installation, uninstallation, listing, and setup functionality.

mod installer;
mod lister;
mod setup;
mod uninstaller;

pub use installer::Installer;
pub use lister::{Lister, SortOrder};
pub use setup::Setup;
pub use uninstaller::Uninstaller;
