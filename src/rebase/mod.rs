use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "rebase";

/// Reapply commits on top of another base tip
/// [Git doc](https://git-scm.com/docs/git-rebase)
pub fn rebase() -> WrapCommand {
    git(GIT_COMMAND)
}
