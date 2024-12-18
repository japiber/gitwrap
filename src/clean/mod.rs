use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "clean";

/// Remove untracked files from the working tree.
/// Cleans the working tree by recursively removing files that are not under version control, starting from the current directory.
/// [Git doc](https://git-scm.com/docs/git-clean)
pub fn clean(current_dir: Option<&str>) -> WrapCommand {
    git(GIT_COMMAND, current_dir)
}
