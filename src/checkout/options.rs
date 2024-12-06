// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// Quiet, suppress feedback messages.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--quiet");
    })
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless --quiet is specified.
/// This flag enables progress reporting even if not attached to a terminal, regardless of --quiet.
/// --progress
pub fn progress() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--progress");
    })
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless --quiet is specified.
/// This flag enables progress reporting even if not attached to a terminal, regardless of --quiet.
/// --no-progress
pub fn no_progress() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-progress");
    })
}

/// When switching branches, proceed even if the index or the working tree differs from HEAD.
/// This is used to throw away local changes.
/// When checking out paths from the index, do not fail upon unmerged entries; instead, unmerged entries are ignored.
/// -f, --force
pub fn force() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--force");
    })
}

/// When checking out paths from the index, check out stage #2 (ours) or #3 (theirs) for unmerged paths.
/// --ours, --theirs
pub fn ours() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ours");
    })
}

/// When checking out paths from the index, check out stage #2 (ours) or #3 (theirs) for unmerged paths.
/// --ours, --theirs
pub fn theirs() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--theirs");
    })
}

/// Create a new branch named <new_branch> and start it at <start_point>; see git-branch(1) for details.
/// -b [new_branch]
pub fn new_branch(new_branch_arg: &str) -> FnOptionArg {
    let l_new_branch_arg = if new_branch_arg.is_empty() {
        String::from("-b")
    } else {
        format!("-b {}", new_branch_arg)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_new_branch_arg.as_str());
    })
}

/// Creates the branch <new_branch> and start it at <start_point>; if it already exists, then reset it to <start_point>.
/// This is equivalent to running 'git branch' with '-f'; see git-branch(1) for details.
/// -B [new_branch]
pub fn new_branch_force(new_branch_arg: &str) -> FnOptionArg {
    let l_new_branch_arg = if new_branch_arg.is_empty() {
        String::from("-B")
    } else {
        format!("-B {}", new_branch_arg)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_new_branch_arg.as_str());
    })
}

/// When creating a new branch, set up 'upstream' configuration.
/// See '--track' in git-branch(1) for details.
/// -t, --track
pub fn track() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--track");
    })
}

/// Do not set up 'upstream' configuration, even if the branch.autoSetupMerge configuration variable is true.
/// --no-track
pub fn no_track() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-track");
    })
}

/// Create the new branch’s reflog; see git-branch(1) for details.
/// -l
pub fn new_branch_reflog() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("-l");
    })
}

/// Rather than checking out a branch to work on it, check out a commit for inspection and discardable experiments.
/// This is the default behavior of 'git checkout <commit>' when <commit> is not a branch name.
/// See the 'DETACHED HEAD' section below for details.
/// --detach
pub fn detach() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--detach");
    })
}

/// Create a new orphan branch, named <new_branch>, started from <start_point> and switch to it.
/// The first commit made on this new branch will have no parents and it will be the root of a new history totally disconnected from all the other branches and commits.
/// --orphan <new_branch>
pub fn orphan(new_branch_arg: &str) -> FnOptionArg {
    let l_new_branch_arg = String::from(new_branch_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--orphan");
        cmd.arg(l_new_branch_arg.as_str());
    })
}

/// In sparse checkout mode, git checkout -- <paths> would update only entries matched by <paths> and sparse patterns in $GIT_DIR/info/sparse-checkout.
/// This option ignores the sparse patterns and adds back any files in <paths>.
/// --ignore-skip-worktree-bits
pub fn ignore_skip_worktree_bits() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ignore-skip-worktree-bits");
    })
}

/// When switching branches, if you have local modifications to one or more files that are different between the current branch and the branch to which you are switching, the command refuses to switch branches in order to preserve your modifications in context.
/// However, with this option, a three-way merge between the current branch, your working tree contents, and the new branch is done, and you will be on the new branch.
/// -m, --merge
pub fn merge() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--merge");
    })
}

/// The same as --merge option above, but changes the way the conflicting hunks are presented, overriding the merge.conflictStyle configuration variable.
/// Possible values are 'merge' (default) and 'diff3' (in addition to what is shown by 'merge' style, shows the original contents).
/// --conflict=<style>
pub fn conflict(style_arg: &str) -> FnOptionArg {
    let l_style_arg = format!("--conflict={}", style_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_style_arg.as_str());
    })
}

/// Interactively select hunks in the difference between the <tree-ish> (or the index, if unspecified) and the working tree.
/// The chosen hunks are then applied in reverse to the working tree (and if a <tree-ish> was specified, the index).
/// -p, --patch
pub fn patch() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--patch");
    })
}

/// git checkout refuses when the wanted ref is already checked out by another worktree.
/// This option makes it check the ref out anyway.
/// In other words, the ref can be held by more than one worktree.
/// --ignore-other-worktrees
pub fn ignore_other_worktrees() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ignore-other-worktrees");
    })
}

/// Using --recurse-submodules will update the content of all initialized submodules according to the commit recorded in the superproject.
/// If local modifications in a submodule would be overwritten the checkout will fail unless -f is used.
/// If nothing (or --no-recurse-submodules) is used, the work trees of submodules will not be updated.
/// --recurse-submodules
pub fn recurse_submodules() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--recurse-submodules");
    })
}

/// Using --recurse-submodules will update the content of all initialized submodules according to the commit recorded in the superproject.
/// If local modifications in a submodule would be overwritten the checkout will fail unless -f is used.
/// If nothing (or --no-recurse-submodules) is used, the work trees of submodules will not be updated.
/// --no-recurse-submodules
pub fn no_recurse_submodules() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-recurse-submodules");
    })
}