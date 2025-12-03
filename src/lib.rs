// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

pub mod config;
pub mod error;
pub mod operations;
pub mod output;
pub mod validator;

pub use config::InstallConfig;
pub use error::{InstallError, Result};
pub use operations::{Installer, Lister, Setup, SortOrder, Uninstaller};
pub use output::{OutputHandler, create_output_handler};
pub use validator::Validator;
