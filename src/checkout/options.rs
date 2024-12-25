// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const QUIET: &str = "--quiet";
pub const PROGRESS: &str = "--progress";
pub const NO_PROGRESS: &str = "--no-progress";
pub const FORCE: &str = "--force";
pub const OURS: &str = "--ours";
pub const THEIRS: &str = "--theirs";
pub const NEW_BRANCH: &str = "-b";
pub const NEW_BRANCH_FORCE: &str = "-B";
pub const TRACK: &str = "--track";
pub const NO_TRACK: &str = "--no-track";
pub const NEW_BRANCH_REFLOG: &str = "-l";
pub const DETACH: &str = "--detach";
pub const ORPHAN: &str = "--orphan";
pub const IGNORE_SKIP_WORKTREE_BITS: &str = "--ignore-skip-worktree-bits";
pub const MERGE: &str = "--merge";
pub const CONFLICT: &str = "--conflict";
pub const PATCH: &str = "--patch";
pub const IGNORE_OTHER_WORKTREES: &str = "--ignore-other-worktrees";
pub const RECURSE_SUBMODULES: &str = "--recurse-submodules";
pub const NO_RECURSE_SUBMODULES: &str = "--no-recurse-submodules";
pub const HYPHEN_HYPHEN: &str = "--";

/// Quiet, suppress feedback messages.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    option_arg::simple(QUIET)
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless --quiet is specified.
/// This flag enables progress reporting even if not attached to a terminal, regardless of --quiet.
/// --progress
pub fn progress() -> FnOptionArg {
    option_arg::simple(PROGRESS)
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless --quiet is specified.
/// This flag enables progress reporting even if not attached to a terminal, regardless of --quiet.
/// --no-progress
pub fn no_progress() -> FnOptionArg {
    option_arg::simple(NO_PROGRESS)
}

/// When switching branches, proceed even if the index or the working tree differs from HEAD.
/// This is used to throw away local changes.
/// When checking out paths from the index, do not fail upon unmerged entries; instead, unmerged entries are ignored.
/// -f, --force
pub fn force() -> FnOptionArg {
    option_arg::simple(FORCE)
}

/// When checking out paths from the index, check out stage #2 (ours) or #3 (theirs) for unmerged paths.
/// --ours, --theirs
pub fn ours() -> FnOptionArg {
    option_arg::simple(OURS)
}

/// When checking out paths from the index, check out stage #2 (ours) or #3 (theirs) for unmerged paths.
/// --ours, --theirs
pub fn theirs() -> FnOptionArg {
    option_arg::simple(THEIRS)
}

/// Create a new branch named <new_branch> and start it at <start_point>; see git-branch(1) for details.
/// -b [new_branch]
pub fn new_branch(new_branch_arg: &str) -> FnOptionArg {
    option_arg::with_optional_parameter(NEW_BRANCH, new_branch_arg)
}

/// Creates the branch <new_branch> and start it at <start_point>; if it already exists, then reset it to <start_point>.
/// This is equivalent to running 'git branch' with '-f'; see git-branch(1) for details.
/// -B [new_branch]
pub fn new_branch_force(new_branch_arg: &str) -> FnOptionArg {
    option_arg::with_optional_parameter(NEW_BRANCH_FORCE, new_branch_arg)
}

/// When creating a new branch, set up 'upstream' configuration.
/// See '--track' in git-branch(1) for details.
/// -t, --track
pub fn track() -> FnOptionArg {
    option_arg::simple(TRACK)
}

/// Do not set up 'upstream' configuration, even if the branch.autoSetupMerge configuration variable is true.
/// --no-track
pub fn no_track() -> FnOptionArg {
    option_arg::simple(NO_TRACK)
}

/// Create the new branch’s reflog; see git-branch(1) for details.
/// -l
pub fn new_branch_reflog() -> FnOptionArg {
    option_arg::simple(NEW_BRANCH_REFLOG)
}

/// Rather than checking out a branch to work on it, check out a commit for inspection and discardable experiments.
/// This is the default behavior of 'git checkout <commit>' when <commit> is not a branch name.
/// See the 'DETACHED HEAD' section below for details.
/// --detach
pub fn detach() -> FnOptionArg {
    option_arg::simple(DETACH)
}

/// Create a new orphan branch, named <new_branch>, started from <start_point> and switch to it.
/// The first commit made on this new branch will have no parents and it will be the root of a new history totally disconnected from all the other branches and commits.
/// --orphan <new_branch>
pub fn orphan(new_branch_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(ORPHAN, new_branch_arg)
}

/// In sparse checkout mode, git checkout -- <paths> would update only entries matched by <paths> and sparse patterns in $GIT_DIR/info/sparse-checkout.
/// This option ignores the sparse patterns and adds back any files in <paths>.
/// --ignore-skip-worktree-bits
pub fn ignore_skip_worktree_bits() -> FnOptionArg {
    option_arg::simple(IGNORE_SKIP_WORKTREE_BITS)
}

/// When switching branches, if you have local modifications to one or more files that are different between the current branch and the branch to which you are switching,
/// the command refuses to switch branches in order to preserve your modifications in context.
/// However, with this option, a three-way merge between the current branch, your working tree contents, and the new branch is done, and you will be on the new branch.
/// -m, --merge
pub fn merge() -> FnOptionArg {
    option_arg::simple(MERGE)
}

/// The same as --merge option above, but changes the way the conflicting hunks are presented, overriding the merge.conflictStyle configuration variable.
/// Possible values are 'merge' (default) and 'diff3' (in addition to what is shown by 'merge' style, shows the original contents).
/// --conflict=<style>
pub fn conflict(style_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(CONFLICT, style_arg)
}

/// Interactively select hunks in the difference between the <tree-ish> (or the index, if unspecified) and the working tree.
/// The chosen hunks are then applied in reverse to the working tree (and if a <tree-ish> was specified, the index).
/// -p, --patch
pub fn patch() -> FnOptionArg {
    option_arg::simple(PATCH)
}

/// git checkout refuses when the wanted ref is already checked out by another worktree.
/// This option makes it check the ref out anyway.
/// In other words, the ref can be held by more than one worktree.
/// --ignore-other-worktrees
pub fn ignore_other_worktrees() -> FnOptionArg {
    option_arg::simple(IGNORE_OTHER_WORKTREES)
}

/// Using --recurse-submodules will update the content of all initialized submodules according to the commit recorded in the superproject.
/// If local modifications in a submodule would be overwritten the checkout will fail unless -f is used.
/// If nothing (or --no-recurse-submodules) is used, the work trees of submodules will not be updated.
/// --recurse-submodules
pub fn recurse_submodules() -> FnOptionArg {
    option_arg::simple(RECURSE_SUBMODULES)
}

/// Using --recurse-submodules will update the content of all initialized submodules according to the commit recorded in the superproject.
/// If local modifications in a submodule would be overwritten the checkout will fail unless -f is used.
/// If nothing (or --no-recurse-submodules) is used, the work trees of submodules will not be updated.
/// --no-recurse-submodules
pub fn no_recurse_submodules() -> FnOptionArg {
    option_arg::simple(NO_RECURSE_SUBMODULES)
}

/// Do not interpret any more arguments as options.
/// --
pub fn hyphen_hyphen() -> FnOptionArg {
    option_arg::simple(HYPHEN_HYPHEN)
}

/// Limits the paths affected by the operation.
/// <pathspec>
pub fn pathspec(pathspec: &str) -> FnOptionArg {
    option_arg::value_parameter(pathspec)
}

/// Branch to checkout; if it refers to a branch (i.e., a name that, when prepended with "refs/heads/", is a valid ref), then that branch is checked out.
/// Otherwise, if it refers to a valid commit, your HEAD becomes "detached" and you are no longer on any branch (see below for details).
/// <branch>
pub fn branch(branch: &str) -> FnOptionArg {
    option_arg::value_parameter(branch)
}
