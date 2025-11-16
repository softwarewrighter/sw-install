// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

pub mod config;
pub mod error;
pub mod installer;
pub mod lister;
pub mod output;
pub mod setup;
pub mod uninstaller;
pub mod validator;

pub use config::InstallConfig;
pub use error::{InstallError, Result};
pub use installer::Installer;
pub use lister::Lister;
pub use output::{create_output_handler, OutputHandler};
pub use setup::Setup;
pub use uninstaller::Uninstaller;
pub use validator::Validator;
