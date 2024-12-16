// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;
/// Does not touch the index file or the working tree at all (but resets the head to <commit>, just like all modes do).
/// This leaves all your changed files 'Changes to be committed', as git status would put it.
/// --soft
pub fn soft() -> FnOptionArg {
    optionarg::simple("--soft")
}

/// Resets the index but not the working tree (i.e., the changed files are preserved but not marked for commit) and reports what has not been updated.
/// This is the default action.
/// If -N is specified, removed paths are marked as intent-to-add (see git-add(1)).
/// --mixed
pub fn mixed() -> FnOptionArg {
    optionarg::simple("--mixed")
}

/// Resets the index and working tree. Any changes to tracked files in the working tree since <commit> are discarded.
/// --hard
pub fn hard() -> FnOptionArg {
    optionarg::simple("--hard")
}

/// Resets the index and updates the files in the working tree that are different between <commit> and HEAD, but keeps those which are different between the index and working tree (i.e. which have changes which have not been added).
/// If a file that is different between <commit> and the index has unstaged changes, reset is aborted.
///  In other words, --merge does something like a git read-tree -u -m <commit>, but carries forward unmerged index entries.
/// --merge
pub fn merge() -> FnOptionArg {
    optionarg::simple("--merge")
}

/// Resets index entries and updates files in the working tree that are different between <commit> and HEAD. If a file that is different between <commit> and HEAD has local changes, reset is aborted.
/// --keep
pub fn keep() -> FnOptionArg {
    optionarg::simple("--keep")
}

/// Be quiet, only report errors.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    optionarg::simple("--quiet")
}

/// Do not interpret any more arguments as options
/// --
pub fn hyphen_hyphen() -> FnOptionArg {
    optionarg::simple("--")
}

/// Limits the paths affected by the operation.
/// <pathspec>
pub fn pathspec(pathspec: &str) -> FnOptionArg {
    optionarg::value_parameter(pathspec)
}
