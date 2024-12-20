use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "branch";

/// List, create, or delete branches.
/// [Git doc](https://git-scm.com/docs/git-branch)
pub fn branch(current_dir: Option<&str>) -> WrapCommand {
    git(GIT_COMMAND, current_dir)
}
