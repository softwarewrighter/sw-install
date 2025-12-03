// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

//! Tests for the OutputHandler module.

use sw_install::create_output_handler;

#[test]
fn test_create_normal_output() {
    let output = create_output_handler(false, false);
    output.success("test");
}

#[test]
fn test_create_verbose_output() {
    let output = create_output_handler(true, false);
    output.step("test");
}

#[test]
fn test_create_dry_run_output() {
    let output = create_output_handler(false, true);
    output.success("test");
}

#[test]
fn test_create_verbose_dry_run_output() {
    let output = create_output_handler(true, true);
    output.step("test");
}
