// Code generated automatically

// This file must not be edited by hand

use crate::command_executor::{CommandExecutor, CommandOption};

/// Does not touch the index file or the working tree at all (but resets the head to <commit>, just like all modes do).
/// This leaves all your changed files 'Changes to be committed', as git status would put it.
/// --soft
pub fn soft() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--soft"))
}

/// Resets the index but not the working tree (i.e., the changed files are preserved but not marked for commit) and reports what has not been updated.
/// This is the default action.
/// If -N is specified, removed paths are marked as intent-to-add (see git-add(1)).
/// --mixed
pub fn mixed() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--mixed"))
}

/// Resets the index and working tree. Any changes to tracked files in the working tree since <commit> are discarded.
/// --hard
pub fn hard() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--hard"))
}

/// Resets the index and updates the files in the working tree that are different between <commit> and HEAD, but keeps those which are different between the index and working tree (i.e. which have changes which have not been added).
/// If a file that is different between <commit> and the index has unstaged changes, reset is aborted.
///  In other words, --merge does something like a git read-tree -u -m <commit>, but carries forward unmerged index entries.
/// --merge
pub fn merge() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--merge"))
}

/// Resets index entries and updates files in the working tree that are different between <commit> and HEAD. If a file that is different between <commit> and HEAD has local changes, reset is aborted.
/// --keep
pub fn keep() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--keep"))
}

/// Be quiet, only report errors.
/// -q, --quiet
pub fn quiet() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--quiet"))
}