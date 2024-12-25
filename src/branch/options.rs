// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const ABBREV: &str = "--abbrev";
pub const ALL: &str = "--all";
pub const COLOR: &str = "--color";
pub const COLUMN: &str = "--column";
pub const CONTAINS: &str = "--contains";
pub const NO_CONTAINS: &str = "--no-contains";
pub const CREATE_REFLOG: &str = "--create-reflog";
pub const DELETE: &str = "--delete";
pub const DELETE_FORCE: &str = "-D";
pub const EDIT_DESCRIPTION: &str = "--edit-description";
pub const FORCE: &str = "--force";
pub const FORMAT: &str = "--format";
pub const IGNORE_CASE: &str = "--ignore-case";
pub const LIST: &str = "--list";
pub const POINTS_AT: &str = "--points-at";
pub const MERGED: &str = "--merged";
pub const NO_MERGED: &str = "--no-merged";
pub const MOVE_BRANCH: &str = "--move";
pub const MOVE_FORCE: &str = "-M";
pub const QUIET: &str = "--quiet";
pub const REMOTES: &str = "--remotes";
pub const SHOW_CURRENT: &str = "--show-current";
pub const UNSET_UPSTREAM: &str = "--unset-upstream";
pub const SET_UPSTREAM_TO: &str = "--set-upstream-to";
pub const SORT: &str = "--sort";
pub const TRACK: &str = "--track";
pub const VERBOSE: &str = "--verbose";

/// use <n> digits to display SHA-1s
/// --abbrev[=<n>]
pub fn abbrev(n_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(ABBREV, n_arg)
}

/// All list both remote-tracking and local branches
/// -a, --all
pub fn all() -> FnOptionArg {
    option_arg::simple(ALL)
}

/// specify a valid branch name
/// <branch_name>
pub fn branch_name(branch_name: &str) -> FnOptionArg {
    option_arg::value_parameter(branch_name)
}

/// use colored output
/// --color[=<when>]
pub fn color(when_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(COLOR, when_arg)
}

/// list branches in columns
/// --column[=<style>]
pub fn column(style_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(COLUMN, style_arg)
}

/// print only branches that contain the commit
/// --contains <commit>
pub fn contains(commit_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(CONTAINS, commit_arg)
}

/// print only branches that don't contain the commit
/// --no-contains <commit>
pub fn no_contains(commit_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(NO_CONTAINS, commit_arg)
}

/// create the branch's reflog
/// -l, --create-reflog
pub fn create_reflog() -> FnOptionArg {
    option_arg::simple(CREATE_REFLOG)
}

/// delete fully merged branch
/// -d, --delete
pub fn delete() -> FnOptionArg {
    option_arg::simple(DELETE)
}

/// delete branch (even if not merged)
/// -D
pub fn delete_force() -> FnOptionArg {
    option_arg::simple(DELETE_FORCE)
}

/// edit the description for the branch
/// --edit-description
pub fn edit_description() -> FnOptionArg {
    option_arg::simple(EDIT_DESCRIPTION)
}

/// force creation, move/rename, deletion
/// -f, --force
pub fn force() -> FnOptionArg {
    option_arg::simple(FORCE)
}

/// format to use for the output
/// --format <format>
pub fn format(format_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(FORMAT, format_arg)
}

/// sorting and filtering are case insensitive
/// -i, --ignore-case
pub fn ignore_case() -> FnOptionArg {
    option_arg::simple(IGNORE_CASE)
}

/// list branch names
/// --list
pub fn list() -> FnOptionArg {
    option_arg::simple(LIST)
}

/// print only branches of the object
/// --points-at <object>
pub fn points_at(object_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(POINTS_AT, object_arg)
}

/// print only branches that are merged
/// --merged <commit>
pub fn merged(commit_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(MERGED, commit_arg)
}

/// print only branches that are not merged
/// --no-merged <commit>
pub fn no_merged(commit_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(NO_MERGED, commit_arg)
}

/// move/rename a branch and its reflog
/// -m, --move
pub fn move_branch() -> FnOptionArg {
    option_arg::simple(MOVE_BRANCH)
}

/// move/rename a branch, even if target exists
/// -M
pub fn move_force() -> FnOptionArg {
    option_arg::simple(MOVE_FORCE)
}

/// suppress informational messages
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    option_arg::simple(QUIET)
}

/// act on remote-tracking branches
/// -r, --remotes
pub fn remotes() -> FnOptionArg {
    option_arg::simple(REMOTES)
}

/// print the name of the current branch.
/// In detached HEAD state, nothing is printed
/// --show-current
pub fn show_current() -> FnOptionArg {
    option_arg::simple(SHOW_CURRENT)
}

/// SetUpstream change upstream info
/// --unset-upstream
pub fn unset_upstream(branchname_arg: &str) -> FnOptionArg {
    option_arg::with_optional_parameter(UNSET_UPSTREAM, branchname_arg)
}

/// change the upstream info to upstream
/// -u, --set-upstream-to <upstream>
pub fn set_upstream_to(upstream_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(SET_UPSTREAM_TO, upstream_arg)
}

/// field name to sort on
/// --sort <key>
pub fn sort(key_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(SORT, key_arg)
}

/// set up tracking mode (see git-pull(1)
/// -t, --track
pub fn track() -> FnOptionArg {
    option_arg::simple(TRACK)
}

/// show hash and subject, give twice for upstream branch
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    option_arg::simple(VERBOSE)
}
