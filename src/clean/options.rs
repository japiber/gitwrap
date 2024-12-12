// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// Normally, when no <pathspec> is specified, git clean will not recurse into untracked directories to avoid removing too much. Specify -d to have it recurse into such directories as well. If a <pathspec> is specified, -d is irrelevant; all untracked files matching the specified paths (with exceptions for nested git directories mentioned under --force) will be removed
/// -d
pub fn recurse_directories() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("-d");
    })
}

/// If the Git configuration variable clean.requireForce is not set to false, git clean will refuse to delete files or directories unless given -f. Git will refuse to modify untracked nested git repositories (directories with a .git subdirectory) unless a second -f is given
/// -f, --force
pub fn force() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--force");
    })
}

/// Show what would be done and clean files interactively. See “Interactive mode” for details. Configuration variable clean.requireForce is ignored, as this mode gives its own safety protection by going interactive
/// -i, --interactive
pub fn interactive() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--interactive");
    })
}

/// Don’t actually remove anything, just show what would be done. Configuration variable clean.requireForce is ignored, as nothing will be deleted anyway
/// -n, --dry-run
pub fn dry_run() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--dry-run");
    })
}

/// Be quiet, only report errors, but not the files that are successfully removed
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--quiet");
    })
}

/// Use the given exclude pattern in addition to the standard ignore rules (see gitignore[5])
/// -e=<pattern>, --exclude=<pattern>
pub fn exclude(pattern_arg: &str) -> FnOptionArg {
    let l_pattern_arg = format!("--exclude={}", pattern_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_pattern_arg.as_str());
    })
}

/// Don’t use the standard ignore rules (see gitignore[5]), but still use the ignore rules given with -e options from the command line. This allows removing all untracked files, including build products. This can be used (possibly in conjunction with git restore or git reset) to create a pristine working directory to test a clean build
/// -x
pub fn no_gitignore() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("-x");
    })
}

/// Remove only files ignored by Git. This may be useful to rebuild everything from scratch, but keep manually created files
/// -X
pub fn gitignore() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("-X");
    })
}

/// If any optional <pathspec>... arguments are given, only those paths that match the pathspec are affected
/// -- <pathspec>
pub fn pathspec(pathspec_arg: &str) -> FnOptionArg {
    let l_pathspec_arg = String::from(pathspec_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--");
        cmd.arg(l_pathspec_arg.as_str());
    })
}