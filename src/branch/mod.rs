use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// List, create, or delete branches.
/// [Git doc](https://git-scm.com/docs/git-branch)
pub fn branch(current_dir: Option<&str>) -> WrapCommand {
    git("branch", current_dir)
}
