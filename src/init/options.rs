// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// Only print error and warning messages; all other output will be suppressed.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--quiet");
    })
}

/// Create a bare repository.
/// If GIT_DIR environment is not set, it is set to the current working directory.
/// --bare
pub fn bare() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--bare");
    })
}

/// Specify the directory from which templates will be used.
/// (See the 'TEMPLATE DIRECTORY' section below.)
/// --template=<template_directory>
pub fn template(template_directory_arg: &str) -> FnOptionArg {
    let l_template_directory_arg = format!("--template={}", template_directory_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_template_directory_arg.as_str());
    })
}

/// Instead of initializing the repository as a directory to either $GIT_DIR or ./.git/, create a text file there containing the path to the actual repository.
/// This file acts as filesystem-agnostic Git symbolic link to the repository.
/// --separate-git-dir=<git-dir>
pub fn separate_git_dir(git_dir_arg: &str) -> FnOptionArg {
    let l_git_dir_arg = format!("--separate-git-dir={}", git_dir_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_git_dir_arg.as_str());
    })
}

/// Specify that the Git repository is to be shared amongst several users.
/// This allows users belonging to the same group to push into that repository.
/// When specified, the config variable 'core.sharedRepository' is set so that files and directories under $GIT_DIR are created with the requested permissions.
/// When not specified, Git will use permissions reported by umask(2).
/// --shared[=(false|true|umask|group|all|world|everybody|0xxx)]
pub fn shared(value: &str) -> FnOptionArg {
    let l_value = if value.is_empty() {
        String::from("--shared")
    } else {
        format!("--shared={}", value)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_value.as_str());
    })
}