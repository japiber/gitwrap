// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;

pub const RECURSE_DIRECTORIES: &str = "-d";
pub const FORCE: &str = "--force";
pub const INTERACTIVE: &str = "--interactive";
pub const DRY_RUN: &str = "--dry-run";
pub const QUIET: &str = "--quiet";
pub const EXCLUDE: &str = "--exclude";
pub const NO_GITIGNORE: &str = "-x";
pub const GITIGNORE: &str = "-X";
pub const HYPHEN_HYPHEN: &str = "--";

/// Normally, when no <pathspec> is specified, git clean will not recurse into untracked directories to avoid removing too much.
/// Specify -d to have it recurse into such directories as well.
/// If a <pathspec> is specified, -d is irrelevant; all untracked files matching the specified paths (with exceptions for nested git directories mentioned under --force) will be removed
/// -d
pub fn recurse_directories() -> FnOptionArg {
    optionarg::simple(RECURSE_DIRECTORIES)
}

/// If the Git configuration variable clean.requireForce is not set to false, git clean will refuse to delete files or directories unless given -f.
/// Git will refuse to modify untracked nested git repositories (directories with a .git subdirectory) unless a second -f is given
/// -f, --force
pub fn force() -> FnOptionArg {
    optionarg::simple(FORCE)
}

/// Show what would be done and clean files interactively.
/// See “Interactive mode” for details.
/// Configuration variable clean.requireForce is ignored, as this mode gives its own safety protection by going interactive
/// -i, --interactive
pub fn interactive() -> FnOptionArg {
    optionarg::simple(INTERACTIVE)
}

/// Don’t actually remove anything, just show what would be done.
/// Configuration variable clean.requireForce is ignored, as nothing will be deleted anyway
/// -n, --dry-run
pub fn dry_run() -> FnOptionArg {
    optionarg::simple(DRY_RUN)
}

/// Be quiet, only report errors, but not the files that are successfully removed
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    optionarg::simple(QUIET)
}

/// Use the given exclude pattern in addition to the standard ignore rules.
/// -e=<pattern>, --exclude=<pattern>
pub fn exclude(pattern_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(EXCLUDE, pattern_arg)
}

/// Don’t use the standard ignore rules, but still use the ignore rules given with -e options from the command line.
/// This allows removing all untracked files, including build products.
/// This can be used (possibly in conjunction with git restore or git reset) to create a pristine working directory to test a clean build
/// -x
pub fn no_gitignore() -> FnOptionArg {
    optionarg::simple(NO_GITIGNORE)
}

/// Remove only files ignored by Git.
/// This may be useful to rebuild everything from scratch, but keep manually created files
/// -X
pub fn gitignore() -> FnOptionArg {
    optionarg::simple(GITIGNORE)
}

/// Should appear just before any pathspec option
/// --
pub fn hyphen_hyphen() -> FnOptionArg {
    optionarg::simple(HYPHEN_HYPHEN)
}

/// If any optional <pathspec>... arguments are given,
/// only those paths that match the pathspec are affected
/// <pathspec>
pub fn pathspec(pathspec: &str) -> FnOptionArg {
    optionarg::value_parameter(pathspec)
}
