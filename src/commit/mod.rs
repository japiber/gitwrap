use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "commit";

/// Record changes to the repository.
/// Create a new commit containing the current contents of the index and the given log message describing the changes.
/// The new commit is a direct child of HEAD, usually the tip of the current branch, and the branch is updated to point to it.
/// [Git doc](https://git-scm.com/docs/git-commit)
pub fn commit(current_dir: Option<&str>) -> WrapCommand {
    git(GIT_COMMAND, current_dir)
}
