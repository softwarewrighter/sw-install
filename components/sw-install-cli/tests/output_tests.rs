// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Tests for the NormalOutput module.

use sw_install::NormalOutput;

#[test]
fn test_create_normal_output() {
    let output = NormalOutput::new(false, false);
    output.success("test");
}

#[test]
fn test_create_verbose_output() {
    let output = NormalOutput::new(true, false);
    output.info("test");
}

#[test]
fn test_create_dry_run_output() {
    let output = NormalOutput::new(false, true);
    output.success("test");
}

#[test]
fn test_create_verbose_dry_run_output() {
    let output = NormalOutput::new(true, true);
    output.info("test");
}
