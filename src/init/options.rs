// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const QUIET: &str = "--quiet";
pub const BARE: &str = "--bare";
pub const TEMPLATE: &str = "--template";
pub const SEPARATE_GIT_DIR: &str = "--separate-git-dir";
pub const SHARED: &str = "--shared";

/// Only print error and warning messages; all other output will be suppressed.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    option_arg::simple(QUIET)
}

/// Create a bare repository.
/// If GIT_DIR environment is not set, it is set to the current working directory.
/// --bare
pub fn bare() -> FnOptionArg {
    option_arg::simple(BARE)
}

/// Specify the directory from which templates will be used.
/// (See the 'TEMPLATE DIRECTORY' section below.)
/// --template=<template_directory>
pub fn template(template_directory_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(TEMPLATE, template_directory_arg)
}

/// Instead of initializing the repository as a directory to either $GIT_DIR or ./.git/, create a text file there containing the path to the actual repository.
/// This file acts as filesystem-agnostic Git symbolic link to the repository.
/// --separate-git-dir=<git-dir>
pub fn separate_git_dir(git_dir_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(SEPARATE_GIT_DIR, git_dir_arg)
}

/// Specify that the Git repository is to be shared amongst several users.
/// This allows users belonging to the same group to push into that repository.
/// When specified, the config variable 'core.sharedRepository' is set so that files and directories under $GIT_DIR are created with the requested permissions.
/// When not specified, Git will use permissions reported by umask(2).
/// --shared[=(false|true|umask|group|all|world|everybody|0xxx)]
pub fn shared(value: &str) -> FnOptionArg {
    option_arg::equal_optional(SHARED, value)
}
