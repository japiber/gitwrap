use crate::command_executor::{CommandExecutor, CommandOption};

/// Only print error and warning messages; all other output will be suppressed.
/// -q, --quiet
pub fn quiet_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--quiet"))
}

/// Create a bare repository.
/// If GIT_DIR environment is not set, it is set to the current working directory.
/// --bare
pub fn bare_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--bare"))
}

/// Specify the directory from which templates will be used.
/// (See the 'TEMPLATE DIRECTORY' section below.)
/// --template=<template_directory>
pub fn template_option(template_directory_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--template={}", template_directory_arg)))
}

/// Instead of initializing the repository as a directory to either $GIT_DIR or ./.git/, create a text file there containing the path to the actual repository.
/// This file acts as filesystem-agnostic Git symbolic link to the repository.
/// --separate-git-dir=<git-dir>
pub fn separate_git_dir_option(git_dir_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--separate-git-dir={}", git_dir_arg)))
}

/// Specify that the Git repository is to be shared amongst several users.
/// This allows users belonging to the same group to push into that repository.
/// When specified, the config variable 'core.sharedRepository' is set so that files and directories under $GIT_DIR are created with the requested permissions.
/// When not specified, Git will use permissions reported by umask(2).
/// --shared[=(false|true|umask|group|all|world|everybody|0xxx)]
pub fn shared_option(value :&str) -> CommandOption {
    if value.len() == 0 {
        Box::new(|g: &mut CommandExecutor| g.add_option_string(format!("--shared")))
    } else {
        Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--shared={}", value)))
    }
}
