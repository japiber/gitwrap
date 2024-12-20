use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "status";

/// Show the working tree status.
/// Displays paths that have differences between the index file and the current HEAD commit, paths that have differences between the working tree and the index file, and paths in the working tree that are not tracked by Git
/// [Git doc](https://git-scm.com/docs/git-status)
pub fn status(current_dir: Option<&str>) -> WrapCommand {
    git(GIT_COMMAND, current_dir)
}
