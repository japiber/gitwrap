use crate::wrap_command::{WrapCommand, FnOptionArg};
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "pull";

/// Incorporates changes from a remote repository into the current branch.
/// If the current branch is behind the remote, then by default it will fast-forward the current branch to match the remote.
/// If the current branch and the remote have diverged, the user needs to specify how to reconcile the divergent branches with --rebase or --no-rebase
/// [Git doc](https://git-scm.com/docs/git-pull)
pub fn pull<I>(current_dir: Option<&str>, options: I) -> WrapCommand
where
    I: IntoIterator<Item = FnOptionArg>
{
    let mut gc = git(GIT_COMMAND, current_dir);
    for opt in options {
        gc.option(opt);
    }
    gc
}
