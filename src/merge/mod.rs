use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "merge";

/// Join two or more development histories together.
/// Incorporates changes from the named commits (since the time their histories diverged from the current branch) into the current branch.
/// This command is used by git pull to incorporate changes from another repository and can be used by hand to merge changes from one branch into another.
/// [Git doc](https://git-scm.com/docs/git-merge)
pub fn merge() -> WrapCommand {
    git(GIT_COMMAND)
}
