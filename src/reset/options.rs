// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const SOFT: &str = "--soft";
pub const MIXED: &str = "--mixed";
pub const HARD: &str = "--hard";
pub const MERGE: &str = "--merge";
pub const KEEP: &str = "--keep";
pub const QUIET: &str = "--quiet";
pub const HYPHEN_HYPHEN: &str = "--";

/// Does not touch the index file or the working tree at all (but resets the head to <commit>, just like all modes do).
/// This leaves all your changed files 'Changes to be committed', as git status would put it.
/// --soft
pub fn soft() -> FnOptionArg {
    option_arg::simple(SOFT)
}

/// Resets the index but not the working tree (i.e., the changed files are preserved but not marked for commit) and reports what has not been updated.
/// This is the default action.
/// If -N is specified, removed paths are marked as intent-to-add (see git-add(1)).
/// --mixed
pub fn mixed() -> FnOptionArg {
    option_arg::simple(MIXED)
}

/// Resets the index and working tree.
/// Any changes to tracked files in the working tree since <commit> are discarded.
/// --hard
pub fn hard() -> FnOptionArg {
    option_arg::simple(HARD)
}

/// Resets the index and updates the files in the working tree that are different between <commit> and HEAD,
/// but keeps those which are different between the index and working tree (i.e.
/// which have changes which have not been added).
/// If a file that is different between <commit> and the index has unstaged changes, reset is aborted.
///  In other words, --merge does something like a git read-tree -u -m <commit>, but carries forward unmerged index entries.
/// --merge
pub fn merge() -> FnOptionArg {
    option_arg::simple(MERGE)
}

/// Resets index entries and updates files in the working tree that are different between <commit> and HEAD.
/// If a file that is different between <commit> and HEAD has local changes, reset is aborted.
/// --keep
pub fn keep() -> FnOptionArg {
    option_arg::simple(KEEP)
}

/// Be quiet, only report errors.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    option_arg::simple(QUIET)
}

/// Do not interpret any more arguments as options
/// --
pub fn hyphen_hyphen() -> FnOptionArg {
    option_arg::simple(HYPHEN_HYPHEN)
}

/// Limits the paths affected by the operation.
/// <pathspec>
pub fn pathspec(pathspec: &str) -> FnOptionArg {
    option_arg::value_parameter(pathspec)
}
